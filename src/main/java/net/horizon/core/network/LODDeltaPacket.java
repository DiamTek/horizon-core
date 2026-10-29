package net.horizon.core.network;

import java.nio.ByteBuffer;
import java.util.Objects;

/**
 * LODDeltaPacket (ID: 0x05, Server to Client).
 * Streams differential palette and column modifications when a distant chunk is modified.
 */
public record LODDeltaPacket(
        int chunkX,
        int chunkZ,
        long dirtyColumnMask,
        byte[] surfaceModifications
) {
    public static final int PACKET_ID = 0x05;
    public static final int HEADER_BYTES = 24;

    public LODDeltaPacket {
        Objects.requireNonNull(surfaceModifications, "surfaceModifications cannot be null");
    }

    public void encode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        buffer.putInt(PACKET_ID);
        buffer.putInt(chunkX);
        buffer.putInt(chunkZ);
        buffer.putLong(dirtyColumnMask);
        buffer.putInt(surfaceModifications.length);
        buffer.put(surfaceModifications);
    }

    public static LODDeltaPacket decode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        if (buffer.remaining() < HEADER_BYTES) {
            throw new IllegalArgumentException("Insufficient bytes to decode LODDeltaPacket header: expected at least "
                    + HEADER_BYTES + ", got " + buffer.remaining());
        }
        int id = buffer.getInt();
        if (id != PACKET_ID) {
            throw new IllegalArgumentException("Invalid packet ID: 0x" + Integer.toHexString(id));
        }
        int cx = buffer.getInt();
        int cz = buffer.getInt();
        long mask = buffer.getLong();
        int len = buffer.getInt();
        if (len < 0 || buffer.remaining() < len) {
            throw new IllegalArgumentException("Corrupt modifications length in LODDeltaPacket: length="
                    + len + ", buffer remaining=" + buffer.remaining());
        }
        byte[] mods = new byte[len];
        buffer.get(mods);

        return new LODDeltaPacket(cx, cz, mask, mods);
    }
}