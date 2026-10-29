package net.horizon.core.crypto;

import java.io.File;
import java.io.IOException;
import java.io.RandomAccessFile;
import java.nio.channels.FileChannel;
import java.nio.channels.FileLock;
import java.nio.channels.OverlappingFileLockException;
import java.nio.file.Path;
import java.util.Objects;

/**
 * Multi-process advisory lock guard preventing database/region corruption when multiple
 * Minecraft or test instances share the same filesystem cache directory.
 * If the advisory lock is contested, gracefully falls back to memory-only ring buffers.
 */
public final class CacheDirectoryLock implements AutoCloseable {

    private final Path lockFilePath;
    private final RandomAccessFile file;
    private final FileChannel channel;
    private final FileLock lock;
    private final boolean acquired;

    private CacheDirectoryLock(Path lockFilePath, RandomAccessFile file, FileChannel channel, FileLock lock, boolean acquired) {
        this.lockFilePath = lockFilePath;
        this.file = file;
        this.channel = channel;
        this.lock = lock;
        this.acquired = acquired;
    }

    /**
     * Attempts to acquire an exclusive OS advisory lock on the cache directory.
     *
     * @param directory The cache folder to protect.
     * @return CacheDirectoryLock instance indicating whether exclusive write access was secured.
     */
    public static CacheDirectoryLock tryAcquire(File directory) {
        Objects.requireNonNull(directory, "directory cannot be null");
        if (!directory.exists()) {
            directory.mkdirs();
        }

        File lockFile = new File(directory, ".horizon.lock");
        RandomAccessFile raf = null;
        FileChannel channel = null;
        FileLock lock = null;

        try {
            raf = new RandomAccessFile(lockFile, "rw");
            channel = raf.getChannel();
            lock = channel.tryLock();

            if (lock != null && lock.isValid()) {
                return new CacheDirectoryLock(lockFile.toPath(), raf, channel, lock, true);
            }
        } catch (IOException | OverlappingFileLockException ignored) {
            // Contested lock or filesystem failure: fallback to memory-only
        }

        // Cleanup resources on failure to acquire
        if (lock != null) {
            try { lock.release(); } catch (IOException ignored) {}
        }
        if (channel != null) {
            try { channel.close(); } catch (IOException ignored) {}
        }
        if (raf != null) {
            try { raf.close(); } catch (IOException ignored) {}
        }

        return new CacheDirectoryLock(lockFile.toPath(), null, null, null, false);
    }

    /**
     * Whether exclusive file lock was successfully acquired.
     * If false, callers must fall back to process-isolated in-memory cache mode.
     */
    public boolean isAcquired() {
        return acquired;
    }

    public Path getLockFilePath() {
        return lockFilePath;
    }

    @Override
    public void close() {
        if (lock != null && lock.isValid()) {
            try {
                lock.release();
            } catch (IOException ignored) {}
        }
        if (channel != null) {
            try {
                channel.close();
            } catch (IOException ignored) {}
        }
        if (file != null) {
            try {
                file.close();
            } catch (IOException ignored) {}
        }
    }
}