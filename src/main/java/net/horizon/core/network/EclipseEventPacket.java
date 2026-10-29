package net.horizon.core.network;

import java.nio.ByteBuffer;
import java.util.Objects;

/**
 * EclipseEventPacket (ID: 0x06, Server to Client).
 * Broadcasts celestial astronomical eclipse state and atmospheric cooling parameters
 * to synchronize client-side volumetric lighting, HRC ambient probe darkening,
 * and nocturnal entity behavioral triggers.
 *
 * Fields:
 * - eclipseObscuration (float): Fractional solar disc coverage in range [0.0, 1.0].
 * - umbraCoreDistance (float): Distance from player to the center of total darkness umbral cone (meters).
 * - umbraRadiusGround (float): Radius of the 100% totality ground footprint (meters).
 * - penumbraRadiusGround (float): Radius of the partial penumbral ground footprint (meters).
 * - coronaIntensity (float): Solar magnetic field filament radiance multiplier.
 * - atmosphericTempDrop (float): Local temperature drop in Celsius driving mist/condensation.
 * - isTotality (boolean): Whether solar obscuration meets totality threshold (>= 0.92).
 */
public record EclipseEventPacket(
        float eclipseObscuration,
        float umbraCoreDistance,
        float umbraRadiusGround,
        float penumbraRadiusGround,
        float coronaIntensity,
        float atmosphericTempDrop,
        boolean isTotality
) {
    public static final int PACKET_ID = 0x06;
    public static final int PACKET_BYTES = 29;

    public EclipseEventPacket {
        float obs = Float.isNaN(eclipseObscuration) ? 0.0f : Math.max(0.0f, Math.min(1.0f, eclipseObscuration));
        float dist = Float.isNaN(umbraCoreDistance) || Float.isInfinite(umbraCoreDistance) ? 0.0f : Math.max(0.0f, umbraCoreDistance);
        float uRad = Float.isNaN(umbraRadiusGround) || Float.isInfinite(umbraRadiusGround) ? 0.0f : Math.max(0.0f, umbraRadiusGround);
        float pRad = Float.isNaN(penumbraRadiusGround) || Float.isInfinite(penumbraRadiusGround) ? 0.0f : Math.max(0.0f, penumbraRadiusGround);
        float corona = Float.isNaN(coronaIntensity) || Float.isInfinite(coronaIntensity) ? 0.0f : Math.max(0.0f, coronaIntensity);
        float temp = Float.isNaN(atmosphericTempDrop) || Float.isInfinite(atmosphericTempDrop) ? 0.0f : atmosphericTempDrop;
        boolean tot = isTotality || (obs >= 0.92f);

        eclipseObscuration = obs;
        umbraCoreDistance = dist;
        umbraRadiusGround = uRad;
        penumbraRadiusGround = pRad;
        coronaIntensity = corona;
        atmosphericTempDrop = temp;
        isTotality = tot;
    }

    public static EclipseEventPacket of(float obscuration, float umbraDistance, float umbraRadius, float penumbraRadius, float corona, float tempDrop) {
        float obs = Float.isNaN(obscuration) ? 0.0f : Math.max(0.0f, Math.min(1.0f, obscuration));
        boolean totality = obs >= 0.92f;
        return new EclipseEventPacket(obs, umbraDistance, umbraRadius, penumbraRadius, corona, tempDrop, totality);
    }

    public void encode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        buffer.putInt(PACKET_ID);
        buffer.putFloat(eclipseObscuration);
        buffer.putFloat(umbraCoreDistance);
        buffer.putFloat(umbraRadiusGround);
        buffer.putFloat(penumbraRadiusGround);
        buffer.putFloat(coronaIntensity);
        buffer.putFloat(atmosphericTempDrop);
        buffer.put((byte) (isTotality ? 1 : 0));
    }

    public static EclipseEventPacket decode(ByteBuffer buffer) {
        Objects.requireNonNull(buffer, "buffer cannot be null");
        if (buffer.remaining() < PACKET_BYTES) {
            throw new IllegalArgumentException("Insufficient bytes to decode EclipseEventPacket: expected at least "
                    + PACKET_BYTES + ", got " + buffer.remaining());
        }
        int id = buffer.getInt();
        if (id != PACKET_ID) {
            throw new IllegalArgumentException("Invalid packet ID: 0x" + Integer.toHexString(id));
        }
        float obscuration = buffer.getFloat();
        float umbraDist = buffer.getFloat();
        float umbraRadius = buffer.getFloat();
        float penumbraRadius = buffer.getFloat();
        float corona = buffer.getFloat();
        float tempDrop = buffer.getFloat();
        boolean totality = buffer.get() != 0;

        return new EclipseEventPacket(obscuration, umbraDist, umbraRadius, penumbraRadius, corona, tempDrop, totality);
    }
}