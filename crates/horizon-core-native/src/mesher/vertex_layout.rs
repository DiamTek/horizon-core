//! 64-Bit Packed Vertex Format Encoders and Decoders.
//!
//! Provides two zero-overhead vertex formats:
//! 1. `PackedVertex64`: Optimized coordinate layout with packed UV offsets.
//! 2. `ReconciledVertex64`: Exact master format with dual-channel lighting, 1024 vertical range,
//!    operational flags, and 18-bit dual-mode perceptual biome tint / hydrodynamic river flow.

use bytemuck::{Pod, Zeroable};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Pod, Zeroable)]
pub struct PackedVertex64(pub u64);

impl PackedVertex64 {
    #[inline(always)]
    pub fn encode(
        x: u32,
        y: u32,
        z: u32,
        normal_idx: u32,
        block_light: u32,
        sky_light: u32,
        u: u8,
        v: u8,
        material_id: u16,
    ) -> Self {
        let val = ((x as u64 & 0x3F) << 0)
            | ((y as u64 & 0x1FF) << 6)
            | ((z as u64 & 0x3F) << 15)
            | ((normal_idx as u64 & 0x07) << 21)
            | ((block_light as u64 & 0x0F) << 24)
            | ((sky_light as u64 & 0x0F) << 28)
            | ((u as u64) << 32)
            | ((v as u64) << 40)
            | ((material_id as u64) << 48);
        PackedVertex64(val)
    }

    #[inline(always)]
    pub fn x(&self) -> u32 {
        (self.0 & 0x3F) as u32
    }

    #[inline(always)]
    pub fn y(&self) -> u32 {
        ((self.0 >> 6) & 0x1FF) as u32
    }

    #[inline(always)]
    pub fn z(&self) -> u32 {
        ((self.0 >> 15) & 0x3F) as u32
    }

    #[inline(always)]
    pub fn normal_idx(&self) -> u32 {
        ((self.0 >> 21) & 0x07) as u32
    }

    #[inline(always)]
    pub fn block_light(&self) -> u32 {
        ((self.0 >> 24) & 0x0F) as u32
    }

    #[inline(always)]
    pub fn sky_light(&self) -> u32 {
        ((self.0 >> 28) & 0x0F) as u32
    }

    #[inline(always)]
    pub fn u(&self) -> u8 {
        ((self.0 >> 32) & 0xFF) as u8
    }

    #[inline(always)]
    pub fn v(&self) -> u8 {
        ((self.0 >> 40) & 0xFF) as u8
    }

    #[inline(always)]
    pub fn material_id(&self) -> u16 {
        ((self.0 >> 48) & 0xFFFF) as u16
    }
}

/// Reconciled 64-Bit Packed Vertex Specification (5 + 10 + 5 + 9 + 3 + 2 + 4 + 4 + 4 + 18 = 64 bits).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Pod, Zeroable)]
pub struct ReconciledVertex64(pub u64);

impl ReconciledVertex64 {
    pub const FLAG_WATER: u32 = 0x01;
    pub const FLAG_FOLIAGE_SWAY: u32 = 0x02;
    pub const FLAG_EMISSIVE: u32 = 0x04;
    pub const FLAG_SKIRT: u32 = 0x08;

    #[inline(always)]
    pub fn encode(
        local_x: u32,       // 0..31 (5 bits)
        world_y: i32,       // -128..+895 (10 bits, biased +128)
        local_z: u32,       // 0..31 (5 bits)
        material_id: u32,   // 0..511 (9 bits)
        normal_idx: u32,    // 0..5 (3 bits)
        ambient_occlusion: u32, // 0..3 (2 bits)
        block_light: u32,   // 0..15 (4 bits)
        sky_light: u32,     // 0..15 (4 bits)
        flags: u32,         // 0..15 (4 bits)
        tint_or_flow: u32,  // 18 bits (RGB 6:6:6 or water flow parameters)
    ) -> Self {
        let biased_y = (world_y + 128).clamp(0, 1023) as u64;
        let val = (local_x as u64 & 0x1F)
            | (biased_y << 5)
            | ((local_z as u64 & 0x1F) << 15)
            | ((material_id as u64 & 0x1FF) << 20)
            | ((normal_idx as u64 & 0x07) << 29)
            | ((ambient_occlusion as u64 & 0x03) << 32)
            | ((block_light as u64 & 0x0F) << 34)
            | ((sky_light as u64 & 0x0F) << 38)
            | ((flags as u64 & 0x0F) << 42)
            | ((tint_or_flow as u64 & 0x3FFFF) << 46);
        ReconciledVertex64(val)
    }

    #[inline(always)]
    pub fn local_x(&self) -> u32 {
        (self.0 & 0x1F) as u32
    }

    #[inline(always)]
    pub fn world_y(&self) -> i32 {
        (((self.0 >> 5) & 0x3FF) as i32) - 128
    }

    #[inline(always)]
    pub fn local_z(&self) -> u32 {
        ((self.0 >> 15) & 0x1F) as u32
    }

    #[inline(always)]
    pub fn material_id(&self) -> u32 {
        ((self.0 >> 20) & 0x1FF) as u32
    }

    #[inline(always)]
    pub fn normal_idx(&self) -> u32 {
        ((self.0 >> 29) & 0x07) as u32
    }

    #[inline(always)]
    pub fn ambient_occlusion(&self) -> u32 {
        ((self.0 >> 32) & 0x03) as u32
    }

    #[inline(always)]
    pub fn block_light(&self) -> u32 {
        ((self.0 >> 34) & 0x0F) as u32
    }

    #[inline(always)]
    pub fn sky_light(&self) -> u32 {
        ((self.0 >> 38) & 0x0F) as u32
    }

    #[inline(always)]
    pub fn flags(&self) -> u32 {
        ((self.0 >> 42) & 0x0F) as u32
    }

    #[inline(always)]
    pub fn tint_or_flow(&self) -> u32 {
        ((self.0 >> 46) & 0x3FFFF) as u32
    }
}

// C-ABI Exports
#[no_mangle]
pub extern "C" fn hz_vertex_encode(
    x: u32,
    y: u32,
    z: u32,
    normal_idx: u32,
    block_light: u32,
    sky_light: u32,
    u: u8,
    v: u8,
    material_id: u16,
) -> u64 {
    PackedVertex64::encode(x, y, z, normal_idx, block_light, sky_light, u, v, material_id).0
}

#[no_mangle]
pub extern "C" fn hz_reconciled_vertex_encode(
    local_x: u32,
    world_y: i32,
    local_z: u32,
    material_id: u32,
    normal_idx: u32,
    ao: u32,
    block_light: u32,
    sky_light: u32,
    flags: u32,
    tint_or_flow: u32,
) -> u64 {
    ReconciledVertex64::encode(
        local_x,
        world_y,
        local_z,
        material_id,
        normal_idx,
        ao,
        block_light,
        sky_light,
        flags,
        tint_or_flow,
    )
    .0
}