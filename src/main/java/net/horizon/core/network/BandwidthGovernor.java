package net.horizon.core.network;

/**
 * Implements the Bandwidth Governor and Hardened Frustum Prioritization function.
 *
 * Prevents zero-division NaNs when the player is inside the target chunk,
 * and uses an angularly weighted positive-definite priority function.
 * Also provides a high-precision Leaky-Bucket Rate Limiter with burst capacity.
 */
public final class BandwidthGovernor {

    private static final float ALPHA = 0.75f;
    private static final float EPSILON = 0.001f;

    public static final long DEFAULT_SUSTAINED_BYTES_PER_SEC = 15L * 1024L; // 15 KB/s
    public static final long DEFAULT_BURST_CAPACITY_BYTES = 30L * 1024L;    // 30 KB burst

    private BandwidthGovernor() {
    }

    /**
     * Calculates the streaming priority for a chunk given player position and look vector.
     * Higher priority chunks are scheduled first.
     *
     * Invariant: Returns non-negative, finite priority; returns 0.0f on invalid or NaN inputs.
     */
    public static float computeChunkPriority(
            float playerX,
            float playerZ,
            float lookX,
            float lookZ,
            float chunkCenterX,
            float chunkCenterZ
    ) {
        if (Float.isNaN(playerX) || Float.isInfinite(playerX)
                || Float.isNaN(playerZ) || Float.isInfinite(playerZ)
                || Float.isNaN(lookX) || Float.isInfinite(lookX)
                || Float.isNaN(lookZ) || Float.isInfinite(lookZ)
                || Float.isNaN(chunkCenterX) || Float.isInfinite(chunkCenterX)
                || Float.isNaN(chunkCenterZ) || Float.isInfinite(chunkCenterZ)) {
            return 0.0f;
        }

        float deltaX = chunkCenterX - playerX;
        float deltaZ = chunkCenterZ - playerZ;
        float dist = (float) Math.sqrt(deltaX * deltaX + deltaZ * deltaZ);
        if (Float.isNaN(dist) || Float.isInfinite(dist)) {
            return 0.0f;
        }

        // Normalize look vector (2D planar)
        float lookLen = (float) Math.sqrt(lookX * lookX + lookZ * lookZ);
        float normLookX = (lookLen > EPSILON && !Float.isNaN(lookLen)) ? lookX / lookLen : 0.0f;
        float normLookZ = (lookLen > EPSILON && !Float.isNaN(lookLen)) ? lookZ / lookLen : 0.0f;

        // Angle weight: max(0.0, V_look dot (Delta / (||Delta|| + epsilon)))
        float normDeltaX = deltaX / (dist + EPSILON);
        float normDeltaZ = deltaZ / (dist + EPSILON);
        float dot = normLookX * normDeltaX + normLookZ * normDeltaZ;
        float angleWeight = Math.max(0.0f, dot);

        // Priority = (alpha * Weight_angle + (1.0 - alpha)) / (||Delta|| + epsilon)
        float priority = (ALPHA * angleWeight + (1.0f - ALPHA)) / (dist + EPSILON);
        if (Float.isNaN(priority) || Float.isInfinite(priority) || priority < 0.0f) {
            return 0.0f;
        }
        return priority;
    }

    /**
     * Leaky-Bucket / Token-Bucket rate limiter enforcing per-client network upload ceilings.
     */
    public static final class LeakyBucket {
        private final long sustainedBytesPerSec;
        private final long maxCapacityBytes;
        private double availableTokens;
        private long lastReplenishNanos;

        public LeakyBucket() {
            this(DEFAULT_SUSTAINED_BYTES_PER_SEC, DEFAULT_BURST_CAPACITY_BYTES);
        }

        public LeakyBucket(long sustainedBytesPerSec, long maxCapacityBytes) {
            if (sustainedBytesPerSec <= 0 || maxCapacityBytes <= 0) {
                throw new IllegalArgumentException("Rates and capacities must be strictly positive");
            }
            this.sustainedBytesPerSec = sustainedBytesPerSec;
            this.maxCapacityBytes = maxCapacityBytes;
            this.availableTokens = maxCapacityBytes; // Start with full burst capacity
            this.lastReplenishNanos = System.nanoTime();
        }

        /**
         * Attempts to consume the specified number of bytes.
         *
         * @param bytes Number of bytes to transfer.
         * @return true if permitted under bandwidth ceiling; false if throttled.
         */
        public synchronized boolean tryConsume(long bytes) {
            if (bytes <= 0) {
                return true;
            }
            replenish();
            if (availableTokens >= bytes) {
                availableTokens -= bytes;
                return true;
            }
            return false;
        }

        /**
         * Returns current available token budget in bytes.
         */
        public synchronized long getAvailableBytes() {
            replenish();
            return (long) availableTokens;
        }

        private void replenish() {
            long now = System.nanoTime();
            long elapsedNanos = now - lastReplenishNanos;
            if (elapsedNanos > 0) {
                double addedTokens = (elapsedNanos / 1_000_000_000.0) * sustainedBytesPerSec;
                availableTokens = Math.min(maxCapacityBytes, availableTokens + addedTokens);
                lastReplenishNanos = now;
            }
        }
    }
}