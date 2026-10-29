//! Lock-Free Single-Producer Single-Consumer (SPSC) Ring Buffer
//! Designed for zero-copy off-heap chunk task transmission between Java (Panama FFM) and Rust.

use bytemuck::{Pod, Zeroable};
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

pub const RING_BUFFER_MAGIC: u32 = 0x485A_4351; // "HZCQ" (Horizon Core Queue)
pub const RING_CAPACITY: usize = 4096; // Must be power of 2
pub const RING_MASK: u64 = (RING_CAPACITY as u64) - 1;

/// Cache-line aligned SPSC Ring Buffer Header.
/// Enforces 128-byte alignment to prevent false sharing across modern x86_64 and ARM architectures.
#[repr(C, align(128))]
#[derive(Debug)]
pub struct SpscHeader {
    // Cache Line 0: Identification & Heartbeat Watchdog (Read-mostly)
    pub magic: u32,
    pub version: u32,
    pub capacity: u32,
    pub heartbeat_epoch: AtomicU32,
    pub crash_recovery_flag: AtomicU32,
    pub _pad0: [u8; 108],

    // Cache Line 1: Producer State (Written solely by JVM / Producer)
    pub producer_seq: AtomicU64,
    pub _pad1: [u8; 120],

    // Cache Line 2: Consumer State (Written solely by Rust / Consumer)
    pub consumer_seq: AtomicU64,
    pub _pad2: [u8; 120],
}

/// Fixed-size chunk task payload transmitted across the SPSC queue.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FfiChunkTaskPayload {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub dimension_id: i16,
    pub lod_tier: u8,
    pub flags: u8,
    pub data_ptr: u64,       // Physical off-heap address of raw block/palette data
    pub data_length: u32,
    pub sequence_tag: u32,
}

unsafe impl Zeroable for FfiChunkTaskPayload {}
unsafe impl Pod for FfiChunkTaskPayload {}

/// Complete SPSC Ring Buffer in contiguous memory.
#[repr(C, align(128))]
pub struct SpscRingBuffer {
    pub header: SpscHeader,
    pub entries: [FfiChunkTaskPayload; RING_CAPACITY],
}

impl SpscRingBuffer {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            header: SpscHeader {
                magic: RING_BUFFER_MAGIC,
                version: 1,
                capacity: RING_CAPACITY as u32,
                heartbeat_epoch: AtomicU32::new(0),
                crash_recovery_flag: AtomicU32::new(0),
                _pad0: [0; 108],
                producer_seq: AtomicU64::new(0),
                _pad1: [0; 120],
                consumer_seq: AtomicU64::new(0),
                _pad2: [0; 120],
            },
            entries: [FfiChunkTaskPayload::default(); RING_CAPACITY],
        })
    }

    /// Safe initialization inside shared off-heap memory
    pub fn init_in_place(raw_ptr: *mut u8, total_bytes: usize) -> Option<&'static mut Self> {
        if total_bytes < core::mem::size_of::<Self>() || (raw_ptr as usize % 128) != 0 {
            return None;
        }
        let ring = unsafe { &mut *(raw_ptr as *mut Self) };
        ring.header.magic = RING_BUFFER_MAGIC;
        ring.header.version = 1;
        ring.header.capacity = RING_CAPACITY as u32;
        ring.header.heartbeat_epoch.store(0, Ordering::Release);
        ring.header.crash_recovery_flag.store(0, Ordering::Release);
        ring.header.producer_seq.store(0, Ordering::Release);
        ring.header.consumer_seq.store(0, Ordering::Release);
        Some(ring)
    }

    /// Enqueue a chunk task payload into the ring buffer.
    /// Returns true if successful, false if full.
    pub fn try_push(&mut self, entry: FfiChunkTaskPayload) -> bool {
        let head = self.header.producer_seq.load(Ordering::Relaxed);
        let tail = self.header.consumer_seq.load(Ordering::Acquire);

        if head.wrapping_sub(tail) >= (RING_CAPACITY as u64) {
            return false; // Queue is full
        }

        let idx = (head & RING_MASK) as usize;
        self.entries[idx] = entry;

        self.header.producer_seq.store(head.wrapping_add(1), Ordering::Release);
        true
    }

    /// Dequeue a chunk task from the ring buffer.
    /// Returns Some(entry) if available, None if empty.
    pub fn try_pop(&mut self) -> Option<FfiChunkTaskPayload> {
        let tail = self.header.consumer_seq.load(Ordering::Relaxed);
        let head = self.header.producer_seq.load(Ordering::Acquire);

        if tail == head {
            return None; // Queue is empty
        }

        let idx = (tail & RING_MASK) as usize;
        let entry = self.entries[idx];

        self.header.consumer_seq.store(tail.wrapping_add(1), Ordering::Release);
        Some(entry)
    }

    /// Zero-copy, zero-allocation pop batch for high-throughput Rayon worker dispatch
    #[inline(always)]
    pub fn pop_batch(&self, destination: &mut [FfiChunkTaskPayload], max_count: usize) -> usize {
        let consumer = self.header.consumer_seq.load(Ordering::Relaxed);
        let producer = self.header.producer_seq.load(Ordering::Acquire);

        let available = producer.saturating_sub(consumer);
        if available == 0 {
            self.header.heartbeat_epoch.fetch_add(1, Ordering::Relaxed);
            return 0;
        }

        let to_read = available.min(max_count as u64).min(destination.len() as u64) as usize;

        for i in 0..to_read {
            let idx = ((consumer + i as u64) & RING_MASK) as usize;
            destination[i] = unsafe { core::ptr::read_volatile(&self.entries[idx]) };
        }

        self.header.consumer_seq.store(consumer + to_read as u64, Ordering::Release);
        to_read
    }

    /// Autonomous crash detection and self-reset watchdog
    pub fn verify_and_recover(&self) -> bool {
        let c1 = self.header.consumer_seq.load(Ordering::Acquire);
        let p = self.header.producer_seq.load(Ordering::Acquire);
        let c2 = self.header.consumer_seq.load(Ordering::Acquire);

        if p < c1 || (p - c2) > RING_CAPACITY as u64 {
            self.header.crash_recovery_flag.store(1, Ordering::Release);
            self.header.consumer_seq.store(p, Ordering::Release);
            self.header.crash_recovery_flag.store(0, Ordering::Release);
            return false;
        }
        true
    }

    /// Touch heartbeat watchdog to verify process liveness.
    pub fn tick_heartbeat(&self) {
        self.header.heartbeat_epoch.fetch_add(1, Ordering::Relaxed);
    }
}

// C-ABI Exports for Java Panama FFM
#[no_mangle]
pub extern "C" fn hz_spsc_create() -> *mut SpscRingBuffer {
    Box::into_raw(SpscRingBuffer::new())
}

#[no_mangle]
pub extern "C" fn hz_spsc_destroy(ring: *mut SpscRingBuffer) {
    if !ring.is_null() {
        unsafe {
            drop(Box::from_raw(ring));
        }
    }
}

#[no_mangle]
pub extern "C" fn hz_spsc_push_chunk_task(
    ring: *mut SpscRingBuffer,
    chunk_x: i32,
    chunk_z: i32,
    dimension_id: i16,
    lod_tier: u8,
    flags: u8,
    data_ptr: u64,
    data_length: u32,
    sequence_tag: u32,
) -> i32 {
    if ring.is_null() {
        return -1;
    }
    let ring = unsafe { &mut *ring };
    let entry = FfiChunkTaskPayload {
        chunk_x,
        chunk_z,
        dimension_id,
        lod_tier,
        flags,
        data_ptr,
        data_length,
        sequence_tag,
    };
    if ring.try_push(entry) {
        0
    } else {
        1 // Full
    }
}

#[no_mangle]
pub extern "C" fn hz_spsc_pop_chunk_task(
    ring: *mut SpscRingBuffer,
    out_task: *mut FfiChunkTaskPayload,
) -> i32 {
    if ring.is_null() || out_task.is_null() {
        return -1;
    }
    let ring = unsafe { &mut *ring };
    match ring.try_pop() {
        Some(entry) => {
            unsafe { *out_task = entry };
            0
        }
        None => 1, // Empty
    }
}

#[no_mangle]
pub extern "C" fn hz_spsc_pop_batch(
    ring: *mut SpscRingBuffer,
    destination: *mut FfiChunkTaskPayload,
    max_count: usize,
) -> usize {
    if ring.is_null() || destination.is_null() || max_count == 0 {
        return 0;
    }
    let ring = unsafe { &*ring };
    let slice = unsafe { std::slice::from_raw_parts_mut(destination, max_count) };
    ring.pop_batch(slice, max_count)
}

#[no_mangle]
pub extern "C" fn hz_spsc_verify_and_recover(ring: *mut SpscRingBuffer) -> i32 {
    if ring.is_null() {
        return -1;
    }
    let ring = unsafe { &*ring };
    if ring.verify_and_recover() {
        0
    } else {
        1 // Recovered from desynchronization
    }
}

#[no_mangle]
pub extern "C" fn hz_spsc_init_in_place(raw_ptr: *mut u8, total_bytes: usize) -> *mut SpscRingBuffer {
    if raw_ptr.is_null() {
        return std::ptr::null_mut();
    }
    match SpscRingBuffer::init_in_place(raw_ptr, total_bytes) {
        Some(ring) => ring as *mut SpscRingBuffer,
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn hz_spsc_ring_size() -> usize {
    core::mem::size_of::<SpscRingBuffer>()
}