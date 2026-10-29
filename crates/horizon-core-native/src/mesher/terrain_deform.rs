//! Real-time dynamic micro-topography cratering, footprint deformation, and restitution state.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TerrainDeformUniforms {
    pub player_world_pos: [f32; 4],
    pub clipmap_origin_xz: [f32; 2],
    pub texel_world_size: f32,
    pub restitution_rate: f32,
    pub event_count: u32,
    pub delta_time: f32,
    pub weather_type: u32,
    pub padding: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceDeformationEvent {
    pub world_pos: [f32; 3],
    pub radius: f32,
    pub max_depth: f32,
    pub event_type: u32,
    pub material_mask: u32,
    pub compaction_strength: f32,
}

impl SurfaceDeformationEvent {
    pub const TYPE_FOOTSTEP: u32 = 0;
    pub const TYPE_GALLOP: u32 = 1;
    pub const TYPE_EXPLOSION: u32 = 2;
    pub const TYPE_VEHICLE_WHEEL: u32 = 3;

    pub fn footstep(pos: [f32; 3], depth: f32) -> Self {
        Self {
            world_pos: pos,
            radius: 0.25,
            max_depth: depth,
            event_type: Self::TYPE_FOOTSTEP,
            material_mask: 0xFFFF_FFFF,
            compaction_strength: 0.6,
        }
    }

    pub fn explosion(pos: [f32; 3], radius: f32, depth: f32) -> Self {
        Self {
            world_pos: pos,
            radius,
            max_depth: depth,
            event_type: Self::TYPE_EXPLOSION,
            material_mask: 0xFFFF_FFFF,
            compaction_strength: 1.0,
        }
    }
}