//! 2D Thermodynamic Diffusion Field and FLIR Shading Models.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalCell {
    pub temperature_kelvin: f32,   // Current surface temperature in Kelvin
    pub ambient_base_kelvin: f32,  // Local climate baseline (e.g. 263K Snow, 310K Desert)
    pub heat_capacity: f32,        // Specific heat capacity of surface material (J/kg*K)
    pub heat_source_emission: f32, // Constant thermal emitter flux Q (Watts)
}

impl Default for ThermalCell {
    fn default() -> Self {
        Self {
            temperature_kelvin: 293.15,
            ambient_base_kelvin: 293.15,
            heat_capacity: 840.0,
            heat_source_emission: 0.0,
        }
    }
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThermalSimUniforms {
    pub grid_dimension_chunks: u32,    // Width/depth of simulation region in chunks
    pub thermal_diffusivity_alpha: f32, // Conduction rate (m^2/s, e.g. 0.12)
    pub convective_cooling_h: f32,     // Atmosphere heat dissipation rate
    pub solar_flux_irradiance: f32,    // Sunlight heating flux (W/m^2)
    pub rain_cooling_flux: f32,        // Evaporative heat loss during rain/storms
    pub mirage_distortion_scale: f32,  // Optical shimmer deflection intensity
    pub flir_display_mode: u32,        // 0 = Normal PBR, 1 = FLIR Ironbow, 2 = FLIR White-Hot
}

/// Evaluates physical Gladstone-Dale air refractivity mirage index:
/// n(T) = 1 + (n_0 - 1) * (T_0 / T), where n_0 = 1.000293, T_0 = 288.15 K
pub fn compute_gladstone_dale_refractivity(temperature_kelvin: f32) -> f32 {
    let t = if temperature_kelvin.is_nan() || temperature_kelvin < 1.0 {
        288.15
    } else {
        temperature_kelvin.max(10.0)
    };
    1.0 + (0.000293) * (288.15 / t)
}

// C-ABI Export
#[no_mangle]
pub extern "C" fn hz_thermal_air_refractivity(t_kelvin: f32) -> f32 {
    compute_gladstone_dale_refractivity(t_kelvin)
}