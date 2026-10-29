//! Smallest-Three 32-Bit Quaternion Compression and Branchless Decompression.
//!
//! Encodes a normalized unit quaternion [x, y, z, w] into a single 32-bit integer:
//! - bits 30..31 ( 2 bits): Index of the dropped largest component (0..3)
//! - bits 20..29 (10 bits): Quantized component A over [-1/sqrt(2), +1/sqrt(2)]
//! - bits 10..19 (10 bits): Quantized component B over [-1/sqrt(2), +1/sqrt(2)]
//! - bits  0.. 9 (10 bits): Quantized component C over [-1/sqrt(2), +1/sqrt(2)]
//!
//! Provides 93.75% network bandwidth reduction over 16-float (64-byte) matrices.

const INV_SQRT_2: f32 = 0.7071067811865475;
const SCALE: f32 = 1023.0 / (2.0 * INV_SQRT_2);

#[inline(always)]
fn quantize_10bit(val: f32) -> u32 {
    let safe_val = if val.is_nan() { 0.0 } else { val };
    let clamped = safe_val.clamp(-INV_SQRT_2, INV_SQRT_2);
    let normalized = (clamped + INV_SQRT_2) * SCALE;
    (normalized.round() as u32).min(1023)
}

#[inline(always)]
fn dequantize_10bit(bits: u32) -> f32 {
    (bits as f32) * (2.0 * INV_SQRT_2 / 1023.0) - INV_SQRT_2
}

/// Pack a normalized unit quaternion [x, y, z, w] into a 32-bit integer.
pub fn pack_quaternion(mut q: [f32; 4]) -> u32 {
    for val in &mut q {
        if val.is_nan() || val.is_infinite() {
            q = [0.0, 0.0, 0.0, 1.0];
            break;
        }
    }

    let norm_sq = q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3];
    if norm_sq.is_nan() || norm_sq < 1e-8 {
        // Fallback to identity quaternion
        q = [0.0, 0.0, 0.0, 1.0];
    } else {
        let inv_norm = 1.0 / norm_sq.sqrt();
        for val in &mut q {
            *val *= inv_norm;
        }
    }

    // Find component with largest absolute value
    let mut max_idx = 0;
    let mut max_abs = q[0].abs();
    for i in 1..4 {
        let abs = q[i].abs();
        if abs > max_abs {
            max_abs = abs;
            max_idx = i;
        }
    }

    // Sender-side sign normalization: guarantee max component is positive
    if q[max_idx] < 0.0 {
        for val in &mut q {
            *val = -*val;
        }
    }

    // Extract the remaining 3 components
    let (a, b, c) = match max_idx {
        0 => (q[1], q[2], q[3]),
        1 => (q[0], q[2], q[3]),
        2 => (q[0], q[1], q[3]),
        _ => (q[0], q[1], q[2]),
    };

    let qa = quantize_10bit(a);
    let qb = quantize_10bit(b);
    let qc = quantize_10bit(c);

    ((max_idx as u32) << 30) | (qa << 20) | (qb << 10) | qc
}

/// Branchless decompression of a 32-bit packed quaternion into [x, y, z, w].
pub fn unpack_quaternion(packed: u32) -> [f32; 4] {
    let max_idx = (packed >> 30) as usize;
    let a = dequantize_10bit((packed >> 20) & 0x3FF);
    let b = dequantize_10bit((packed >> 10) & 0x3FF);
    let c = dequantize_10bit(packed & 0x3FF);

    let d_sq = 1.0 - (a * a + b * b + c * c);
    let d = if d_sq > 0.0 { d_sq.sqrt() } else { 0.0 };

    match max_idx {
        0 => [d, a, b, c],
        1 => [a, d, b, c],
        2 => [a, b, d, c],
        _ => [a, b, c, d],
    }
}

// C-ABI Exports
#[no_mangle]
pub extern "C" fn hz_quat_pack(x: f32, y: f32, z: f32, w: f32) -> u32 {
    pack_quaternion([x, y, z, w])
}

#[no_mangle]
pub extern "C" fn hz_quat_unpack(packed: u32, out: *mut f32) {
    if out.is_null() {
        return;
    }
    let res = unpack_quaternion(packed);
    unsafe {
        std::ptr::copy_nonoverlapping(res.as_ptr(), out, 4);
    }
}