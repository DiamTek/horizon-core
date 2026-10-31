package net.horizon.core.ffi;

import java.io.File;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.lang.foreign.Arena;
import java.lang.foreign.SymbolLookup;
import java.nio.file.Files;
import java.util.Locale;

/**
 * Robust Cross-Platform Native Library Extractor and Loader.
 * Unpacks the platform-appropriate native binary (.dll on Windows, .so on Linux, .dylib on macOS)
 * from JAR resources or local build artifacts into temporary off-heap storage and links SymbolLookup.
 */
public final class NativeLoader {

    private static volatile boolean loaded = false;
    private static volatile SymbolLookup symbolLookup = null;

    private NativeLoader() {
    }

    public static synchronized SymbolLookup loadNativeCore() {
        if (loaded && symbolLookup != null) {
            return symbolLookup;
        }

        String os = System.getProperty("os.name", "").toLowerCase(Locale.ROOT);
        String arch = System.getProperty("os.arch", "").toLowerCase(Locale.ROOT);

        String libName;
        String extension;

        if (os.contains("win")) {
            libName = "horizon_core_native";
            extension = ".dll";
        } else if (os.contains("mac") || os.contains("darwin")) {
            libName = "libhorizon_core_native";
            extension = ".dylib";
        } else {
            libName = "libhorizon_core_native";
            extension = ".so";
        }

        File targetFile = null;

        // 1. Check custom path or standard build output directories
        String customPath = System.getProperty("horizon.native.path");
        if (customPath != null && new File(customPath).exists()) {
            targetFile = new File(customPath);
        } else {
            String[] searchPaths = new String[]{
                    "target/debug/" + libName + extension,
                    "target/release/" + libName + extension,
                    "crates/horizon-core-native/target/debug/" + libName + extension,
                    "crates/horizon-core-native/target/release/" + libName + extension,
                    "horizon-core/target/debug/" + libName + extension,
                    "horizon-core/target/release/" + libName + extension,
                    "../target/debug/" + libName + extension,
                    "../target/release/" + libName + extension
            };
            for (String p : searchPaths) {
                File f = new File(p);
                if (f.exists()) {
                    targetFile = f.getAbsoluteFile();
                    break;
                }
            }
        }

        if (targetFile == null) {
            // 2. Extract from embedded JAR resources
            String resourcePath = "/natives/" + (arch.contains("aarch64") || arch.contains("arm64") ? "arm64/" : "x64/") + libName + extension;
            try (InputStream in = NativeLoader.class.getResourceAsStream(resourcePath)) {
                if (in != null) {
                    File temp = Files.createTempFile("horizon_core_native_", extension).toFile();
                    temp.deleteOnExit();
                    try (FileOutputStream out = new FileOutputStream(temp)) {
                        in.transferTo(out);
                    }
                    targetFile = temp;
                }
            } catch (Exception ignored) {
            }
        }

        if (targetFile != null && targetFile.exists()) {
            try {
                if (NativeMemoryDispatcher.getJavaSpecVersion() >= 21) {
                    symbolLookup = SymbolLookup.libraryLookup(targetFile.toPath(), Arena.global());
                } else {
                    System.load(targetFile.getAbsolutePath());
                    symbolLookup = SymbolLookup.loaderLookup();
                }
                loaded = true;
                return symbolLookup;
            } catch (Throwable t) {
                // Fallback to loaderLookup
            }
        }

        symbolLookup = SymbolLookup.loaderLookup();
        loaded = true;
        return symbolLookup;
    }

    public static boolean isLoaded() {
        return loaded;
    }
}