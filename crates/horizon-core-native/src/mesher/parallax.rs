//! Quad-clustered steep parallax occlusion mapping (POM) and contact self-shadowing uniforms.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ParallaxLODUniforms {
    pub pom_depth_scale: f32,
    pub near_distance_threshold: f32,
    pub far_distance_threshold: f32,
    pub max_steps_near: u32,
    pub min_steps_far: u32,
    pub self_shadow_step_count: u32,
    pub shadow_softness: f32,
    pub atlas_tile_size: f32,
}

impl ParallaxLODUniforms {
    pub fn new(depth_scale: f32, tile_size: f32) -> Self {
        Self {
            pom_depth_scale: depth_scale,
            near_distance_threshold: 128.0,
            far_distance_threshold: 512.0,
            max_steps_near: 16,
            min_steps_far: 4,
            self_shadow_step_count: 4,
            shadow_softness: 1.0,
            atlas_tile_size: tile_size,
        }
    }

    #[inline(always)]
    pub fn compute_steps(&self, distance: f32, n_dot_v: f32) -> u32 {
        let d = if distance.is_nan() { self.near_distance_threshold } else { distance };
        let ndv = if n_dot_v.is_nan() { 1.0 } else { n_dot_v.clamp(0.0, 1.0) };
        let denom = (self.far_distance_threshold - self.near_distance_threshold).max(1e-4);
        let t = ((d - self.near_distance_threshold) / denom).clamp(0.0, 1.0);
        let base_steps = self.max_steps_near as f32 * (1.0 - t) + self.min_steps_far as f32 * t;
        let angle_factor = 0.5 + 0.5 * (1.0 - ndv);
        (base_steps * angle_factor).round() as u32
    }
}