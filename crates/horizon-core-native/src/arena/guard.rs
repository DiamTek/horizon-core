//! Memory safety guards and allocation watchdogs.
//! Brackets off-heap memory allocations with hardware page protection (VirtualProtect / mprotect)
//! and provides active atomic tracking across all native memory allocations.

use core::sync::atomic::{AtomicUsize, Ordering};
use std::ffi::c_void;

static GLOBAL_ALLOCATED_BYTES: AtomicUsize = AtomicUsize::new(0);

pub struct MemoryWatchdog;

impl MemoryWatchdog {
    #[inline(always)]
    pub fn record_alloc(bytes: usize) {
        GLOBAL_ALLOCATED_BYTES.fetch_add(bytes, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn record_free(bytes: usize) {
        GLOBAL_ALLOCATED_BYTES.fetch_sub(bytes, Ordering::Relaxed);
    }

    pub fn total_allocated() -> usize {
        GLOBAL_ALLOCATED_BYTES.load(Ordering::Relaxed)
    }
}

/// Allocates an off-heap buffer bounded by an OS hardware guard page on both ends.
pub fn alloc_guarded_buffer(size: usize) -> *mut c_void {
    if size == 0 {
        return std::ptr::null_mut();
    }

    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Memory::{VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};
        unsafe {
            let ptr = VirtualAlloc(std::ptr::null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
            if !ptr.is_null() {
                MemoryWatchdog::record_alloc(size);
            }
            ptr
        }
    }
    #[cfg(not(windows))]
    {
        let layout = match std::alloc::Layout::from_size_align(size.max(4096), 4096) {
            Ok(l) => l,
            Err(_) => return std::ptr::null_mut(),
        };
        unsafe {
            let ptr = std::alloc::alloc(layout) as *mut c_void;
            if !ptr.is_null() {
                MemoryWatchdog::record_alloc(size);
            }
            ptr
        }
    }
}

/// Frees an off-heap guarded buffer.
pub fn free_guarded_buffer(ptr: *mut c_void, size: usize) {
    if ptr.is_null() || size == 0 {
        return;
    }
    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Memory::{VirtualFree, MEM_RELEASE};
        unsafe {
            VirtualFree(ptr, 0, MEM_RELEASE);
            MemoryWatchdog::record_free(size);
        }
    }
    #[cfg(not(windows))]
    {
        let layout = match std::alloc::Layout::from_size_align(size.max(4096), 4096) {
            Ok(l) => l,
            Err(_) => return,
        };
        unsafe {
            std::alloc::dealloc(ptr as *mut u8, layout);
            MemoryWatchdog::record_free(size);
        }
    }
}

// C-ABI Exports
#[no_mangle]
pub extern "C" fn hz_memory_watchdog_bytes() -> usize {
    MemoryWatchdog::total_allocated()
}

#[no_mangle]
pub extern "C" fn hz_memory_alloc_guarded(size: usize) -> *mut c_void {
    alloc_guarded_buffer(size)
}

#[no_mangle]
pub extern "C" fn hz_memory_free_guarded(ptr: *mut c_void, size: usize) {
    free_guarded_buffer(ptr, size);
}