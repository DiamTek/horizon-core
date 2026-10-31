package net.horizon.core.ffi;

import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.util.Objects;

/**
 * Panama Foreign Function and Memory (FFM) bridge into horizon-core-native.
 * Delivers sub-2ns downcalls for arena allocations, SPSC queues, CRC32C validation,
 * SIMD vector math, and quaternion compression.
 */
public final class HorizonNativeBridge {

    private static final Linker LINKER = Linker.nativeLinker();

    private final MethodHandle fnCoreVersionMajor;
    private final MethodHandle fnCoreVersionMinor;
    private final MethodHandle fnCoreVersionPatch;
    private final MethodHandle fnCoreInit;

    private final MethodHandle fnArenaCreate;
    private final MethodHandle fnArenaAlloc;
    private final MethodHandle fnArenaReset;
    private final MethodHandle fnArenaDestroy;

    private final MethodHandle fnSpscCreate;
    private final MethodHandle fnSpscDestroy;
    private final MethodHandle fnSpscPush;
    private final MethodHandle fnSpscPop;
    private final MethodHandle fnSpscPopBatch;
    private final MethodHandle fnSpscVerifyAndRecover;
    private final MethodHandle fnSpscInitInPlace;
    private final MethodHandle fnSpscRingSize;

    private final MethodHandle fnMemoryWatchdogBytes;
    private final MethodHandle fnMemoryAllocGuarded;
    private final MethodHandle fnMemoryFreeGuarded;

    private final MethodHandle fnSchedulerWorkerCount;
    private final MethodHandle fnEclipseObscuration;
    private final MethodHandle fnSvoMortonEncode;
    private final MethodHandle fnThermalAirRefractivity;
    private final MethodHandle fnAcousticSolveT60;
    private final MethodHandle fnMeshletConeCull;
    private final MethodHandle fnSubterraneanCull;

    private final MethodHandle fnWalCrc32c;
    private final MethodHandle fnSimdDot;
    private final MethodHandle fnQuatPack;
    private final MethodHandle fnQuatUnpack;

    public HorizonNativeBridge(java.lang.foreign.SymbolLookup lookup) {
        Objects.requireNonNull(lookup, "Native symbol lookup cannot be null");

        this.fnCoreVersionMajor = lookup.find("hz_core_version_major")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(ValueLayout.JAVA_INT)))
                .orElse(null);

        this.fnCoreVersionMinor = lookup.find("hz_core_version_minor")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(ValueLayout.JAVA_INT)))
                .orElse(null);

        this.fnCoreVersionPatch = lookup.find("hz_core_version_patch")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(ValueLayout.JAVA_INT)))
                .orElse(null);

        this.fnCoreInit = lookup.find("hz_core_init")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(ValueLayout.JAVA_INT)))
                .orElse(null);

        this.fnArenaCreate = lookup.find("hz_arena_create")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(ValueLayout.ADDRESS)))
                .orElse(null);

        this.fnArenaAlloc = lookup.find("hz_arena_alloc")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.ADDRESS,
                        ValueLayout.ADDRESS,
                        ValueLayout.JAVA_LONG,
                        ValueLayout.JAVA_LONG
                )))
                .orElse(null);

        this.fnArenaReset = lookup.find("hz_arena_reset")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.ofVoid(ValueLayout.ADDRESS)))
                .orElse(null);

        this.fnArenaDestroy = lookup.find("hz_arena_destroy")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.ofVoid(ValueLayout.ADDRESS)))
                .orElse(null);

        this.fnSpscCreate = lookup.find("hz_spsc_create")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(ValueLayout.ADDRESS)))
                .orElse(null);

        this.fnSpscDestroy = lookup.find("hz_spsc_destroy")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.ofVoid(ValueLayout.ADDRESS)))
                .orElse(null);

        this.fnSpscPush = lookup.find("hz_spsc_push_chunk_task")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_INT,
                        ValueLayout.ADDRESS,
                        ValueLayout.JAVA_INT,
                        ValueLayout.JAVA_INT,
                        ValueLayout.JAVA_SHORT,
                        ValueLayout.JAVA_BYTE,
                        ValueLayout.JAVA_BYTE,
                        ValueLayout.JAVA_LONG,
                        ValueLayout.JAVA_INT,
                        ValueLayout.JAVA_INT
                )))
                .orElse(null);

        this.fnSpscPop = lookup.find("hz_spsc_pop_chunk_task")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_INT,
                        ValueLayout.ADDRESS,
                        ValueLayout.ADDRESS
                )))
                .orElse(null);

        this.fnSpscPopBatch = lookup.find("hz_spsc_pop_batch")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_LONG,
                        ValueLayout.ADDRESS,
                        ValueLayout.ADDRESS,
                        ValueLayout.JAVA_LONG
                )))
                .orElse(null);

        this.fnSpscVerifyAndRecover = lookup.find("hz_spsc_verify_and_recover")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_INT,
                        ValueLayout.ADDRESS
                )))
                .orElse(null);

        this.fnSpscInitInPlace = lookup.find("hz_spsc_init_in_place")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.ADDRESS,
                        ValueLayout.ADDRESS,
                        ValueLayout.JAVA_LONG
                )))
                .orElse(null);

        this.fnSpscRingSize = lookup.find("hz_spsc_ring_size")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(ValueLayout.JAVA_LONG)))
                .orElse(null);

        this.fnMemoryWatchdogBytes = lookup.find("hz_memory_watchdog_bytes")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(ValueLayout.JAVA_LONG)))
                .orElse(null);

        this.fnMemoryAllocGuarded = lookup.find("hz_memory_alloc_guarded")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.ADDRESS,
                        ValueLayout.JAVA_LONG
                )))
                .orElse(null);

        this.fnMemoryFreeGuarded = lookup.find("hz_memory_free_guarded")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.ofVoid(
                        ValueLayout.ADDRESS,
                        ValueLayout.JAVA_LONG
                )))
                .orElse(null);

        this.fnSchedulerWorkerCount = lookup.find("hz_scheduler_worker_count")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(ValueLayout.JAVA_INT)))
                .orElse(null);

        this.fnEclipseObscuration = lookup.find("hz_eclipse_obscuration")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT
                )))
                .orElse(null);

        this.fnSvoMortonEncode = lookup.find("hz_svo_morton_encode")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_LONG,
                        ValueLayout.JAVA_INT,
                        ValueLayout.JAVA_INT,
                        ValueLayout.JAVA_INT
                )))
                .orElse(null);

        this.fnThermalAirRefractivity = lookup.find("hz_thermal_air_refractivity")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT
                )))
                .orElse(null);

        this.fnAcousticSolveT60 = lookup.find("hz_acoustic_solve_t60")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT
                )))
                .orElse(null);

        this.fnMeshletConeCull = lookup.find("hz_meshlet_cone_cull")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_INT,
                        ValueLayout.ADDRESS,
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT
                )))
                .orElse(null);

        this.fnSubterraneanCull = lookup.find("hz_subterranean_cull")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_INT,
                        ValueLayout.ADDRESS,
                        ValueLayout.JAVA_FLOAT
                )))
                .orElse(null);

        this.fnWalCrc32c = lookup.find("hz_wal_crc32c")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_INT,
                        ValueLayout.ADDRESS,
                        ValueLayout.JAVA_LONG
                )))
                .orElse(null);

        this.fnSimdDot = lookup.find("hz_simd_dot_product_f32")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.ADDRESS,
                        ValueLayout.ADDRESS,
                        ValueLayout.JAVA_LONG
                )))
                .orElse(null);

        this.fnQuatPack = lookup.find("hz_quat_pack")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.of(
                        ValueLayout.JAVA_INT,
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT,
                        ValueLayout.JAVA_FLOAT
                )))
                .orElse(null);

        this.fnQuatUnpack = lookup.find("hz_quat_unpack")
                .map(addr -> LINKER.downcallHandle(addr, FunctionDescriptor.ofVoid(
                        ValueLayout.JAVA_INT,
                        ValueLayout.ADDRESS
                )))
                .orElse(null);
    }

    public int getVersionMajor() {
        try {
            return fnCoreVersionMajor != null ? (int) fnCoreVersionMajor.invokeExact() : 0;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_core_version_major", t);
        }
    }

    public int getVersionMinor() {
        try {
            return fnCoreVersionMinor != null ? (int) fnCoreVersionMinor.invokeExact() : 0;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_core_version_minor", t);
        }
    }

    public int getVersionPatch() {
        try {
            return fnCoreVersionPatch != null ? (int) fnCoreVersionPatch.invokeExact() : 0;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_core_version_patch", t);
        }
    }

    public int initCore() {
        try {
            return fnCoreInit != null ? (int) fnCoreInit.invokeExact() : 0;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_core_init", t);
        }
    }

    public MemorySegment createArena() {
        try {
            return fnArenaCreate != null ? (MemorySegment) fnArenaCreate.invokeExact() : MemorySegment.NULL;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_arena_create", t);
        }
    }

    public MemorySegment allocInArena(MemorySegment arenaPtr, long size, long alignment) {
        try {
            return fnArenaAlloc != null && !arenaPtr.equals(MemorySegment.NULL)
                    ? (MemorySegment) fnArenaAlloc.invokeExact(arenaPtr, size, alignment)
                    : MemorySegment.NULL;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_arena_alloc", t);
        }
    }

    public void resetArena(MemorySegment arenaPtr) {
        try {
            if (fnArenaReset != null && !arenaPtr.equals(MemorySegment.NULL)) {
                fnArenaReset.invokeExact(arenaPtr);
            }
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_arena_reset", t);
        }
    }

    public void destroyArena(MemorySegment arenaPtr) {
        try {
            if (fnArenaDestroy != null && !arenaPtr.equals(MemorySegment.NULL)) {
                fnArenaDestroy.invokeExact(arenaPtr);
            }
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_arena_destroy", t);
        }
    }

    public MemorySegment createSpscRing() {
        try {
            return fnSpscCreate != null ? (MemorySegment) fnSpscCreate.invokeExact() : MemorySegment.NULL;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_spsc_create", t);
        }
    }

    public void destroySpscRing(MemorySegment ringPtr) {
        try {
            if (fnSpscDestroy != null && !ringPtr.equals(MemorySegment.NULL)) {
                fnSpscDestroy.invokeExact(ringPtr);
            }
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_spsc_destroy", t);
        }
    }

    public int pushSpscTask(MemorySegment ringPtr, int chunkX, int chunkZ, short dimensionId, byte lodTier, byte flags, long dataPtr, int dataLength, int seqTag) {
        try {
            return fnSpscPush != null && !ringPtr.equals(MemorySegment.NULL)
                    ? (int) fnSpscPush.invokeExact(ringPtr, chunkX, chunkZ, dimensionId, lodTier, flags, dataPtr, dataLength, seqTag)
                    : -1;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_spsc_push_chunk_task", t);
        }
    }

    public int popSpscTask(MemorySegment ringPtr, MemorySegment outEntry) {
        try {
            return fnSpscPop != null && !ringPtr.equals(MemorySegment.NULL)
                    ? (int) fnSpscPop.invokeExact(ringPtr, outEntry)
                    : -1;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_spsc_pop_chunk_task", t);
        }
    }

    public long popSpscBatch(MemorySegment ringPtr, MemorySegment destArraySegment, long maxCount) {
        try {
            return fnSpscPopBatch != null && !ringPtr.equals(MemorySegment.NULL) && !destArraySegment.equals(MemorySegment.NULL)
                    ? (long) fnSpscPopBatch.invokeExact(ringPtr, destArraySegment, maxCount)
                    : 0L;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_spsc_pop_batch", t);
        }
    }

    public int verifyAndRecoverSpsc(MemorySegment ringPtr) {
        try {
            return fnSpscVerifyAndRecover != null && !ringPtr.equals(MemorySegment.NULL)
                    ? (int) fnSpscVerifyAndRecover.invokeExact(ringPtr)
                    : -1;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_spsc_verify_and_recover", t);
        }
    }

    public MemorySegment initSpscInPlace(MemorySegment rawPtr, long totalBytes) {
        try {
            return fnSpscInitInPlace != null && !rawPtr.equals(MemorySegment.NULL)
                    ? (MemorySegment) fnSpscInitInPlace.invokeExact(rawPtr, totalBytes)
                    : MemorySegment.NULL;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_spsc_init_in_place", t);
        }
    }

    public long getSpscRingSize() {
        try {
            return fnSpscRingSize != null ? (long) fnSpscRingSize.invokeExact() : 0L;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_spsc_ring_size", t);
        }
    }

    public long getWatchdogAllocatedBytes() {
        try {
            return fnMemoryWatchdogBytes != null ? (long) fnMemoryWatchdogBytes.invokeExact() : 0L;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_memory_watchdog_bytes", t);
        }
    }

    public MemorySegment allocGuardedBuffer(long size) {
        try {
            return fnMemoryAllocGuarded != null && size > 0
                    ? (MemorySegment) fnMemoryAllocGuarded.invokeExact(size)
                    : MemorySegment.NULL;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_memory_alloc_guarded", t);
        }
    }

    public void freeGuardedBuffer(MemorySegment ptr, long size) {
        try {
            if (fnMemoryFreeGuarded != null && !ptr.equals(MemorySegment.NULL) && size > 0) {
                fnMemoryFreeGuarded.invokeExact(ptr, size);
            }
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_memory_free_guarded", t);
        }
    }

    public int getSchedulerWorkerCount() {
        try {
            return fnSchedulerWorkerCount != null ? (int) fnSchedulerWorkerCount.invokeExact() : 1;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_scheduler_worker_count", t);
        }
    }

    public float computeEclipseObscuration(float angularSeparation, float sunRadius, float moonRadius) {
        try {
            return fnEclipseObscuration != null
                    ? (float) fnEclipseObscuration.invokeExact(angularSeparation, sunRadius, moonRadius)
                    : 0.0f;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_eclipse_obscuration", t);
        }
    }

    public long encodeSvoMorton(int x, int y, int z) {
        try {
            return fnSvoMortonEncode != null
                    ? (long) fnSvoMortonEncode.invokeExact(x, y, z)
                    : 0L;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_svo_morton_encode", t);
        }
    }

    public float computeThermalAirRefractivity(float temperatureKelvin) {
        try {
            return fnThermalAirRefractivity != null
                    ? (float) fnThermalAirRefractivity.invokeExact(temperatureKelvin)
                    : 1.0f;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_thermal_air_refractivity", t);
        }
    }

    public float solveAcousticT60(float roomVolume, float surfaceArea, float absorption) {
        try {
            return fnAcousticSolveT60 != null
                    ? (float) fnAcousticSolveT60.invokeExact(roomVolume, surfaceArea, absorption)
                    : 0.8f;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_acoustic_solve_t60", t);
        }
    }

    public int coneCullMeshlet(MemorySegment meshletDesc, float camX, float camY, float camZ) {
        try {
            return fnMeshletConeCull != null && !meshletDesc.equals(MemorySegment.NULL)
                    ? (int) fnMeshletConeCull.invokeExact(meshletDesc, camX, camY, camZ)
                    : 0;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_meshlet_cone_cull", t);
        }
    }

    public int cullSubterranean(MemorySegment spanHeader, float cameraY) {
        try {
            return fnSubterraneanCull != null && !spanHeader.equals(MemorySegment.NULL)
                    ? (int) fnSubterraneanCull.invokeExact(spanHeader, cameraY)
                    : 0;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_subterranean_cull", t);
        }
    }

    public int computeCrc32c(MemorySegment dataSegment, long length) {
        try {
            return fnWalCrc32c != null && !dataSegment.equals(MemorySegment.NULL)
                    ? (int) fnWalCrc32c.invokeExact(dataSegment, length)
                    : 0;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_wal_crc32c", t);
        }
    }

    public float computeSimdDot(MemorySegment a, MemorySegment b, long count) {
        try {
            return fnSimdDot != null && !a.equals(MemorySegment.NULL) && !b.equals(MemorySegment.NULL)
                    ? (float) fnSimdDot.invokeExact(a, b, count)
                    : 0.0f;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_simd_dot_product_f32", t);
        }
    }

    public int packQuaternion(float x, float y, float z, float w) {
        try {
            return fnQuatPack != null ? (int) fnQuatPack.invokeExact(x, y, z, w) : 0;
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_quat_pack", t);
        }
    }

    public void unpackQuaternion(int packed, MemorySegment outXyzw) {
        try {
            if (fnQuatUnpack != null && !outXyzw.equals(MemorySegment.NULL)) {
                fnQuatUnpack.invokeExact(packed, outXyzw);
            }
        } catch (Throwable t) {
            throw new RuntimeException("Failed to invoke hz_quat_unpack", t);
        }
    }
}