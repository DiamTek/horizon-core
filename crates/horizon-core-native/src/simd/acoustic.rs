//! Geometric Acoustic Raytracer and Sabine-Eyring Reverberation Engine.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AcousticParameters {
    pub room_volume_m3: f32,
    pub t60_decay_time_seconds: f32,     // 0.1s (outdoors) to 4.5s (mega-cavern)
    pub high_frequency_decay_ratio: f32, // 0.1 to 2.0
    pub direct_occlusion_gain: f32,      // 0.0 (silent) to 1.0 (unobstructed)
    pub direct_lowpass_cutoff_hz: f32,   // 200 Hz to 20000 Hz
    pub early_reflections_gain: f32,     // Early echo reflection level
    pub late_reverb_gain: f32,           // Diffuse reverberation tail level
    pub listener_in_cave_ratio: f32,     // 0.0 (open sky) to 1.0 (subterranean)
}

impl Default for AcousticParameters {
    fn default() -> Self {
        Self {
            room_volume_m3: 1000.0,
            t60_decay_time_seconds: 0.8,
            high_frequency_decay_ratio: 0.5,
            direct_occlusion_gain: 1.0,
            direct_lowpass_cutoff_hz: 20000.0,
            early_reflections_gain: 0.3,
            late_reverb_gain: 0.4,
            listener_in_cave_ratio: 0.0,
        }
    }
}

/// Solves Eyring-Sabine reverberation decay time T60.
pub fn compute_eyring_sabine_t60(cavern_volume: f32, total_surface_area: f32, mean_absorption: f32) -> f32 {
    if cavern_volume <= 0.0
        || total_surface_area <= 0.0
        || cavern_volume.is_nan()
        || total_surface_area.is_nan()
        || mean_absorption.is_nan()
    {
        return 0.1;
    }

    let alpha = mean_absorption.clamp(0.01, 0.99);
    let denominator = -total_surface_area * (1.0 - alpha).ln() + 4.0 * 0.001 * cavern_volume;
    if denominator <= 0.0 || denominator.is_nan() {
        return 0.1;
    }
    (0.161 * cavern_volume / denominator).clamp(0.1, 4.5)
}

// C-ABI Export
#[no_mangle]
pub extern "C" fn hz_acoustic_solve_t60(volume: f32, area: f32, absorption: f32) -> f32 {
    compute_eyring_sabine_t60(volume, area, absorption)
}