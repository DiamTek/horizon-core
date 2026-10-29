package net.horizon.core.ffi;

/**
 * Factory and runtime dispatcher for NativeMemoryAccessor.
 * Automatically detects whether the running JVM is Java 17 (Minecraft 1.20.1) or Java 21+ (1.20.5+ / 1.21+),
 * instantiating the optimal zero-overhead FFI bridge.
 */
public final class NativeMemoryDispatcher {

    private static final NativeMemoryAccessor INSTANCE;
    private static final int JAVA_SPEC_VERSION;

    static {
        int version = 17;
        try {
            String v = System.getProperty("java.specification.version");
            if (v != null) {
                version = Integer.parseInt(v);
            }
        } catch (NumberFormatException ignored) {
        }
        JAVA_SPEC_VERSION = version;

        NativeMemoryAccessor chosen = null;
        if (JAVA_SPEC_VERSION >= 21) {
            try {
                // Dynamically attempt Panama modern accessor
                Class<?> clazz = Class.forName("net.horizon.core.ffi.PanamaMemoryAccessor");
                chosen = (NativeMemoryAccessor) clazz.getDeclaredConstructor().newInstance();
            } catch (Throwable ignored) {
                // Fallback to Java 17 accessor if Panama class is absent or restricted
            }
        }

        if (chosen == null) {
            chosen = new Java17MemoryAccessor();
        }
        INSTANCE = chosen;
    }

    public static NativeMemoryAccessor get() {
        return INSTANCE;
    }

    public static int getJavaSpecVersion() {
        return JAVA_SPEC_VERSION;
    }
}