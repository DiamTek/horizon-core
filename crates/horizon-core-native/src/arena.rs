//! Native memory arena wrappers powered by `bumpalo`.

use bumpalo::Bump;
use std::ffi::c_void;

/// Opaque wrapper around a Bumpalo arena.
pub struct HorizonArena {
    bump: Bump,
}

#[no_mangle]
pub extern "C" fn hz_arena_create() -> *mut HorizonArena {
    Box::into_raw(Box::new(HorizonArena {
        bump: Bump::new(),
    }))
}

#[no_mangle]
pub extern "C" fn hz_arena_alloc(arena: *mut HorizonArena, size: usize, align: usize) -> *mut c_void {
    if arena.is_null() {
        return std::ptr::null_mut();
    }
    let arena = unsafe { &mut *arena };
    match std::alloc::Layout::from_size_align(size, align) {
        Ok(layout) => arena.bump.alloc_layout(layout).as_ptr() as *mut c_void,
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn hz_arena_reset(arena: *mut HorizonArena) {
    if !arena.is_null() {
        let arena = unsafe { &mut *arena };
        arena.bump.reset();
    }
}

#[no_mangle]
pub extern "C" fn hz_arena_destroy(arena: *mut HorizonArena) {
    if !arena.is_null() {
        unsafe {
            drop(Box::from_raw(arena));
        }
    }
}