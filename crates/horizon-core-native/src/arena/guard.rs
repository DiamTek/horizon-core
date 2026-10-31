//! Memory safety guards and allocation watchdogs.
//! Brackets off-heap memory allocations with hardware page protection (VirtualProtect / mprotect)
//! and provides active atomic tracking across all native memory allocations.

use core::sync::atomic::{AtomicUsize, Ordering};
use std::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};

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

/// Page size constant for guard page computations.
const PAGE_SIZE: usize = 4096;

/// Rounds `size` up to the next multiple of `PAGE_SIZE`.
#[inline(always)]
fn align_to_page(size: usize) -> usize {
    (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

/// Allocates an off-heap buffer sandwiched between two PAGE_NOACCESS guard pages.
/// Layout: [GUARD_PAGE | usable_region | GUARD_PAGE]
/// Any out-of-bounds access triggers an immediate hardware trap (segfault / access violation).
pub fn alloc_guarded_buffer(size: usize) -> *mut c_void {
    if size == 0 {
        return std::ptr::null_mut();
    }

    let usable_pages = align_to_page(size);
    let total_alloc = PAGE_SIZE + usable_pages + PAGE_SIZE; // guard + data + guard

    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Memory::{
            VirtualAlloc, VirtualProtect,
            MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE, PAGE_NOACCESS,
        };
        unsafe {
            // Reserve and commit the entire region as read/write
            let base = VirtualAlloc(
                std::ptr::null_mut(),
                total_alloc,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_READWRITE,
            );
            if base.is_null() {
                return std::ptr::null_mut();
            }

            // Set leading guard page to PAGE_NOACCESS
            let mut old_protect: u32 = 0;
            VirtualProtect(base, PAGE_SIZE, PAGE_NOACCESS, &mut old_protect);

            // Set trailing guard page to PAGE_NOACCESS
            let trailing_guard = (base as *mut u8).add(PAGE_SIZE + usable_pages) as *mut c_void;
            VirtualProtect(trailing_guard, PAGE_SIZE, PAGE_NOACCESS, &mut old_protect);

            // Return pointer past the leading guard page
            let usable_ptr = (base as *mut u8).add(PAGE_SIZE) as *mut c_void;
            MemoryWatchdog::record_alloc(size);
            usable_ptr
        }
    }
    #[cfg(not(windows))]
    {
        let layout = match std::alloc::Layout::from_size_align(total_alloc, PAGE_SIZE) {
            Ok(l) => l,
            Err(_) => return std::ptr::null_mut(),
        };
        unsafe {
            let base = std::alloc::alloc(layout);
            if base.is_null() {
                return std::ptr::null_mut();
            }

            // Protect leading guard page
            libc::mprotect(base as *mut _, PAGE_SIZE, libc::PROT_NONE);

            // Protect trailing guard page
            let trailing = base.add(PAGE_SIZE + usable_pages);
            libc::mprotect(trailing as *mut _, PAGE_SIZE, libc::PROT_NONE);

            let usable_ptr = base.add(PAGE_SIZE) as *mut c_void;
            MemoryWatchdog::record_alloc(size);
            usable_ptr
        }
    }
}

/// Frees an off-heap guarded buffer previously allocated with `alloc_guarded_buffer`.
/// The caller must pass the original `size` used at allocation time.
pub fn free_guarded_buffer(ptr: *mut c_void, size: usize) {
    if ptr.is_null() || size == 0 {
        return;
    }

    let usable_pages = align_to_page(size);
    let _total_alloc = PAGE_SIZE + usable_pages + PAGE_SIZE;

    #[cfg(windows)]
    {
        use windows_sys::Win32::System::Memory::{VirtualFree, MEM_RELEASE};
        unsafe {
            // Base pointer is one page before the usable region
            let base = (ptr as *mut u8).sub(PAGE_SIZE) as *mut c_void;
            VirtualFree(base, 0, MEM_RELEASE);
            MemoryWatchdog::record_free(size);
        }
    }
    #[cfg(not(windows))]
    {
        let total_alloc = _total_alloc;
        let layout = match std::alloc::Layout::from_size_align(total_alloc, PAGE_SIZE) {
            Ok(l) => l,
            Err(_) => return,
        };
        unsafe {
            let base = (ptr as *mut u8).sub(PAGE_SIZE);

            // Restore R/W protection before dealloc so the allocator can touch the memory
            libc::mprotect(base as *mut _, PAGE_SIZE, libc::PROT_READ | libc::PROT_WRITE);
            let trailing = base.add(PAGE_SIZE + usable_pages);
            libc::mprotect(trailing as *mut _, PAGE_SIZE, libc::PROT_READ | libc::PROT_WRITE);

            std::alloc::dealloc(base, layout);
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
    catch_unwind(AssertUnwindSafe(|| alloc_guarded_buffer(size)))
        .unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn hz_memory_free_guarded(ptr: *mut c_void, size: usize) {
    let _ = catch_unwind(AssertUnwindSafe(|| free_guarded_buffer(ptr, size)));
}