package net.horizon.core.crypto;

import java.nio.charset.StandardCharsets;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Objects;

/**
 * Enforces Cryptographic World Fingerprinting to isolate cache paths
 * and guarantee session immunity across LAN, P2P (Essential, e4mc), and dedicated servers.
 */
public final class WorldFingerprint {

    private final String hashHex;
    private final byte[] rawBytes;

    private WorldFingerprint(String hashHex, byte[] rawBytes) {
        this.hashHex = hashHex;
        this.rawBytes = rawBytes;
    }

    /**
     * Standard computation using known server seed, dimension identifier, and generator settings.
     */
    public static WorldFingerprint compute(long serverSeed, String dimensionIdentifier, String generatorSettings) {
        Objects.requireNonNull(dimensionIdentifier, "dimensionIdentifier cannot be null");
        String settings = generatorSettings != null ? generatorSettings : "";
        String payload = serverSeed + ":" + dimensionIdentifier + ":" + settings;
        return fromPayload(payload);
    }

    /**
     * Fallback computation when server seed is hidden on public servers (Hypixel, SMPs, velocity proxies).
     * FingerprintFallback = SHA256(ServerSocketAddress + DimensionTypeID + InitialSpawnCoordinates)
     */
    public static WorldFingerprint computeFallback(String serverSocketAddress, String dimensionTypeId, int spawnX, int spawnY, int spawnZ) {
        Objects.requireNonNull(serverSocketAddress, "serverSocketAddress cannot be null");
        Objects.requireNonNull(dimensionTypeId, "dimensionTypeId cannot be null");
        String payload = serverSocketAddress + ":" + dimensionTypeId + ":" + spawnX + "," + spawnY + "," + spawnZ;
        return fromPayload(payload);
    }

    /**
     * Reconstructs a WorldFingerprint from existing 32 raw bytes (e.g., from network packet).
     */
    public static WorldFingerprint fromBytes(byte[] bytes) {
        Objects.requireNonNull(bytes, "bytes cannot be null");
        if (bytes.length != 32) {
            throw new IllegalArgumentException("WorldFingerprint requires exactly 32 bytes, got " + bytes.length);
        }
        byte[] copy = bytes.clone();
        StringBuilder sb = new StringBuilder(64);
        for (byte b : copy) {
            sb.append(String.format("%02x", b));
        }
        return new WorldFingerprint(sb.toString(), copy);
    }

    private static WorldFingerprint fromPayload(String payload) {
        try {
            MessageDigest digest = MessageDigest.getInstance("SHA-256");
            byte[] hashBytes = digest.digest(payload.getBytes(StandardCharsets.UTF_8));
            StringBuilder sb = new StringBuilder(hashBytes.length * 2);
            for (byte b : hashBytes) {
                sb.append(String.format("%02x", b));
            }
            return new WorldFingerprint(sb.toString(), hashBytes);
        } catch (NoSuchAlgorithmException e) {
            throw new IllegalStateException("SHA-256 algorithm missing from JVM", e);
        }
    }

    public String getHex() {
        return hashHex;
    }

    public byte[] getRawBytes() {
        return rawBytes.clone();
    }

    public String getCacheDirectoryName() {
        return "horizon_cache_" + hashHex.substring(0, 16);
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof WorldFingerprint that)) return false;
        return Objects.equals(hashHex, that.hashHex);
    }

    @Override
    public int hashCode() {
        return Objects.hash(hashHex);
    }

    @Override
    public String toString() {
        return "WorldFingerprint[" + hashHex + "]";
    }
}