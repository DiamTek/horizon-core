pub mod guard;

use bumpalo::Bump;
use std::ffi::c_void;
use std::panic::{catch_unwind, AssertUnwindSafe};

/// Opaque wrapper around a Bumpalo arena.
pub struct HorizonArena {
    bump: Bump,
    allocated_bytes: usize,
}

#[no_mangle]
pub extern "C" fn hz_arena_create() -> *mut HorizonArena {
    catch_unwind(AssertUnwindSafe(|| {
        Box::into_raw(Box::new(HorizonArena {
            bump: Bump::new(),
            allocated_bytes: 0,
        }))
    })).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn hz_arena_alloc(arena: *mut HorizonArena, size: usize, align: usize) -> *mut c_void {
    catch_unwind(AssertUnwindSafe(|| {
        if arena.is_null() || size == 0 {
            return std::ptr::null_mut();
        }
        let arena = unsafe { &mut *arena };
        match std::alloc::Layout::from_size_align(size, align) {
            Ok(layout) => {
                guard::MemoryWatchdog::record_alloc(size);
                arena.allocated_bytes += size;
                arena.bump.alloc_layout(layout).as_ptr() as *mut c_void
            }
            Err(_) => std::ptr::null_mut(),
        }
    })).unwrap_or(std::ptr::null_mut())
}

#[no_mangle]
pub extern "C" fn hz_arena_reset(arena: *mut HorizonArena) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if !arena.is_null() {
            let arena = unsafe { &mut *arena };
            guard::MemoryWatchdog::record_free(arena.allocated_bytes);
            arena.allocated_bytes = 0;
            arena.bump.reset();
        }
    }));
}

#[no_mangle]
pub extern "C" fn hz_arena_destroy(arena: *mut HorizonArena) {
    let _ = catch_unwind(AssertUnwindSafe(|| {
        if !arena.is_null() {
            let arena_box = unsafe { Box::from_raw(arena) };
            guard::MemoryWatchdog::record_free(arena_box.allocated_bytes);
            drop(arena_box);
        }
    }));
}