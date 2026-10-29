//! Planetary geocentric dynamics, wave interference, combustion kinetics, and non-Euclidean manifolds.

use bytemuck::{Pod, Zeroable};

/// GPU Planetary Constant Uniform Block (Geocentric Curvature and Gravitational Tides)
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PlanetaryUniformData {
    pub planet_center_and_radius: [f32; 4],
    pub omega_and_phase: [f32; 4],
    pub lunar_tide_params: [f32; 4],
    pub solar_tide_params: [f32; 4],
    pub curvature_thresholds: [f32; 4],
}

/// Polarized Atmospheric Radiative Transfer & Stokes Vector Uniforms
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct PolarizedAtmosphereUniforms {
    pub sun_direction: [f32; 4],
    pub earth_radii: [f32; 4],
    pub rayleigh_stokes: [f32; 4],
    pub ice_crystal_params: [f32; 4],
}

/// GPU Non-Euclidean Portal Transform State & Clipping Manifold
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct PortalGpuData {
    pub transform_a_to_b: [[f32; 4]; 4],
    pub clip_plane_world: [f32; 4],
    pub portal_vertices: [[f32; 4]; 4],
    pub meta_params: [u32; 4],
}

/// GPU Thin-Film Wave Interference & Microfacet Diffraction Record
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ThinFilmMaterialGpuRecord {
    pub ior_constants: [f32; 4],
    pub film_params: [f32; 4],
    pub roughness_aniso: [f32; 4],
    pub spectral_tint: [f32; 4],
}

/// 3D Navier-Stokes Thermodynamic Combustion & Wildfire Simulation Parameters
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CombustionSimulationParams {
    pub grid_origin: [f32; 4],
    pub wind_vector: [f32; 4],
    pub reaction_constants: [f32; 4],
    pub dissipation_rates: [f32; 4],
}

/// GPU Morphogenetic Flora Node Graph Element
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct FloraNodeGpu {
    pub position_radius: [f32; 4],
    pub rotation_quat: [f32; 4],
    pub phenology_state: [f32; 4],
    pub topology_ptrs: [u32; 4],
}

/// Sub-Voxel Soil Hydrology and Root Transpiration Cell
#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct FloraHydrologyCell {
    pub matric_potential: f32,
    pub moisture_content: f32,
    pub root_sink_strength: f32,
    pub soil_temperature: f32,
}