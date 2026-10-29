//! FFI definitions, safety contracts, and C-ABI export verifications.

pub mod spsc_ring;
pub use crate::arena::guard::*;
pub use crate::arena::*;
pub use crate::mesher::debris::*;
pub use crate::mesher::mesh_format::*;
pub use crate::mesher::meshlet::*;
pub use crate::mesher::parallax::*;
pub use crate::mesher::sdf_heightfield::*;
pub use crate::mesher::subterranean::*;
pub use crate::mesher::terrain_deform::*;
pub use crate::mesher::vertex_layout::*;
pub use crate::simd::acoustic::*;
pub use crate::simd::ast_compiler::*;
pub use crate::simd::aurora::*;
pub use crate::simd::caustic::*;
pub use crate::simd::cloud_shadow::*;
pub use crate::simd::ephemeris::*;
pub use crate::simd::planetary_dynamics::*;
pub use crate::simd::precipitation::*;
pub use crate::simd::quaternion::*;
pub use crate::simd::radiance_cache::*;
pub use crate::simd::subsurface::*;
pub use crate::simd::thermal::*;
pub use crate::simd::weather_wind::*;
pub use crate::simd::*;
pub use crate::storage::morton_svo::*;
pub use crate::storage::wal_recovery::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert_eq!(crate::hz_core_version_major(), 0);
        assert_eq!(crate::hz_core_version_minor(), 1);
        assert_eq!(crate::hz_core_version_patch(), 0);
    }

    #[test]
    fn test_arena_lifecycle() {
        let arena = hz_arena_create();
        assert!(!arena.is_null());

        let ptr = hz_arena_alloc(arena, 128, 8);
        assert!(!ptr.is_null());

        hz_arena_reset(arena);
        hz_arena_destroy(arena);
    }

    #[test]
    fn test_simd_add() {
        let a = vec![1.0f32, 2.0, 3.0, 4.0];
        let b = vec![10.0f32, 20.0, 30.0, 40.0];
        let mut out = vec![0.0f32; 4];

        hz_simd_add_f32(a.as_ptr(), b.as_ptr(), out.as_mut_ptr(), 4);
        assert_eq!(out, vec![11.0, 22.0, 33.0, 44.0]);
    }

    #[test]
    fn test_spsc_ring_buffer() {
        let ring = spsc_ring::hz_spsc_create();
        assert!(!ring.is_null());

        let res = spsc_ring::hz_spsc_push_chunk_task(ring, 12, -4, 0, 1, 0, 0xCAFEBABE, 1024, 7);
        assert_eq!(res, 0);

        let mut entry = spsc_ring::FfiChunkTaskPayload::default();
        let pop_res = spsc_ring::hz_spsc_pop_chunk_task(ring, &mut entry);
        assert_eq!(pop_res, 0);
        assert_eq!(entry.chunk_x, 12);
        assert_eq!(entry.chunk_z, -4);
        assert_eq!(entry.lod_tier, 1);
        assert_eq!(entry.data_ptr, 0xCAFEBABE);
        assert_eq!(entry.data_length, 1024);
        assert_eq!(entry.sequence_tag, 7);

        spsc_ring::hz_spsc_destroy(ring);
    }

    #[test]
    fn test_wal_crc32c_and_verification() {
        let payload = b"Horizon Ecosystem Chunk Data Payload";
        let crc = hz_wal_crc32c(payload.as_ptr(), payload.len());
        assert_ne!(crc, 0);

        let header = WalRecordHeader::new(1, 42, payload.len() as u32, crc);
        let verify_status = hz_wal_verify_record(&header, payload.as_ptr(), payload.len());
        assert_eq!(verify_status, 0);
    }

    #[test]
    fn test_packed_vertex64_roundtrip() {
        let encoded = PackedVertex64::encode(15, 320, 63, 4, 12, 15, 64, 128, 4096);
        assert_eq!(encoded.x(), 15);
        assert_eq!(encoded.y(), 320);
        assert_eq!(encoded.z(), 63);
        assert_eq!(encoded.normal_idx(), 4);
        assert_eq!(encoded.block_light(), 12);
        assert_eq!(encoded.sky_light(), 15);
        assert_eq!(encoded.u(), 64);
        assert_eq!(encoded.v(), 128);
        assert_eq!(encoded.material_id(), 4096);
    }

    #[test]
    fn test_guarded_buffer_lifecycle() {
        let size = 4096;
        let buf = hz_memory_alloc_guarded(size);
        assert!(!buf.is_null());
        assert!(hz_memory_watchdog_bytes() >= size);
        hz_memory_free_guarded(buf, size);
    }

    #[test]
    fn test_quaternion_compression_roundtrip() {
        let q = [0.0f32, 0.70710678f32, 0.0f32, 0.70710678f32];
        let packed = hz_quat_pack(q[0], q[1], q[2], q[3]);
        let mut out = [0.0f32; 4];
        hz_quat_unpack(packed, out.as_mut_ptr());

        assert!((out[0] - q[0]).abs() < 0.005);
        assert!((out[1] - q[1]).abs() < 0.005);
        assert!((out[2] - q[2]).abs() < 0.005);
        assert!((out[3] - q[3]).abs() < 0.005);
    }

    #[test]
    fn test_subterranean_cull() {
        let header = SubterraneanSpanHeader {
            min_x: 0,
            min_z: 0,
            surface_roof_y: 64,
            cave_ceiling_max_y: 10,
            cave_floor_min_y: -60,
            surface_aperture_mask: 0,
            cave_meshlet_count: 10,
            cave_meshlet_offset: 0,
        };
        assert_eq!(hz_subterranean_cull(&header, 70.0), 1);
        assert_eq!(hz_subterranean_cull(&header, 5.0), 0);
    }

    #[test]
    fn test_acoustic_t60_solver() {
        let t60 = hz_acoustic_solve_t60(50000.0, 10000.0, 0.05);
        assert!(t60 > 1.0 && t60 <= 4.5);
    }

    #[test]
    fn test_ephemeris_eclipse_obscuration() {
        let total = hz_eclipse_obscuration(0.0, 0.005, 0.005);
        assert_eq!(total, 1.0);
        let none = hz_eclipse_obscuration(0.02, 0.005, 0.005);
        assert_eq!(none, 0.0);
    }

    #[test]
    fn test_morton_encoding_roundtrip() {
        let code = hz_svo_morton_encode(10, 20, 30);
        assert_ne!(code, 0);
    }

    #[test]
    fn test_thermal_air_refractivity() {
        let n_cold = hz_thermal_air_refractivity(273.15);
        let n_hot = hz_thermal_air_refractivity(1200.0);
        assert!(n_cold > n_hot, "Cold air must have higher refractive index than blazing air");
    }

    #[test]
    fn test_meshlet_cone_culling() {
        let desc = MeshletDescriptor {
            center_radius: [0.0, 0.0, 10.0, 5.0],
            cone_axis_cutoff: [0.0, 0.0, 1.0, 0.5],
            vertex_offset: 0,
            primitive_offset: 0,
            vertex_count: 64,
            primitive_count: 126,
            lod_level: 1,
            flags: 0,
        };
        assert_eq!(hz_meshlet_cone_cull(&desc, 0.0, 0.0, 0.0), 1);
    }

    #[test]
    fn test_distant_sdf_and_height_mip() {
        let sdf = DistantSdfHeader::new(0, 0, 512, 512, 4.0);
        assert_eq!(sdf.world_width_blocks(), 2048.0);

        let node1 = HeightMipNode { min_y: 10, max_y: 40 };
        let node2 = HeightMipNode { min_y: -5, max_y: 64 };
        let merged = node1.merge(&node2);
        assert_eq!(merged.min_y, -5);
        assert_eq!(merged.max_y, 64);
        assert!(merged.contains_y(32));
        assert!(!merged.contains_y(70));
    }

    #[test]
    fn test_weather_and_precipitation_uniforms() {
        let wind = MacroWindUniforms::new(10.0, 0.0, 8.5, 0.3);
        assert!((wind.global_wind_dir[0] - 1.0).abs() < 1e-4);
        assert_eq!(wind.global_wind_dir[1], 0.0);

        let precip = PrecipitationUniforms::new([0.0, 70.0, 0.0], PrecipitationUniforms::TYPE_RAIN);
        assert!(precip.is_active());
        assert_eq!(precip.terminal_velocity, 9.0);
    }

    #[test]
    fn test_parallax_and_subsurface_profiles() {
        let pom = ParallaxLODUniforms::new(0.08, 0.0625);
        let steps_near = pom.compute_steps(100.0, 1.0);
        let steps_far = pom.compute_steps(600.0, 1.0);
        assert!(steps_near >= steps_far);

        let foliage = SubsurfaceProfile::foliage();
        assert_eq!(foliage.profile_id, SubsurfaceProfile::PROFILE_FOLIAGE);
        assert_eq!(foliage.ior, 1.33);
    }

    #[test]
    fn test_self_healing_wal_recovery() {
        use std::io::Cursor;

        let mut wal_data = Vec::new();
        let chunk_payload = vec![42u8; 128];
        let crc = crc32c(&chunk_payload);
        let header = WalRecordHeader::new(1, 0, chunk_payload.len() as u32, crc);

        // Write header and payload into mock WAL
        let header_bytes = unsafe {
            std::slice::from_raw_parts(
                &header as *const _ as *const u8,
                core::mem::size_of::<WalRecordHeader>(),
            )
        };
        wal_data.extend_from_slice(header_bytes);
        wal_data.extend_from_slice(&chunk_payload);

        let region_data = vec![0u8; 4096];
        let mut manager = SelfHealingRegionManager::new(
            Cursor::new(region_data),
            Cursor::new(wal_data),
        );

        let recovered = manager.recover_from_wal().expect("WAL recovery must succeed");
        assert_eq!(recovered, 1);
        assert_eq!(&manager.region_file.into_inner()[0..128], &chunk_payload[..]);
    }

    #[test]
    fn test_mesh_format_headers() {
        let header = HzMeshHeader::new(10, -5, HzMeshHeader::FORMAT_MODE64, HzMeshHeader::COMPRESSION_LZ4, 1024);
        assert!(header.is_valid());
        assert_eq!(header.chunk_x, 10);
        assert_eq!(header.chunk_z, -5);

        let span = TerrainColumnSpan::single(60, 72, 1);
        assert_eq!(span.interval_count, 1);
        assert_eq!(span.intervals[0].y_bottom, 60);
        assert_eq!(span.intervals[0].y_top, 72);
    }

    #[test]
    fn test_planetary_and_manifold_uniforms() {
        assert_eq!(core::mem::size_of::<PlanetaryUniformData>(), 80);
        assert_eq!(core::mem::size_of::<PolarizedAtmosphereUniforms>(), 64);
        assert_eq!(core::mem::size_of::<PortalGpuData>(), 160);
        assert_eq!(core::mem::size_of::<ThinFilmMaterialGpuRecord>(), 64);
    }

    #[test]
    fn test_simd_column_mesher() {
        crate::mesher::simd_column::init_mesher_dispatch();
        let mut columns = crate::mesher::simd_column::VoxelColumn16 { solid_mask: [0; 16] };
        columns.solid_mask[0] = 0b0101_0101;
        let mut out_faces = [0u64; 64];
        let count = crate::mesher::simd_column::hz_mesher_extract_faces(
            &columns,
            out_faces.as_mut_ptr(),
            64,
        );
        assert!(count > 0);
        assert_ne!(out_faces[0], 0);
    }
}