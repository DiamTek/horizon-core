//! Subsurface scattering (SSS) material profiles and dipole diffusion parameters.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SubsurfaceProfile {
    pub absorption_coeff: [f32; 3],
    pub scattering_coeff: [f32; 3],
    pub mean_free_path: [f32; 3],
    pub max_blur_radius_pixels: f32,
    pub transmission_tint: [f32; 3],
    pub membrane_thickness_limit: f32,
    pub ior: f32,
    pub profile_id: u32,
}

impl SubsurfaceProfile {
    pub const PROFILE_FOLIAGE: u32 = 0;
    pub const PROFILE_EPIDERMIS: u32 = 1;
    pub const PROFILE_WAX: u32 = 2;
    pub const PROFILE_GLACIAL_ICE: u32 = 3;

    pub fn foliage() -> Self {
        Self {
            absorption_coeff: [0.13, 0.08, 0.32],
            scattering_coeff: [2.10, 3.40, 0.90],
            mean_free_path: [0.45, 0.58, 0.35],
            max_blur_radius_pixels: 16.0,
            transmission_tint: [0.35, 0.75, 0.20],
            membrane_thickness_limit: 0.05,
            ior: 1.33,
            profile_id: Self::PROFILE_FOLIAGE,
        }
    }

    pub fn glacial_ice() -> Self {
        Self {
            absorption_coeff: [0.04, 0.02, 0.01],
            scattering_coeff: [1.50, 1.80, 2.20],
            mean_free_path: [0.80, 0.90, 1.10],
            max_blur_radius_pixels: 24.0,
            transmission_tint: [0.20, 0.60, 0.90],
            membrane_thickness_limit: 0.50,
            ior: 1.31,
            profile_id: Self::PROFILE_GLACIAL_ICE,
        }
    }
}