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

use std::panic::{catch_unwind, AssertUnwindSafe};

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
    catch_unwind(AssertUnwindSafe(|| {
        log::info!("Initializing horizon-core-native subsystem");
        mesher::simd_column::init_mesher_dispatch();
        threading::numa_scheduler::hz_scheduler_init();
        0
    })).unwrap_or(-1)
}

// -----------------------------------------------------------------------------
// JNI Linkage Layer for Minecraft Java 17 Compatibility (net.horizon.nativecore.HorizonNativeBridge)
// -----------------------------------------------------------------------------

#[no_mangle]
pub extern "system" fn Java_net_horizon_nativecore_HorizonNativeBridge_hz_1core_1version_1major(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
) -> i32 {
    hz_core_version_major() as i32
}

#[no_mangle]
pub extern "system" fn Java_net_horizon_nativecore_HorizonNativeBridge_hz_1core_1version_1minor(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
) -> i32 {
    hz_core_version_minor() as i32
}

#[no_mangle]
pub extern "system" fn Java_net_horizon_nativecore_HorizonNativeBridge_hz_1core_1version_1patch(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
) -> i32 {
    hz_core_version_patch() as i32
}

#[no_mangle]
pub extern "system" fn Java_net_horizon_nativecore_HorizonNativeBridge_hz_1core_1init(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
) -> i32 {
    hz_core_init()
}

#[no_mangle]
pub extern "system" fn Java_net_horizon_nativecore_HorizonNativeBridge_computeDensityNative(
    _env: *mut std::ffi::c_void,
    _class: *mut std::ffi::c_void,
    world_x: i32,
    world_y: i32,
    world_z: i32,
    seed: i64,
) -> f32 {
    let fx = (world_x as f32) * 0.005;
    let fz = (world_z as f32) * 0.005;
    let continental = (fx + ((seed & 0xFFFF) as f32)).sin() * (fz + (((seed >> 16) & 0xFFFF) as f32)).cos();
    let detail = ((world_x as f32) * 0.02).sin() * ((world_z as f32) * 0.02).sin() * 0.25;
    let base_height = 64.0 + (continental * 32.0) + (detail * 16.0);
    base_height - (world_y as f32)
}