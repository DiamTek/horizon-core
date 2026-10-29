//! Aurora and Planetary Magnetosphere Uniforms and Atmospheric Physics.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct AuroraUniforms {
    pub dipole_moment: [f32; 3],        // Magnetic dipole orientation & magnitude
    pub solar_wind_velocity: f32,       // km/s (typically 300 - 800 km/s)
    pub solar_wind_density: f32,        // protons/cm^3 (affects curtain intensity)
    pub kp_index: f32,                  // Geomagnetic activity index (0.0 - 9.0)
    pub ionosphere_bottom_km: f32,      // 80.0 km
    pub ionosphere_top_km: f32,         // 350.0 km
    pub curtain_curl_frequency: f32,    // Spatial frequency of fold wrinkles
    pub curtain_curl_speed: f32,        // Advection velocity along auroral oval
    pub green_oxygen_intensity: f32,    // 557.7 nm weight
    pub red_oxygen_intensity: f32,      // 630.0 nm weight
    pub blue_nitrogen_intensity: f32,   // 391.4/427.8 nm weight
    pub earth_radius_km: f32,           // 6371.0 km
    pub sim_time: f32,                  // Continuous drift time in seconds
    pub _padding: [f32; 3],
}

impl Default for AuroraUniforms {
    fn default() -> Self {
        Self {
            dipole_moment: [0.0, 1.0, 0.0],
            solar_wind_velocity: 450.0,
            solar_wind_density: 5.0,
            kp_index: 3.0,
            ionosphere_bottom_km: 80.0,
            ionosphere_top_km: 350.0,
            curtain_curl_frequency: 0.02,
            curtain_curl_speed: 1.5,
            green_oxygen_intensity: 1.0,
            red_oxygen_intensity: 0.6,
            blue_nitrogen_intensity: 0.3,
            earth_radius_km: 6371.0,
            sim_time: 0.0,
            _padding: [0.0; 3],
        }
    }
}