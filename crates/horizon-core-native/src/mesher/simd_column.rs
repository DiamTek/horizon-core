//! SIMD-accelerated 16-column Voxel Face-Culling Mesher
//! Implements bitwise plane shifts to extract exposed voxel faces with dynamic CPUID/vector dispatch.

#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
use core::arch::aarch64::*;

#[repr(C, align(64))]
pub struct VoxelColumn16 {
    pub solid_mask: [u32; 16], // 16 columns of 32 vertical voxels each
}

pub type MesherFn = unsafe fn(columns: &VoxelColumn16, out_faces: &mut [u64; 64]) -> usize;

/// Dynamic function pointer dispatched at engine initialization via CPUID
pub static mut ACTIVE_MESHER_KERNEL: MesherFn = scalar_mesher_fallback;

/// AVX-512 Mesher Kernel: Processes 16 columns simultaneously (512-bit registers)
#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx512f,avx512bw")]
pub unsafe fn avx512_column_mesher(columns: &VoxelColumn16, out_faces: &mut [u64; 64]) -> usize {
    let voxels = _mm512_load_si512(columns.solid_mask.as_ptr() as *const _);

    // Compute +Y occlusion: Shift right by 1 bit inside each 32-bit element
    let shifted_pos_y = _mm512_srli_epi32(voxels, 1);
    // Exposed +Y faces = voxels & ~shifted_pos_y
    let exposed_pos_y = _mm512_andnot_si512(shifted_pos_y, voxels);

    // Compute -Y occlusion: Shift left by 1 bit inside each 32-bit element
    let shifted_neg_y = _mm512_slli_epi32(voxels, 1);
    // Exposed -Y faces = voxels & ~shifted_neg_y
    let exposed_neg_y = _mm512_andnot_si512(shifted_neg_y, voxels);

    // Store exposed masks directly into face buffer
    _mm512_storeu_si512(out_faces.as_mut_ptr() as *mut _, exposed_pos_y);
    _mm512_storeu_si512(out_faces.as_mut_ptr().add(8) as *mut _, exposed_neg_y);

    let mut pos_buf = [0u32; 16];
    let mut neg_buf = [0u32; 16];
    _mm512_storeu_si512(pos_buf.as_mut_ptr() as *mut _, exposed_pos_y);
    _mm512_storeu_si512(neg_buf.as_mut_ptr() as *mut _, exposed_neg_y);

    let mut total = 0;
    for i in 0..16 {
        total += pos_buf[i].count_ones() as usize + neg_buf[i].count_ones() as usize;
    }
    total
}

/// ARM NEON Mesher Kernel: Processes 4 columns per 128-bit register
#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon")]
pub unsafe fn neon_column_mesher(columns: &VoxelColumn16, out_faces: &mut [u64; 64]) -> usize {
    let mut total_faces = 0;
    for i in (0..16).step_by(4) {
        let v = vld1q_u32(columns.solid_mask.as_ptr().add(i));

        let s_pos = vshrq_n_u32(v, 1);
        let s_neg = vshlq_n_u32(v, 1);

        let exp_pos = vbicq_u32(v, s_pos);
        let exp_neg = vbicq_u32(v, s_neg);

        vst1q_u32(out_faces.as_mut_ptr().add(i) as *mut u32, exp_pos);
        vst1q_u32(out_faces.as_mut_ptr().add(i + 16) as *mut u32, exp_neg);

        total_faces += (vaddvq_u32(vcntq_u8(vreinterpretq_u8_u32(exp_pos)))) as usize;
        total_faces += (vaddvq_u32(vcntq_u8(vreinterpretq_u8_u32(exp_neg)))) as usize;
    }
    total_faces
}

/// Scalar fallback for baseline architectures
pub unsafe fn scalar_mesher_fallback(columns: &VoxelColumn16, out_faces: &mut [u64; 64]) -> usize {
    let mut total = 0;
    for i in 0..16 {
        let v = columns.solid_mask[i];
        let exp_pos = v & !(v >> 1);
        let exp_neg = v & !(v << 1);
        out_faces[i] = exp_pos as u64;
        out_faces[i + 16] = exp_neg as u64;
        total += exp_pos.count_ones() as usize + exp_neg.count_ones() as usize;
    }
    total
}

/// Dynamic Runtime CPUID Dispatcher
pub fn init_mesher_dispatch() {
    #[cfg(target_arch = "x86_64")]
    {
        if is_x86_feature_detected!("avx512f") && is_x86_feature_detected!("avx512bw") {
            unsafe {
                ACTIVE_MESHER_KERNEL = avx512_column_mesher;
            }
            return;
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        if std::arch::is_aarch64_feature_detected!("neon") {
            unsafe {
                ACTIVE_MESHER_KERNEL = neon_column_mesher;
            }
            return;
        }
    }
    unsafe {
        ACTIVE_MESHER_KERNEL = scalar_mesher_fallback;
    }
}

// C-ABI Export
#[no_mangle]
pub extern "C" fn hz_mesher_extract_faces(
    columns: *const VoxelColumn16,
    out_faces: *mut u64,
    max_faces: usize,
) -> usize {
    if columns.is_null() || out_faces.is_null() || max_faces < 64 {
        return 0;
    }
    let columns = unsafe { &*columns };
    let mut buffer = [0u64; 64];
    let written = unsafe { (ACTIVE_MESHER_KERNEL)(columns, &mut buffer) };
    let copy_count = written.min(max_faces).min(64);
    unsafe {
        std::ptr::copy_nonoverlapping(buffer.as_ptr(), out_faces, copy_count);
    }
    written
}