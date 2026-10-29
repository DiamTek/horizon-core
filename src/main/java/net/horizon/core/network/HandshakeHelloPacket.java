package net.horizon.core.network;

import java.nio.ByteBuffer;
import java.util.Objects;

/**
 * HandshakeHelloPacket (ID: 0x01, Client to Server).
 * Initiates connection and negotiates protocol capabilities.
 */
public record HandshakeHelloPacket(
        int protocolVersion,
        byte clientCapabilities,
        short viewDistancePreference
) {
    public static final int PACKET_ID = 0x01;
    public static final int MIN_PACKET_BYTES = 11;

    public void encode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        buffer.putInt(PACKET_ID);
        buffer.putInt(protocolVersion);
        buffer.put(clientCapabilities);
        buffer.putShort(viewDistancePreference);
    }

    public static HandshakeHelloPacket decode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        if (buffer.remaining() < MIN_PACKET_BYTES) {
            throw new IllegalArgumentException("Insufficient bytes to decode HandshakeHelloPacket: expected at least "
                    + MIN_PACKET_BYTES + ", got " + buffer.remaining());
        }
        int id = buffer.getInt();
        if (id != PACKET_ID) {
            throw new IllegalArgumentException("Invalid packet ID: 0x" + Integer.toHexString(id));
        }
        int version = buffer.getInt();
        byte caps = buffer.get();
        short dist = buffer.getShort();
        return new HandshakeHelloPacket(version, caps, dist);
    }
}