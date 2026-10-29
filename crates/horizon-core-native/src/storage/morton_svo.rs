//! Hierarchical Sparse Voxel Octree (SVO) with Pointerless Morton Encoding.

#[repr(C, align(8))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SvoNodeGPU {
    pub child_mask: u8,            // 8-bit child occupancy mask
    pub leaf_mask: u8,             // 8-bit terminal leaf flag
    pub emissive_rgb565: u16,      // Quantized colored radiance (5R:6G:5B)
    pub child_pointer_offset: u32, // Direct index into global SSBO
}

impl SvoNodeGPU {
    /// Encodes a 3D coordinate (0..1023) into a 64-bit Morton code (Z-order curve).
    pub fn morton_encode_3d(x: u32, y: u32, z: u32) -> u64 {
        #[inline(always)]
        fn split_by_3(mut a: u64) -> u64 {
            a &= 0x00000000001fffff;
            a = (a | (a << 32)) & 0x001f00000000ffff;
            a = (a | (a << 16)) & 0x001f0000ff0000ff;
            a = (a | (a << 8))  & 0x100f00f00f00f00f;
            a = (a | (a << 4))  & 0x10c30c30c30c30c3;
            a = (a | (a << 2))  & 0x1249249249249249;
            a
        }
        split_by_3(x as u64) | (split_by_3(y as u64) << 1) | (split_by_3(z as u64) << 2)
    }
}

// C-ABI Export
#[no_mangle]
pub extern "C" fn hz_svo_morton_encode(x: u32, y: u32, z: u32) -> u64 {
    SvoNodeGPU::morton_encode_3d(x, y, z)
}