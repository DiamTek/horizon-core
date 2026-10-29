package net.horizon.core.ffi;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class NativeMemoryDispatcherTest {

    @Test
    void testDispatcherProvidesNonNullAccessor() {
        NativeMemoryAccessor accessor = NativeMemoryDispatcher.get();
        assertNotNull(accessor);
        assertTrue(NativeMemoryDispatcher.getJavaSpecVersion() >= 17);
    }

    @Test
    void testJava17BaselineAllocationAndFree() {
        Java17MemoryAccessor accessor = new Java17MemoryAccessor();
        long addr = accessor.allocateMemory(1024, 16);
        assertNotEquals(0, addr);
        assertEquals(0, addr % 16, "Address must satisfy 16-byte alignment");
        accessor.freeMemory(addr);
    }

    @Test
    void testJava17InvalidAllocationThrows() {
        Java17MemoryAccessor accessor = new Java17MemoryAccessor();
        assertThrows(IllegalArgumentException.class, () -> accessor.allocateMemory(0, 16));
        assertThrows(IllegalArgumentException.class, () -> accessor.allocateMemory(-10, 16));
        assertThrows(IllegalArgumentException.class, () -> accessor.allocateMemory(1024, 0));
        assertThrows(IllegalArgumentException.class, () -> accessor.allocateMemory(1024, 15)); // Non-power-of-two
    }

    @Test
    void testJava17DirectByteBufferCreation() {
        Java17MemoryAccessor accessor = new Java17MemoryAccessor();
        long addr = accessor.allocateMemory(512, 64);
        assertEquals(0, addr % 64);

        java.nio.ByteBuffer buffer = accessor.asDirectByteBuffer(addr, 512);
        assertNotNull(buffer);
        assertTrue(buffer.isDirect());
        assertEquals(512, buffer.capacity());

        accessor.freeMemory(addr);
    }

    @Test
    void testCoreVersionMetadata() {
        NativeMemoryAccessor accessor = NativeMemoryDispatcher.get();
        assertEquals(0, accessor.getCoreVersionMajor());
        assertEquals(1, accessor.getCoreVersionMinor());
        assertEquals(0, accessor.getCoreVersionPatch());
    }
}