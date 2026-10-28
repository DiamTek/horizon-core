//! FFI definitions, safety contracts, and C-ABI export verifications.

pub use crate::arena::*;
pub use crate::simd::*;

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
}