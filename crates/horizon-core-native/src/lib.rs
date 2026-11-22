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
    let nx = ((world_x as f64) + ((seed & 0x7FFF) as f64)) * 0.0018;
    let nz = ((world_z as f64) + (((seed >> 16) & 0x7FFF) as f64)) * 0.0018;

    let cont = nx.sin() * nz.cos() + 0.5 * (nx * 2.1 + 1.2).sin() * (nz * 2.1 + 0.8).cos();
    let erosion = (nx * 3.7 - 0.5).cos() * (nz * 3.7 + 0.3).sin();
    let pv = (nx * 5.3).sin() * (nz * 5.3).sin();

    let continental_height = 63.0 + cont * 38.0;
    let mountain_peaks = pv.max(0.0) * (1.0 - erosion.abs()) * 75.0;
    let base_height = (continental_height + mountain_peaks).max(50.0).min(240.0) as f32;

    base_height - (world_y as f32)
}