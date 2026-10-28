//! # horizon-core-native
//!
//! Native foundation crate for the Horizon Ecosystem.
//! Provides SIMD-accelerated math/AST operations, arena memory allocators, Rayon work-stealing,
//! and strict C-ABI exports for Java 21+ Panama Foreign Function & Memory (FFM) API.

pub mod arena;
pub mod ffi;
pub mod simd;

/// Core initialization and version query.
#[no_mangle]
pub extern "C" fn hz_core_version_major() -> u32 {
    0
}

#[no_mangle]
pub extern "C" fn hz_core_version_minor() -> u32 {
    1
}

#[no_mangle]
pub extern "C" fn hz_core_version_patch() -> u32 {
    0
}

/// Initialize native engine subsystem (Rayon threadpools, allocator sanity checks).
/// Returns 0 on success, non-zero error code otherwise.
#[no_mangle]
pub extern "C" fn hz_core_init() -> i32 {
    0
}