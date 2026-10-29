//! Structural Physics, Voronoi Shards, and XPBD Debris Simulation Layouts.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GpuRigidDebrisShard {
    pub position: [f32; 3],          // World-space XYZ position (meters)
    pub sleep_state: u32,            // 0 = Active, 1 = Sleeping, 2 = Marked for despawn
    pub orientation_quat: [f32; 4],  // Quaternion rotation [x, y, z, w]
    pub linear_velocity: [f32; 3],   // Linear velocity vector (m/s)
    pub mass_inv: f32,               // 1.0 / Mass (0.0 = static/immovable)
    pub angular_velocity: [f32; 3],  // Angular velocity vector (rad/s)
    pub restitution: f32,            // Coefficient of restitution [0.0 to 1.0]
    pub static_friction: f32,        // Static friction coefficient mu_s
    pub dynamic_friction: f32,       // Kinetic friction coefficient mu_k
    pub bounding_radius: f32,        // Bounding collision sphere radius (m)
    pub shard_type_index: u32,       // Voronoi polyhedral shard geometry mesh index (0..15)
    pub block_uv_min: [f32; 2],      // Atlas UV coordinate bounds for original block texture
    pub block_uv_max: [f32; 2],      // Atlas UV coordinate bounds for original block texture
    pub lifetime_ticks: u32,         // Ticks elapsed since explosion injection
    pub max_lifetime: u32,           // Maximum ticks before fade-out/despawn
}

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DebrisIndirectCommand {
    pub count: u32,                  // Number of indices to draw per shard
    pub instance_count: u32,         // Compacted count of active shards
    pub first_index: u32,            // Starting offset in index buffer
    pub base_vertex: i32,            // Base vertex offset
    pub base_instance: u32,          // Starting instance index
}

// C-ABI Export
#[no_mangle]
pub extern "C" fn hz_debris_max_shards() -> u32 {
    65536
}