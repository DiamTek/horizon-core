//! Horizon Radiance Cache (HRC) Probe Clipmap Uniforms and Octahedral Encoding.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct HrcProbeCascadeUniforms {
    pub cascade_centers: [[f32; 4]; 4],   // World-space center X, Y, Z + probe spacing W
    pub cascade_grid_dims: [[u32; 4]; 4], // DimX, DimY, DimZ, ToroidalOffsetPacked
    pub sun_irradiance: [f32; 4],         // RGB sun light + intensity
    pub sky_ambient_color: [f32; 4],      // Sky hemisphere baseline
    pub temporal_weight: f32,             // e.g., 0.85
    pub max_cascade_index: u32,
    pub padding: [u32; 2],
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PackedL1Probe {
    pub l0_rgb_fp16: [u16; 3],            // Average diffuse radiance
    pub l1_dir_oct: [i8; 2],              // Dominant incoming bounce direction
    pub l1_color_fp16: [u16; 3],          // Directional bounce chromaticity
    pub depth_mean_var: u16,              // Chebyshev probe visibility testing
}

// C-ABI Export
#[no_mangle]
pub extern "C" fn hz_hrc_probe_count_total() -> u32 {
    98368
}