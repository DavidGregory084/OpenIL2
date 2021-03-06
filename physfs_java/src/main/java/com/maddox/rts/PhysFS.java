package com.maddox.rts;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;
import java.util.concurrent.ConcurrentLinkedQueue;
import java.util.concurrent.atomic.AtomicBoolean;

public class PhysFS {
    public static final int ERR_OK = 0;
    public static final int ERR_OTHER_ERROR = 1;
    public static final int ERR_OUT_OF_MEMORY = 2;
    public static final int ERR_NOT_INITIALIZED = 3;
    public static final int ERR_IS_INITIALIZED = 4;
    public static final int ERR_ARGV0_IS_NULL = 5;
    public static final int ERR_UNSUPPORTED = 6;
    public static final int ERR_PAST_EOF = 7;
    public static final int ERR_FILES_STILL_OPEN = 8;
    public static final int ERR_INVALID_ARGUMENT = 9;
    public static final int ERR_NOT_MOUNTED = 10;
    public static final int ERR_NOT_FOUND = 11;
    public static final int ERR_SYMLINK_FORBIDDEN = 12;
    public static final int ERR_NO_WRITE_DIR = 13;
    public static final int ERR_OPEN_FOR_READING = 14;
    public static final int ERR_OPEN_FOR_WRITING = 15;
    public static final int ERR_NOT_A_FILE = 16;
    public static final int ERR_READ_ONLY = 17;
    public static final int ERR_CORRUPT = 18;
    public static final int ERR_SYMLINK_LOOP = 19;
    public static final int ERR_IO = 20;
    public static final int ERR_PERMISSION = 21;
    public static final int ERR_NO_SPACE = 22;
    public static final int ERR_BAD_FILENAME = 23;
    public static final int ERR_BUSY = 24;
    public static final int ERR_DIR_NOT_EMPTY = 25;
    public static final int ERR_OS_ERROR = 26;
    public static final int ERR_DUPLICATE = 27;
    public static final int ERR_BAD_PASSWORD = 28;
    public static final int ERR_APP_CALLBACK = 29;

    static native int init();

    static native int deinit();

    public static native int getLastErrorCode();

    public static void mountArchive(String file) {
        mountArchive(file, false);
    }

    public static void mountArchive(String file, boolean appendToSearchPath) {
        mountArchive(file, appendToSearchPath ? 1 : 0);
    }

    public static void mountArchive(String file, int appendToSearchPath) {
        var zipFile = file.replaceAll("(?i)\\.sfs$", ".zip");
        int res = mount(zipFile, appendToSearchPath);
        if (res == 0) {
            throw new PhysFSException("while mounting file " + zipFile);
        }
    }

    private static native int mount(String file, int appendToSearchPath);

    public static void mountArchiveAt(String file, String mountPoint) {
        mountArchiveAt(file, mountPoint, false);
    }

    public static void mountArchiveAt(String file, String mountPoint, boolean appendToSearchPath) {
        mountArchiveAt(file, mountPoint, appendToSearchPath ? 1 : 0);
    }

    public static void mountArchiveAt(String file, String mountPoint, int appendToSearchPath) {
        var zipFile = file.replaceAll("(?i)\\.sfs$", ".zip");
        int res = mountAt(zipFile, mountPoint, appendToSearchPath);
        if (res == 0) {
            throw new PhysFSException("while mounting file " + zipFile + " at mount point " + mountPoint);
        }
    }

    private static native int mountAt(String file, String mountPoint, int appendToSearchPath);

    public static boolean existsFile(String file) {
        return exists(file) != 0;
    }

    private static native int exists(String file);

    public static void unmountArchive(String file) {
        var zipFile = file.replaceAll("(?i)\\.sfs$", ".zip");
        int res = unmount(zipFile);
        if (res == 0) {
            throw new PhysFSException("while unmounting file " + zipFile);
        }
    }

    private static native int unmount(String file);

    private static AtomicBoolean loggingTerminated = new AtomicBoolean(false);
    private static ConcurrentLinkedQueue<String> logMissingQueue = new ConcurrentLinkedQueue<>();
    private static Thread logMissingThread;

    static void logMissing(String name) {
        logMissingQueue.offer(name);
    }

    static {
        logMissingThread = new Thread(() -> {
            var logFile = Paths.get("missing.log");

            try (var logWriter = Files.newBufferedWriter(
                    logFile,
                    StandardOpenOption.CREATE,
                    StandardOpenOption.WRITE,
                    StandardOpenOption.TRUNCATE_EXISTING
            )) {
                while (!loggingTerminated.get()) {
                    var logEntry = logMissingQueue.poll();
                    if (logEntry != null) {
                        logWriter.write(logEntry);
                        logWriter.newLine();
                    }
                }

                logWriter.flush();

            } catch (IOException exc) {
                System.err.println("Exception when writing to file missing.log");
                exc.printStackTrace(System.err);
            }
        });

        logMissingThread.start();

        Runtime.getRuntime().addShutdownHook(new Thread(() -> {
            loggingTerminated.set(true);
        }));
    }

    private PhysFS() {
    }
}
