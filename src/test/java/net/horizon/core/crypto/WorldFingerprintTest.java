package net.horizon.core.crypto;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class WorldFingerprintTest {

    @Test
    void testDeterministicFingerprint() {
        WorldFingerprint fp1 = WorldFingerprint.compute(123456789L, "minecraft:overworld", "default");
        WorldFingerprint fp2 = WorldFingerprint.compute(123456789L, "minecraft:overworld", "default");
        assertEquals(fp1, fp2);
        assertEquals(fp1.getHex(), fp2.getHex());
        assertEquals(32, fp1.getRawBytes().length);
    }

    @Test
    void testDifferentSeedsProduceDifferentFingerprints() {
        WorldFingerprint fp1 = WorldFingerprint.compute(123456789L, "minecraft:overworld", "default");
        WorldFingerprint fp2 = WorldFingerprint.compute(987654321L, "minecraft:overworld", "default");
        assertNotEquals(fp1, fp2);
        assertNotEquals(fp1.getHex(), fp2.getHex());
    }

    @Test
    void testCacheDirectoryNameFormat() {
        WorldFingerprint fp = WorldFingerprint.compute(42L, "minecraft:the_nether", "");
        assertTrue(fp.getCacheDirectoryName().startsWith("horizon_cache_"));
        assertEquals(30, fp.getCacheDirectoryName().length()); // "horizon_cache_" (14) + 16 hex chars
    }

    @Test
    void testFallbackFingerprint() {
        WorldFingerprint fp1 = WorldFingerprint.computeFallback("play.hypixel.net:25565", "minecraft:overworld", 0, 64, 0);
        WorldFingerprint fp2 = WorldFingerprint.computeFallback("play.hypixel.net:25565", "minecraft:overworld", 0, 64, 0);
        WorldFingerprint fp3 = WorldFingerprint.computeFallback("smp.example.com:25565", "minecraft:overworld", 0, 64, 0);

        assertEquals(fp1, fp2);
        assertNotEquals(fp1, fp3);
        assertEquals(32, fp1.getRawBytes().length);
    }

    @Test
    void testFromBytes() {
        WorldFingerprint original = WorldFingerprint.compute(9999L, "minecraft:the_end", "amplified");
        byte[] bytes = original.getRawBytes();
        WorldFingerprint reconstructed = WorldFingerprint.fromBytes(bytes);

        assertEquals(original, reconstructed);
        assertEquals(original.getHex(), reconstructed.getHex());
    }
}