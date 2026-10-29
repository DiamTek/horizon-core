//! Virtual Geometry Meshlets and Normal Cone Culling Descriptors.

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy, Default)]
pub struct MeshletDescriptor {
    pub center_radius: [f32; 4],     // World-space bounding sphere (X, Y, Z, Radius)
    pub cone_axis_cutoff: [f32; 4],   // Normal cone axis XYZ + cone cutoff W
    pub vertex_offset: u32,           // Base index into global vertex SSBO
    pub primitive_offset: u32,        // Base byte offset into local micro-index SSBO
    pub vertex_count: u8,             // <= 64
    pub primitive_count: u8,          // <= 126
    pub lod_level: u8,                // Tier 1 to Tier 3
    pub flags: u8,                    // Bit 0: Has Water, Bit 1: Foliage Wind
}

impl MeshletDescriptor {
    pub fn is_cone_backfacing(&self, cam_pos: [f32; 3]) -> bool {
        // Spec: n_cone dot ((center - p_cam) / ||center - p_cam||) >= cutoff
        let dir = [
            self.center_radius[0] - cam_pos[0],
            self.center_radius[1] - cam_pos[1],
            self.center_radius[2] - cam_pos[2],
        ];
        let len = (dir[0] * dir[0] + dir[1] * dir[1] + dir[2] * dir[2]).sqrt();
        if len < 0.0001 {
            return false;
        }
        let norm_dir = [dir[0] / len, dir[1] / len, dir[2] / len];
        let dot = self.cone_axis_cutoff[0] * norm_dir[0]
            + self.cone_axis_cutoff[1] * norm_dir[1]
            + self.cone_axis_cutoff[2] * norm_dir[2];
        dot >= self.cone_axis_cutoff[3]
    }
}

// C-ABI Export
#[no_mangle]
pub extern "C" fn hz_meshlet_cone_cull(desc: *const MeshletDescriptor, cam_x: f32, cam_y: f32, cam_z: f32) -> i32 {
    if desc.is_null() {
        return 0;
    }
    let desc = unsafe { &*desc };
    if desc.is_cone_backfacing([cam_x, cam_y, cam_z]) { 1 } else { 0 }
}