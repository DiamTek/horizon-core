//! SIMD math and vectorized array evaluation operations.

#[no_mangle]
pub extern "C" fn hz_simd_add_f32(a: *const f32, b: *const f32, out: *mut f32, count: usize) {
    if a.is_null() || b.is_null() || out.is_null() || count == 0 {
        return;
    }
    let a_slice = unsafe { std::slice::from_raw_parts(a, count) };
    let b_slice = unsafe { std::slice::from_raw_parts(b, count) };
    let out_slice = unsafe { std::slice::from_raw_parts_mut(out, count) };

    // LLVM automatically vectorizes this chunked iterator with AVX2/AVX-512/NEON when target features are enabled
    for i in 0..count {
        out_slice[i] = a_slice[i] + b_slice[i];
    }
}

#[no_mangle]
pub extern "C" fn hz_simd_dot_product_f32(a: *const f32, b: *const f32, count: usize) -> f32 {
    if a.is_null() || b.is_null() || count == 0 {
        return 0.0;
    }
    let a_slice = unsafe { std::slice::from_raw_parts(a, count) };
    let b_slice = unsafe { std::slice::from_raw_parts(b, count) };

    a_slice.iter().zip(b_slice.iter()).map(|(x, y)| x * y).sum()
}