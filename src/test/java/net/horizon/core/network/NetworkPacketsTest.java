package net.horizon.core.network;

import org.junit.jupiter.api.Test;
import java.nio.ByteBuffer;
import static org.junit.jupiter.api.Assertions.*;

class NetworkPacketsTest {

    @Test
    void testHandshakeHelloRoundtrip() {
        HandshakeHelloPacket original = new HandshakeHelloPacket(1, (byte) 0x07, (short) 64);
        ByteBuffer buffer = ByteBuffer.allocate(64);
        original.encode(buffer);
        buffer.flip();

        HandshakeHelloPacket decoded = HandshakeHelloPacket.decode(buffer);
        assertEquals(original, decoded);
    }

    @Test
    void testWorldSeedHandshakeRoundtrip() {
        byte[] fp = new byte[32];
        for (int i = 0; i < 32; i++) fp[i] = (byte) i;

        WorldSeedHandshakePacket original = new WorldSeedHandshakePacket(
                fp,
                9876543210L,
                (short) 63,
                (short) -64,
                (short) 320,
                (short) 128
        );
        ByteBuffer buffer = ByteBuffer.allocate(256);
        original.encode(buffer);
        buffer.flip();

        WorldSeedHandshakePacket decoded = WorldSeedHandshakePacket.decode(buffer);
        assertArrayEquals(original.worldFingerprint(), decoded.worldFingerprint());
        assertEquals(original.globalSeed(), decoded.globalSeed());
        assertEquals(original.seaLevel(), decoded.seaLevel());
        assertEquals(original.minY(), decoded.minY());
        assertEquals(original.maxY(), decoded.maxY());
        assertEquals(original.structureCount(), decoded.structureCount());
    }

    @Test
    void testRequestLODRangeRoundtrip() {
        RequestLODRangePacket original = new RequestLODRangePacket(10, -5, (short) 32, new float[]{0.0f, 0.0f, 1.0f});
        ByteBuffer buffer = ByteBuffer.allocate(64);
        original.encode(buffer);
        buffer.flip();

        RequestLODRangePacket decoded = RequestLODRangePacket.decode(buffer);
        assertEquals(original.centerX(), decoded.centerX());
        assertEquals(original.centerZ(), decoded.centerZ());
        assertEquals(original.radius(), decoded.radius());
        assertArrayEquals(original.lookVector(), decoded.lookVector());
    }

    @Test
    void testSyncLODChunkRoundtrip() {
        byte[] payload = new byte[]{1, 2, 3, 4, 5, 6, 7, 8};
        SyncLODChunkPacket original = new SyncLODChunkPacket(10, -5, (byte) 1, (short) 16, payload);

        ByteBuffer buffer = ByteBuffer.allocate(128);
        original.encode(buffer);
        buffer.flip();

        SyncLODChunkPacket decoded = SyncLODChunkPacket.decode(buffer);
        assertEquals(original.chunkX(), decoded.chunkX());
        assertEquals(original.chunkZ(), decoded.chunkZ());
        assertEquals(original.formatVersion(), decoded.formatVersion());
        assertEquals(original.uncompressedSize(), decoded.uncompressedSize());
        assertArrayEquals(original.payload(), decoded.payload());
    }

    @Test
    void testLODDeltaRoundtrip() {
        byte[] mods = new byte[]{10, 20, 30};
        LODDeltaPacket original = new LODDeltaPacket(3, -8, 0xFFFF0000L, mods);

        ByteBuffer buffer = ByteBuffer.allocate(128);
        original.encode(buffer);
        buffer.flip();

        LODDeltaPacket decoded = LODDeltaPacket.decode(buffer);
        assertEquals(original.chunkX(), decoded.chunkX());
        assertEquals(original.chunkZ(), decoded.chunkZ());
        assertEquals(original.dirtyColumnMask(), decoded.dirtyColumnMask());
        assertArrayEquals(original.surfaceModifications(), decoded.surfaceModifications());
    }

    @Test
    void testEclipseEventRoundtrip() {
        EclipseEventPacket original = EclipseEventPacket.of(0.96f, 1500.0f, 250.0f, 1200.0f, 45.0f, -4.5f);
        assertTrue(original.isTotality());

        ByteBuffer buffer = ByteBuffer.allocate(64);
        original.encode(buffer);
        buffer.flip();

        EclipseEventPacket decoded = EclipseEventPacket.decode(buffer);
        assertEquals(original.eclipseObscuration(), decoded.eclipseObscuration());
        assertEquals(original.umbraCoreDistance(), decoded.umbraCoreDistance());
        assertEquals(original.umbraRadiusGround(), decoded.umbraRadiusGround());
        assertEquals(original.penumbraRadiusGround(), decoded.penumbraRadiusGround());
        assertEquals(original.coronaIntensity(), decoded.coronaIntensity());
        assertEquals(original.atmosphericTempDrop(), decoded.atmosphericTempDrop());
        assertEquals(original.isTotality(), decoded.isTotality());
    }

    @Test
    void testBandwidthGovernorPriority() {
        // Player standing at (0, 0) looking towards +Z (0, 1)
        float prioInFront = BandwidthGovernor.computeChunkPriority(0, 0, 0, 1, 0, 16);
        float prioBehind = BandwidthGovernor.computeChunkPriority(0, 0, 0, 1, 0, -16);
        float prioInside = BandwidthGovernor.computeChunkPriority(0, 0, 0, 1, 0, 0);

        assertTrue(prioInFront > prioBehind, "Chunk directly in front must have higher priority than chunk behind");
        assertTrue(prioInside > prioInFront, "Chunk directly inside player radius must have maximum priority");
        assertFalse(Float.isNaN(prioInside), "Zero distance must never produce NaN");
        assertFalse(Float.isInfinite(prioInside), "Zero distance must never produce Infinity");
    }

    @Test
    void testBandwidthGovernorLeakyBucketThrottling() {
        BandwidthGovernor.LeakyBucket bucket = new BandwidthGovernor.LeakyBucket(15 * 1024, 30 * 1024);
        assertEquals(30 * 1024, bucket.getAvailableBytes());

        // First consume within burst capacity
        assertTrue(bucket.tryConsume(20 * 1024));
        assertEquals(10 * 1024, bucket.getAvailableBytes());

        // Consume remaining burst
        assertTrue(bucket.tryConsume(10 * 1024));

        // Attempting to consume beyond empty bucket must be throttled
        assertFalse(bucket.tryConsume(5 * 1024));

        // Consuming 0 bytes always succeeds
        assertTrue(bucket.tryConsume(0));
    }

    @Test
    void testBandwidthGovernorNaNSafety() {
        assertEquals(0.0f, BandwidthGovernor.computeChunkPriority(Float.NaN, 0, 0, 1, 0, 16));
        assertEquals(0.0f, BandwidthGovernor.computeChunkPriority(0, Float.POSITIVE_INFINITY, 0, 1, 0, 16));
        assertEquals(0.0f, BandwidthGovernor.computeChunkPriority(0, 0, Float.NaN, 1, 0, 16));
        assertEquals(0.0f, BandwidthGovernor.computeChunkPriority(0, 0, 0, 1, Float.NaN, 16));
    }

    @Test
    void testPacketDecodeUnderflowHandling() {
        ByteBuffer emptyBuffer = ByteBuffer.allocate(4);

        assertThrows(IllegalArgumentException.class, () -> HandshakeHelloPacket.decode(emptyBuffer));
        assertThrows(IllegalArgumentException.class, () -> WorldSeedHandshakePacket.decode(emptyBuffer));
        assertThrows(IllegalArgumentException.class, () -> RequestLODRangePacket.decode(emptyBuffer));
        assertThrows(IllegalArgumentException.class, () -> SyncLODChunkPacket.decode(emptyBuffer));
        assertThrows(IllegalArgumentException.class, () -> LODDeltaPacket.decode(emptyBuffer));
        assertThrows(IllegalArgumentException.class, () -> EclipseEventPacket.decode(emptyBuffer));
    }

    @Test
    void testPacketInvalidIdHandling() {
        ByteBuffer badIdBuffer = ByteBuffer.allocate(64);
        badIdBuffer.putInt(0x9999);
        badIdBuffer.putInt(1);
        badIdBuffer.put((byte) 0);
        badIdBuffer.putShort((short) 10);
        badIdBuffer.flip();

        assertThrows(IllegalArgumentException.class, () -> HandshakeHelloPacket.decode(badIdBuffer));
    }

    @Test
    void testEclipseEventSanitization() {
        EclipseEventPacket sanitized = EclipseEventPacket.of(Float.NaN, -100.0f, -50.0f, -20.0f, Float.NaN, Float.NaN);
        assertEquals(0.0f, sanitized.eclipseObscuration());
        assertEquals(0.0f, sanitized.umbraCoreDistance());
        assertEquals(0.0f, sanitized.umbraRadiusGround());
        assertEquals(0.0f, sanitized.penumbraRadiusGround());
        assertEquals(0.0f, sanitized.coronaIntensity());
        assertEquals(0.0f, sanitized.atmosphericTempDrop());
        assertFalse(sanitized.isTotality());

        EclipseEventPacket totality = EclipseEventPacket.of(1.5f, 0.0f, 100.0f, 200.0f, 10.0f, -5.0f);
        assertEquals(1.0f, totality.eclipseObscuration());
        assertTrue(totality.isTotality());
    }
}