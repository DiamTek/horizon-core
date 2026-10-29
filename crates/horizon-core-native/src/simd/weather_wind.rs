//! Macro wind simulation vector field and dynamic foliage deflection uniforms.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MacroWindUniforms {
    pub global_wind_dir: [f32; 2],
    pub macro_speed: f32,
    pub turbulence_intensity: f32,
    pub simulation_time: f32,
    pub gust_wavelength: f32,
    pub micro_climate_flags: u32,
    pub padding: u32,
}

impl MacroWindUniforms {
    pub fn new(wind_dir_x: f32, wind_dir_z: f32, speed: f32, turbulence: f32) -> Self {
        let (wx, wz) = if wind_dir_x.is_nan() || wind_dir_z.is_nan() {
            (1.0, 0.0)
        } else {
            (wind_dir_x, wind_dir_z)
        };
        let len = (wx * wx + wz * wz).sqrt().max(1e-5);
        let s = if speed.is_nan() { 0.0 } else { speed.max(0.0) };
        let t = if turbulence.is_nan() { 0.0 } else { turbulence.clamp(0.0, 10.0) };
        Self {
            global_wind_dir: [wx / len, wz / len],
            macro_speed: s,
            turbulence_intensity: t,
            simulation_time: 0.0,
            gust_wavelength: 120.0,
            micro_climate_flags: 0,
            padding: 0,
        }
    }

    #[inline(always)]
    pub fn is_storm_active(&self) -> bool {
        (self.micro_climate_flags & 0x01) != 0
    }

    #[inline(always)]
    pub fn is_thunder_active(&self) -> bool {
        (self.micro_climate_flags & 0x02) != 0
    }

    #[inline(always)]
    pub fn is_blizzard_active(&self) -> bool {
        (self.micro_climate_flags & 0x04) != 0
    }
}