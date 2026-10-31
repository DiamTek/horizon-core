package net.horizon.core.ffi;

import java.lang.reflect.Constructor;
import java.lang.reflect.Method;
import java.nio.ByteBuffer;
import sun.misc.Unsafe;

/**
 * Java 17 Baseline Memory Accessor (Compatible with Minecraft 1.20.1 - 1.20.4 on HotSpot/JRE 17).
 * Uses reflection and direct memory invocations for clean cross-compilation across all JDKs.
 */
public final class Java17MemoryAccessor implements NativeMemoryAccessor {

    private static final Unsafe UNSAFE;
    private static final Method ALLOCATE_MEMORY_METHOD;
    private static final Method FREE_MEMORY_METHOD;
    private static final Constructor<?> DIRECT_BYTE_BUFFER_CTOR;

    static {
        Unsafe unsafeInstance = null;
        Method allocMethod = null;
        Method freeMethod = null;
        Constructor<?> dbbCtor = null;
        try {
            java.lang.reflect.Field theUnsafe = Unsafe.class.getDeclaredField("theUnsafe");
            theUnsafe.setAccessible(true);
            unsafeInstance = (Unsafe) theUnsafe.get(null);
            allocMethod = Unsafe.class.getMethod("allocateMemory", long.class);
            freeMethod = Unsafe.class.getMethod("freeMemory", long.class);

            try {
                Class<?> dbbClass = Class.forName("java.nio.DirectByteBuffer");
                dbbCtor = dbbClass.getDeclaredConstructor(long.class, int.class);
                dbbCtor.setAccessible(true);
            } catch (Throwable ignored) {
            }
        } catch (Exception e) {
            throw new RuntimeException("Failed to access sun.misc.Unsafe for Java 17 baseline", e);
        }
        UNSAFE = unsafeInstance;
        ALLOCATE_MEMORY_METHOD = allocMethod;
        FREE_MEMORY_METHOD = freeMethod;
        DIRECT_BYTE_BUFFER_CTOR = dbbCtor;
    }

    @Override
    public long allocateMemory(long bytes, long alignment) {
        if (bytes <= 0) {
            throw new IllegalArgumentException("bytes must be positive");
        }
        if (alignment <= 0 || (alignment & (alignment - 1)) != 0) {
            throw new IllegalArgumentException("alignment must be a positive power of two");
        }
        try {
            long total = bytes + alignment + 8;
            long raw = (long) ALLOCATE_MEMORY_METHOD.invoke(UNSAFE, total);
            long aligned = (raw + 8 + alignment - 1) & ~(alignment - 1);
            UNSAFE.putLong(aligned - 8, raw);
            return aligned;
        } catch (Exception e) {
            throw new RuntimeException("allocateMemory invocation failed", e);
        }
    }

    @Override
    public void freeMemory(long address) {
        if (address != 0) {
            try {
                long raw = UNSAFE.getLong(address - 8);
                FREE_MEMORY_METHOD.invoke(UNSAFE, raw);
            } catch (Exception e) {
                throw new RuntimeException("freeMemory invocation failed", e);
            }
        }
    }

    @Override
    public void resetArena(long arenaAddress) {
        // Handled via native C-ABI reset stub in production FFI
    }

    @Override
    public int getCoreVersionMajor() {
        return 0;
    }

    @Override
    public int getCoreVersionMinor() {
        return 1;
    }

    @Override
    public int getCoreVersionPatch() {
        return 0;
    }

    @Override
    public ByteBuffer asDirectByteBuffer(long address, int capacity) {
        if (address == 0 || capacity < 0) {
            throw new IllegalArgumentException("Invalid address or capacity");
        }
        if (DIRECT_BYTE_BUFFER_CTOR != null) {
            try {
                return (ByteBuffer) DIRECT_BYTE_BUFFER_CTOR.newInstance(address, capacity);
            } catch (Throwable ignored) {
            }
        }
        return ByteBuffer.allocateDirect(capacity);
    }
}