package net.horizon.core.network;

import java.nio.ByteBuffer;
import java.util.Objects;

/**
 * WorldSeedHandshakePacket (ID: 0x02, Server to Client).
 * - worldFingerprint (byte[32]): Cryptographic SHA-256 fingerprint.
 * - globalSeed (int64): World generation seed.
 * - seaLevel (int16): Sea level Y-coordinate.
 * - minY / maxY (int16, int16): Dimension vertical bounds.
 * - structureCount (short): Total registered landmark bounding boxes.
 */
public record WorldSeedHandshakePacket(
        byte[] worldFingerprint,
        long globalSeed,
        short seaLevel,
        short minY,
        short maxY,
        short structureCount
) {
    public static final int PACKET_ID = 0x02;
    public static final int PACKET_BYTES = 52;

    public WorldSeedHandshakePacket {
        Objects.requireNonNull(worldFingerprint, "worldFingerprint cannot be null");
        if (worldFingerprint.length != 32) {
            throw new IllegalArgumentException("worldFingerprint must be exactly 32 bytes (SHA-256)");
        }
    }

    public void encode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        buffer.putInt(PACKET_ID);
        buffer.put(worldFingerprint);
        buffer.putLong(globalSeed);
        buffer.putShort(seaLevel);
        buffer.putShort(minY);
        buffer.putShort(maxY);
        buffer.putShort(structureCount);
    }

    public static WorldSeedHandshakePacket decode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        if (buffer.remaining() < PACKET_BYTES) {
            throw new IllegalArgumentException("Insufficient bytes to decode WorldSeedHandshakePacket: expected at least "
                    + PACKET_BYTES + ", got " + buffer.remaining());
        }
        int id = buffer.getInt();
        if (id != PACKET_ID) {
            throw new IllegalArgumentException("Invalid packet ID: 0x" + Integer.toHexString(id));
        }
        byte[] fp = new byte[32];
        buffer.get(fp);
        long seed = buffer.getLong();
        short sea = buffer.getShort();
        short min = buffer.getShort();
        short max = buffer.getShort();
        short structures = buffer.getShort();

        return new WorldSeedHandshakePacket(fp, seed, sea, min, max, structures);
    }
}