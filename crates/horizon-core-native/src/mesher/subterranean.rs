//! Subterranean Dual-Elevation and Cave Occlusion Culler.

#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct SubterraneanSpanHeader {
    pub min_x: i32,
    pub min_z: i32,
    pub surface_roof_y: i16,          // Highest solid surface elevation (e.g. y = 112)
    pub cave_ceiling_max_y: i16,      // Lowest subterranean ceiling elevation (e.g. y = -8)
    pub cave_floor_min_y: i16,        // Lowest cavern floor elevation (e.g. y = -64)
    pub surface_aperture_mask: u16,   // Bitmask of vertical shafts/ravines per 4x4 area
    pub cave_meshlet_count: u32,
    pub cave_meshlet_offset: u32,
}

impl SubterraneanSpanHeader {
    /// Evaluates whether all subterranean geometry can be culled with zero cost.
    pub fn should_cull_subterranean(&self, camera_y: f32) -> bool {
        camera_y > (self.surface_roof_y as f32 + 2.0) && self.surface_aperture_mask == 0
    }
}

// C-ABI Export
#[no_mangle]
pub extern "C" fn hz_subterranean_cull(header: *const SubterraneanSpanHeader, camera_y: f32) -> i32 {
    if header.is_null() {
        return 0;
    }
    let header = unsafe { &*header };
    if header.should_cull_subterranean(camera_y) { 1 } else { 0 }
}