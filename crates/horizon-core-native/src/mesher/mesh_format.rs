//! Binary chunk mesh format definitions, headers, and multi-interval column spans.

use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HzMeshHeader {
    pub magic: [u8; 4],
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub vertex_format: u8,
    pub compression_type: u8,
    pub palette_size: u16,
    pub vertex_count: u32,
}

impl HzMeshHeader {
    pub const MAGIC_V1: [u8; 4] = *b"HZM1";
    pub const MAGIC_V2: [u8; 4] = *b"HZM2";

    pub const FORMAT_MODE64: u8 = 0;
    pub const FORMAT_MODE128: u8 = 1;

    pub const COMPRESSION_NONE: u8 = 0;
    pub const COMPRESSION_LZ4: u8 = 1;
    pub const COMPRESSION_ZSTD: u8 = 2;

    pub fn new(chunk_x: i32, chunk_z: i32, vertex_format: u8, compression_type: u8, vertex_count: u32) -> Self {
        Self {
            magic: Self::MAGIC_V2,
            chunk_x,
            chunk_z,
            vertex_format,
            compression_type,
            palette_size: 0,
            vertex_count,
        }
    }

    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.magic == Self::MAGIC_V1 || self.magic == Self::MAGIC_V2
    }
}

/// GPU-side draw metadata buffer entry representing relative-to-camera chunk origins.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Pod, Zeroable)]
pub struct ChunkDrawMetadata {
    pub chunk_world_x: i32,
    pub chunk_world_z: i32,
    pub palette_base: u32,
    pub render_flags: u32,
}

impl ChunkDrawMetadata {
    pub fn new(chunk_x: i32, chunk_z: i32, palette_base: u32, render_flags: u32) -> Self {
        Self {
            chunk_world_x: chunk_x * 16,
            chunk_world_z: chunk_z * 16,
            palette_base,
            render_flags,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerrainColumnSpan {
    pub intervals: [VoxelInterval; 4],
    pub interval_count: u8,
}

impl TerrainColumnSpan {
    pub const EMPTY: Self = Self {
        intervals: [VoxelInterval::EMPTY; 4],
        interval_count: 0,
    };

    pub fn single(y_bottom: i16, y_top: i16, material_id: u16) -> Self {
        let mut span = Self::EMPTY;
        span.intervals[0] = VoxelInterval {
            y_bottom,
            y_top,
            material_id,
        };
        span.interval_count = 1;
        span
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoxelInterval {
    pub y_bottom: i16,
    pub y_top: i16,
    pub material_id: u16,
}

impl VoxelInterval {
    pub const EMPTY: Self = Self {
        y_bottom: 0,
        y_top: 0,
        material_id: 0,
    };
}