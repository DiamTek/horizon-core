package net.horizon.core.ffi;

import org.junit.jupiter.api.Test;
import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.SymbolLookup;
import java.lang.foreign.ValueLayout;
import static org.junit.jupiter.api.Assertions.*;

@SuppressWarnings("preview")
class HorizonNativeBridgeTest {

    @Test
    void testEndToEndNativeBridgeHandshakeAndSpscTransfer() {
        SymbolLookup lookup = NativeLoader.loadNativeCore();
        assertNotNull(lookup, "Native SymbolLookup must not be null");

        HorizonNativeBridge bridge = new HorizonNativeBridge(lookup);

        // 1. Version Handshake
        int major = bridge.getVersionMajor();
        int minor = bridge.getVersionMinor();
        int patch = bridge.getVersionPatch();
        assertEquals(0, major);
        assertEquals(1, minor);
        assertEquals(0, patch);

        // 2. Subsystem Init
        int initStatus = bridge.initCore();
        assertEquals(0, initStatus, "hz_core_init must succeed with 0");

        // 3. SPSC Lock-Free Ring Buffer Push & Pop (No Segfaults)
        MemorySegment ring = bridge.createSpscRing();
        assertNotEquals(MemorySegment.NULL, ring, "Allocated SPSC ring buffer pointer must not be null");

        int pushRes = bridge.pushSpscTask(
                ring,
                120,
                -45,
                (short) -1,
                (byte) 2,
                (byte) 0x05,
                0xDEADBEEFCAFEBABEL,
                8192,
                42
        );
        assertEquals(0, pushRes, "Push into empty ring buffer must succeed with 0");

        try (Arena arena = Arena.ofConfined()) {
            MemorySegment outEntry = arena.allocate(32, 8);
            int popRes = bridge.popSpscTask(ring, outEntry);
            assertEquals(0, popRes, "Pop from ring buffer must succeed with 0");

            int chunkX = outEntry.get(ValueLayout.JAVA_INT, 0);
            int chunkZ = outEntry.get(ValueLayout.JAVA_INT, 4);
            short dimensionId = outEntry.get(ValueLayout.JAVA_SHORT, 8);
            byte lodTier = outEntry.get(ValueLayout.JAVA_BYTE, 10);
            byte flags = outEntry.get(ValueLayout.JAVA_BYTE, 11);
            long dataPtr = outEntry.get(ValueLayout.JAVA_LONG, 16);
            int dataLength = outEntry.get(ValueLayout.JAVA_INT, 24);
            int seqTag = outEntry.get(ValueLayout.JAVA_INT, 28);

            assertEquals(120, chunkX);
            assertEquals(-45, chunkZ);
            assertEquals((short) -1, dimensionId);
            assertEquals((byte) 2, lodTier);
            assertEquals((byte) 0x05, flags);
            assertEquals(0xDEADBEEFCAFEBABEL, dataPtr);
            assertEquals(8192, dataLength);
            assertEquals(42, seqTag);
        }

        bridge.destroySpscRing(ring);

        // 4. Linear Arena Allocator Lifecycle
        MemorySegment nativeArena = bridge.createArena();
        assertNotEquals(MemorySegment.NULL, nativeArena);

        MemorySegment allocatedBuffer = bridge.allocInArena(nativeArena, 2048, 64);
        assertNotEquals(MemorySegment.NULL, allocatedBuffer);
        assertEquals(0, allocatedBuffer.address() % 64, "Arena allocation must satisfy 64-byte alignment");

        bridge.resetArena(nativeArena);
        bridge.destroyArena(nativeArena);

        // 5. Hardware CRC32C Calculation
        try (Arena arena = Arena.ofConfined()) {
            byte[] bytes = "Horizon Native FFM Pipeline Verification Test String".getBytes(java.nio.charset.StandardCharsets.UTF_8);
            MemorySegment testData = arena.allocate(bytes.length, 1);
            MemorySegment.copy(bytes, 0, testData, ValueLayout.JAVA_BYTE, 0, bytes.length);
            int crc = bridge.computeCrc32c(testData, testData.byteSize());
            assertNotEquals(0, crc, "Hardware CRC32C must return non-zero checksum for non-empty string");
        }

        // 6. Quaternion Packing & Unpacking
        float qx = 0.0f;
        float qy = 0.70710678f;
        float qz = 0.0f;
        float qw = 0.70710678f;
        int packed = bridge.packQuaternion(qx, qy, qz, qw);
        assertNotEquals(0, packed);

        try (Arena arena = Arena.ofConfined()) {
            MemorySegment outQuat = arena.allocate(16, 4);
            bridge.unpackQuaternion(packed, outQuat);
            float ux = outQuat.get(ValueLayout.JAVA_FLOAT, 0);
            float uy = outQuat.get(ValueLayout.JAVA_FLOAT, 4);
            float uz = outQuat.get(ValueLayout.JAVA_FLOAT, 8);
            float uw = outQuat.get(ValueLayout.JAVA_FLOAT, 12);

            assertEquals(qx, ux, 0.01f);
            assertEquals(qy, uy, 0.01f);
            assertEquals(qz, uz, 0.01f);
            assertEquals(qw, uw, 0.01f);
        }
    }

    @Test
    void test128ByteAlignmentEnforcementAndInPlaceSpscInitialization() {
        SymbolLookup lookup = NativeLoader.loadNativeCore();
        assertNotNull(lookup);
        HorizonNativeBridge bridge = new HorizonNativeBridge(lookup);

        long ringSize = bridge.getSpscRingSize();
        assertTrue(ringSize > 0, "SpscRingBuffer size must be positive");

        // 1. Verify Java 21 Project Panama (PanamaMemoryAccessor) 128-byte alignment
        PanamaMemoryAccessor panamaAccessor = new PanamaMemoryAccessor();
        long panamaAddr = panamaAccessor.allocateMemory(ringSize, 128);
        assertNotEquals(0L, panamaAddr);
        assertEquals(0L, panamaAddr % 128L, "PanamaMemoryAccessor must guarantee 128-byte alignment");

        MemorySegment panamaRing = bridge.initSpscInPlace(MemorySegment.ofAddress(panamaAddr), ringSize);
        assertNotEquals(MemorySegment.NULL, panamaRing, "init_in_place must accept 128-byte aligned Panama memory");

        // 2. Verify Java 17 Baseline (Java17MemoryAccessor) 128-byte alignment via upward padding
        Java17MemoryAccessor java17Accessor = new Java17MemoryAccessor();
        long j17Addr = java17Accessor.allocateMemory(ringSize, 128);
        assertNotEquals(0L, j17Addr);
        assertEquals(0L, j17Addr % 128L, "Java17MemoryAccessor must guarantee 128-byte alignment via upward padding");

        MemorySegment j17Ring = bridge.initSpscInPlace(MemorySegment.ofAddress(j17Addr), ringSize);
        assertNotEquals(MemorySegment.NULL, j17Ring, "init_in_place must accept 128-byte aligned Java 17 memory");

        // Push and pop on Java 17 initialized ring
        assertEquals(0, bridge.pushSpscTask(j17Ring, 77, -88, (short) 1, (byte) 3, (byte) 0, 0x11223344L, 512, 10));
        try (Arena arena = Arena.ofConfined()) {
            MemorySegment out = arena.allocate(32, 8);
            assertEquals(0, bridge.popSpscTask(j17Ring, out));
            assertEquals(77, out.get(ValueLayout.JAVA_INT, 0));
            assertEquals(-88, out.get(ValueLayout.JAVA_INT, 4));
        }

        java17Accessor.freeMemory(j17Addr);

        // 3. Intentionally misaligned pointer must be rejected by Rust (returns NULL)
        MemorySegment misaligned = MemorySegment.ofAddress(panamaAddr + 1L);
        MemorySegment rejectedMisaligned = bridge.initSpscInPlace(misaligned, ringSize);
        assertEquals(MemorySegment.NULL, rejectedMisaligned, "init_in_place must reject misaligned pointers");

        // 4. Truncated byte size must be rejected by Rust (returns NULL)
        MemorySegment rejectedTruncated = bridge.initSpscInPlace(MemorySegment.ofAddress(panamaAddr), ringSize - 1L);
        assertEquals(MemorySegment.NULL, rejectedTruncated, "init_in_place must reject buffers smaller than struct size");
    }
}