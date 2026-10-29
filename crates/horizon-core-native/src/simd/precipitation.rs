//! Volumetric precipitation, splash simulation, and snow accumulation uniforms.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PrecipitationUniforms {
    pub camera_pos: [f32; 3],
    pub particle_count: u32,
    pub wind_vector: [f32; 3],
    pub precipitation_type: u32,
    pub domain_extent_radius: f32,
    pub terminal_velocity: f32,
    pub delta_time: f32,
    pub accumulation_rate: f32,
    pub occlusion_min_coords: [f32; 2],
    pub occlusion_texel_size: f32,
    pub shutter_time: f32,
}

impl PrecipitationUniforms {
    pub const TYPE_NONE: u32 = 0;
    pub const TYPE_RAIN: u32 = 1;
    pub const TYPE_SNOW: u32 = 2;
    pub const TYPE_HAIL: u32 = 3;

    pub fn new(camera_pos: [f32; 3], precip_type: u32) -> Self {
        let (term_vel, accum_rate) = match precip_type {
            Self::TYPE_RAIN => (9.0, 0.0),
            Self::TYPE_SNOW => (1.2, 0.005),
            Self::TYPE_HAIL => (18.0, 0.001),
            _ => (0.0, 0.0),
        };

        Self {
            camera_pos,
            particle_count: 524_288,
            wind_vector: [0.0, -term_vel, 0.0],
            precipitation_type: precip_type,
            domain_extent_radius: 64.0,
            terminal_velocity: term_vel,
            delta_time: 0.016,
            accumulation_rate: accum_rate,
            occlusion_min_coords: [camera_pos[0] - 64.0, camera_pos[2] - 64.0],
            occlusion_texel_size: 0.0625,
            shutter_time: 0.016,
        }
    }

    #[inline(always)]
    pub fn is_active(&self) -> bool {
        self.precipitation_type != Self::TYPE_NONE
    }
}