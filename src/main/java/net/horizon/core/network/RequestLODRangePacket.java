package net.horizon.core.network;

import java.nio.ByteBuffer;
import java.util.Objects;

/**
 * RequestLODRangePacket (ID: 0x03, Client to Server).
 * Requests LOD streaming around player center with lookVector for frustum prioritization.
 */
public record RequestLODRangePacket(
        int centerX,
        int centerZ,
        short radius,
        float[] lookVector
) {
    public static final int PACKET_ID = 0x03;
    public static final int PACKET_BYTES = 26;

    public RequestLODRangePacket {
        Objects.requireNonNull(lookVector, "lookVector cannot be null");
        if (lookVector.length != 3) {
            throw new IllegalArgumentException("lookVector must have exactly 3 components (x, y, z)");
        }
        for (int i = 0; i < 3; i++) {
            if (Float.isNaN(lookVector[i]) || Float.isInfinite(lookVector[i])) {
                lookVector[i] = 0.0f;
            }
        }
    }

    public void encode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        buffer.putInt(PACKET_ID);
        buffer.putInt(centerX);
        buffer.putInt(centerZ);
        buffer.putShort(radius);
        buffer.putFloat(lookVector[0]);
        buffer.putFloat(lookVector[1]);
        buffer.putFloat(lookVector[2]);
    }

    public static RequestLODRangePacket decode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        if (buffer.remaining() < PACKET_BYTES) {
            throw new IllegalArgumentException("Insufficient bytes to decode RequestLODRangePacket: expected at least "
                    + PACKET_BYTES + ", got " + buffer.remaining());
        }
        int id = buffer.getInt();
        if (id != PACKET_ID) {
            throw new IllegalArgumentException("Invalid packet ID: 0x" + Integer.toHexString(id));
        }
        int cx = buffer.getInt();
        int cz = buffer.getInt();
        short r = buffer.getShort();
        float[] look = new float[]{buffer.getFloat(), buffer.getFloat(), buffer.getFloat()};

        return new RequestLODRangePacket(cx, cz, r, look);
    }
}