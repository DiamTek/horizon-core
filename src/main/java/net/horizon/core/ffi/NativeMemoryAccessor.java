package net.horizon.core.ffi;

import java.nio.ByteBuffer;

/**
 * Universal Native Memory FFI Accessor interface.
 * Abstracts the underlying memory dispatch mechanism between:
 * - Java 17 Baseline (Minecraft 1.20.1-1.20.4 via direct off-heap pointers / DirectByteBuffer)
 * - Java 21+ Modern (Minecraft 1.20.5+ / 1.21+ via Panama Foreign Function and Memory API)
 */
public interface NativeMemoryAccessor {

    long allocateMemory(long bytes, long alignment);

    void freeMemory(long address);

    void resetArena(long arenaAddress);

    int getCoreVersionMajor();

    int getCoreVersionMinor();

    int getCoreVersionPatch();

    ByteBuffer asDirectByteBuffer(long address, int capacity);
}