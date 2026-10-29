package net.horizon.core.network;

import java.nio.ByteBuffer;
import java.util.Objects;

/**
 * SyncLODChunkPacket (ID: 0x04, Server to Client).
 * Streams LZ4-compressed 64-bit packed vertex arrays to the client.
 */
public record SyncLODChunkPacket(
        int chunkX,
        int chunkZ,
        byte formatVersion,
        short uncompressedSize,
        byte[] payload
) {
    public static final int PACKET_ID = 0x04;
    public static final int HEADER_BYTES = 19;

    public SyncLODChunkPacket {
        Objects.requireNonNull(payload, "payload cannot be null");
    }

    public void encode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        buffer.putInt(PACKET_ID);
        buffer.putInt(chunkX);
        buffer.putInt(chunkZ);
        buffer.put(formatVersion);
        buffer.putShort(uncompressedSize);
        buffer.putInt(payload.length);
        buffer.put(payload);
    }

    public static SyncLODChunkPacket decode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        if (buffer.remaining() < HEADER_BYTES) {
            throw new IllegalArgumentException("Insufficient bytes to decode SyncLODChunkPacket header: expected at least "
                    + HEADER_BYTES + ", got " + buffer.remaining());
        }
        int id = buffer.getInt();
        if (id != PACKET_ID) {
            throw new IllegalArgumentException("Invalid packet ID: 0x" + Integer.toHexString(id));
        }
        int cx = buffer.getInt();
        int cz = buffer.getInt();
        byte ver = buffer.get();
        short uncompSize = buffer.getShort();
        int len = buffer.getInt();
        if (len < 0 || buffer.remaining() < len) {
            throw new IllegalArgumentException("Corrupt payload length in SyncLODChunkPacket: length="
                    + len + ", buffer remaining=" + buffer.remaining());
        }
        byte[] data = new byte[len];
        buffer.get(data);

        return new SyncLODChunkPacket(cx, cz, ver, uncompSize, data);
    }
}