//! Earth-scale dynamic volumetric cloud shadow and crepuscular godray projection uniforms.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CloudShadowUniforms {
    pub sun_direction: [f32; 4],
    pub cloud_altitude_range: [f32; 4],
    pub shadow_projection_origin: [f32; 4],
    pub shadow_texel_scale: f32,
    pub penumbra_scale: f32,
    pub inscatter_intensity: f32,
    pub mie_scattering_coeff: f32,
    pub raymarch_step_count: u32,
    pub temporal_reprojection_weight: f32,
    pub shadow_buffer_dimension: u32,
    pub frame_counter: u32,
}

impl CloudShadowUniforms {
    pub fn new(sun_dir_xyz: [f32; 3], solar_elevation: f32) -> Self {
        Self {
            sun_direction: [sun_dir_xyz[0], sun_dir_xyz[1], sun_dir_xyz[2], solar_elevation],
            cloud_altitude_range: [1200.0, 2400.0, 1200.0, 12.0],
            shadow_projection_origin: [0.0, 0.0, 0.0, 1.0],
            shadow_texel_scale: 5.0,
            penumbra_scale: 0.00925,
            inscatter_intensity: 1.0,
            mie_scattering_coeff: 0.004,
            raymarch_step_count: 16,
            temporal_reprojection_weight: 0.90,
            shadow_buffer_dimension: 1024,
            frame_counter: 0,
        }
    }
}