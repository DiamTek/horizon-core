//! Keplerian Celestial Mechanics and Dynamic Eclipse Umbra/Penumbra Geometry.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CelestialEphemerisUniforms {
    pub sun_direction_world: [f32; 4],       // XYZ = normalized vector to sun, W = angular radius (rad)
    pub moon_direction_world: [f32; 4],      // XYZ = normalized vector to moon, W = angular radius (rad)
    pub planet_axial_tilt: f32,              // Obliquity of planetary rotation axis (e.g. 0.409 rad / 23.44 deg)
    pub precession_angle: f32,               // Current axial precession angle (rad)
    pub eclipse_obscuration: f32,            // Analytical fractional area coverage [0.0 = none, 1.0 = totality]
    pub umbra_core_distance: f32,            // Distance from camera to center of umbral ground shadow (meters)
    pub umbra_radius_ground: f32,            // Radius of total darkness footprint (meters)
    pub penumbra_radius_ground: f32,         // Outer radius of partial shadow footprint (meters)
    pub corona_intensity: f32,               // HDR multiplier for solar magnetic corona [0.0 to 100.0]
    pub atmospheric_temp_drop: f32,          // Delta temperature for procedural mist generation (Celsius)
}

impl Default for CelestialEphemerisUniforms {
    fn default() -> Self {
        Self {
            sun_direction_world: [0.0, 1.0, 0.0, 0.00465],
            moon_direction_world: [0.0, -1.0, 0.0, 0.0045],
            planet_axial_tilt: 0.409,
            precession_angle: 0.0,
            eclipse_obscuration: 0.0,
            umbra_core_distance: 10000.0,
            umbra_radius_ground: 50000.0,
            penumbra_radius_ground: 150000.0,
            corona_intensity: 0.0,
            atmospheric_temp_drop: 0.0,
        }
    }
}

/// Computes analytical solar eclipse obscuration fraction given angular separation between sun and moon.
pub fn compute_eclipse_obscuration(angular_separation: f32, sun_radius: f32, moon_radius: f32) -> f32 {
    if sun_radius <= 0.0 || moon_radius <= 0.0 || angular_separation.is_nan() || sun_radius.is_nan() || moon_radius.is_nan() {
        return 0.0;
    }

    let d = angular_separation.max(0.0);
    let r1 = sun_radius;
    let r2 = moon_radius;

    if d >= r1 + r2 {
        return 0.0; // No overlap
    }
    if d <= (r1 - r2).abs() {
        return if r2 >= r1 { 1.0 } else { (r2 * r2) / (r1 * r1) };
    }

    let r1_sq = r1 * r1;
    let r2_sq = r2 * r2;
    let d_sq = d * d;

    let denom1 = 2.0 * d * r1;
    let denom2 = 2.0 * d * r2;
    if denom1 <= 1e-8 || denom2 <= 1e-8 {
        return if r2 >= r1 { 1.0 } else { (r2 * r2) / (r1 * r1) };
    }

    let alpha = ((r1_sq + d_sq - r2_sq) / denom1).clamp(-1.0, 1.0).acos();
    let beta = ((r2_sq + d_sq - r1_sq) / denom2).clamp(-1.0, 1.0).acos();

    let area_overlap = r1_sq * alpha + r2_sq * beta - 0.5 * ((-d + r1 + r2) * (d + r1 - r2) * (d - r1 + r2) * (d + r1 + r2)).max(0.0).sqrt();
    let area_sun = std::f32::consts::PI * r1_sq;

    if area_sun <= 1e-8 {
        0.0
    } else {
        (area_overlap / area_sun).clamp(0.0, 1.0)
    }
}

// C-ABI Export
#[no_mangle]
pub extern "C" fn hz_eclipse_obscuration(sep: f32, sun_r: f32, moon_r: f32) -> f32 {
    compute_eclipse_obscuration(sep, sun_r, moon_r)
}