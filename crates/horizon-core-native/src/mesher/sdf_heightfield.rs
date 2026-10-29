//! Distance field and hierarchical mipmap heightfield structures for conservative raymarching.

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DistantSdfHeader {
    pub origin_chunk_x: i32,
    pub origin_chunk_z: i32,
    pub width_texels: u32,
    pub height_texels: u32,
    pub block_scale: f32,
    pub min_elevation: f32,
    pub max_elevation: f32,
}

impl DistantSdfHeader {
    pub fn new(origin_chunk_x: i32, origin_chunk_z: i32, width: u32, height: u32, scale: f32) -> Self {
        Self {
            origin_chunk_x,
            origin_chunk_z,
            width_texels: width,
            height_texels: height,
            block_scale: scale,
            min_elevation: -64.0,
            max_elevation: 320.0,
        }
    }

    #[inline(always)]
    pub fn world_width_blocks(&self) -> f32 {
        self.width_texels as f32 * self.block_scale
    }

    #[inline(always)]
    pub fn world_height_blocks(&self) -> f32 {
        self.height_texels as f32 * self.block_scale
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeightMipNode {
    pub min_y: i16,
    pub max_y: i16,
}

impl HeightMipNode {
    pub const EMPTY: Self = Self {
        min_y: i16::MAX,
        max_y: i16::MIN,
    };

    #[inline(always)]
    pub fn merge(&self, other: &Self) -> Self {
        Self {
            min_y: self.min_y.min(other.min_y),
            max_y: self.max_y.max(other.max_y),
        }
    }

    #[inline(always)]
    pub fn contains_y(&self, y: i16) -> bool {
        y >= self.min_y && y <= self.max_y
    }
}