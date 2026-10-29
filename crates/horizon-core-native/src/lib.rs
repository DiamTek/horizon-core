//! # horizon-core-native
//!
//! Native foundation crate for the Horizon Ecosystem.
//! Provides SIMD-accelerated math/AST operations, arena memory allocators, Rayon work-stealing,
//! lock-free SPSC ring buffers, CRC32C WAL recovery, and Panama FFM C-ABI bindings.

pub mod arena;
pub mod ffi;
pub mod mesher;
pub mod simd;
pub mod storage;
pub mod threading;

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

/// Initialize native engine subsystem (Rayon threadpools, allocator sanity checks, mesher dispatch).
/// Returns 0 on success, non-zero error code otherwise.
#[no_mangle]
pub extern "C" fn hz_core_init() -> i32 {
    log::info!("Initializing horizon-core-native subsystem");
    mesher::simd_column::init_mesher_dispatch();
    threading::numa_scheduler::hz_scheduler_init();
    0
}