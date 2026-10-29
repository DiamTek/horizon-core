//! Underwater caustic projection, Snell dispersion, and wave octave uniforms.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CausticProjectionUniforms {
    pub sunlight_dir_world: [f32; 4],
    pub water_plane_elevation: f32,
    pub chromatic_dispersion_strength: f32,
    pub caustic_focus_intensity: f32,
    pub singularity_regularizer_eps: f32,
    pub water_absorption_rgb: [f32; 4],
    pub wave_octave_frequencies: [f32; 4],
    pub wave_octave_amplitudes: [f32; 4],
    pub wave_octave_speeds: [f32; 4],
}

impl CausticProjectionUniforms {
    pub fn default_ocean(sun_dir: [f32; 3], water_level: f32) -> Self {
        Self {
            sunlight_dir_world: [sun_dir[0], sun_dir[1], sun_dir[2], 1.0],
            water_plane_elevation: water_level,
            chromatic_dispersion_strength: 1.0,
            caustic_focus_intensity: 2.5,
            singularity_regularizer_eps: 0.08,
            water_absorption_rgb: [0.06, 0.02, 0.005, 0.0],
            wave_octave_frequencies: [0.25, 0.50, 1.00, 2.00],
            wave_octave_amplitudes: [0.35, 0.15, 0.08, 0.03],
            wave_octave_speeds: [1.2, 1.8, 2.5, 3.2],
        }
    }
}