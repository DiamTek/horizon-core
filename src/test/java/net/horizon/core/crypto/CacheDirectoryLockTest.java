package net.horizon.core.crypto;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.io.File;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.*;

class CacheDirectoryLockTest {

    @Test
    void testAcquireAndRelease(@TempDir Path tempDir) {
        File dir = tempDir.toFile();

        try (CacheDirectoryLock lock1 = CacheDirectoryLock.tryAcquire(dir)) {
            assertTrue(lock1.isAcquired(), "First process must acquire the advisory lock");

            // Attempting second acquisition in contested mode
            try (CacheDirectoryLock lock2 = CacheDirectoryLock.tryAcquire(dir)) {
                assertFalse(lock2.isAcquired(), "Second process must fail to acquire and fall back to memory-only mode");
            }
        }

        // After lock1 closes, another acquisition succeeds
        try (CacheDirectoryLock lock3 = CacheDirectoryLock.tryAcquire(dir)) {
            assertTrue(lock3.isAcquired(), "Lock must be re-acquirable once released");
        }
    }
}