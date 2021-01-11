package com.maddox.rts;

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

    public static native int getLastErrorCode();

    public static void mountArchive(String file) {
        mountArchive(file, false);
    }

    public static void mountArchive(String file, boolean appendToSearchPath) {
        int res = mount(file, appendToSearchPath ? 1 : 0);
        if (res == 0) {
            throw new PhysFSException("while mounting file " + file);
        }
    }

    public static void mountArchive(String file, int appendToSearchPath) {
        int res = mount(file, appendToSearchPath);
        if (res == 0) {
            throw new PhysFSException("while mounting file " + file);
        }
    }

    private static native int mount(String file, int appendToSearchPath);

    public static void mountArchiveAt(String file, String mountPoint) {
        mountArchiveAt(file, mountPoint, false);
    }

    public static void mountArchiveAt(String file, String mountPoint, boolean appendToSearchPath) {
        int res = mountAt(file, mountPoint, appendToSearchPath ? 1 : 0);
        if (res == 0) {
            throw new PhysFSException("while mounting file " + file + " at mount point " + mountPoint);
        }
    }

    public static void mountArchiveAt(String file, String mountPoint, int appendToSearchPath) {
        int res = mountAt(file, mountPoint, appendToSearchPath);
        if (res == 0) {
            throw new PhysFSException("while mounting file " + file + " at mount point " + mountPoint);
        }
    }

    private static native int mountAt(String file, String mountPoint, int appendToSearchPath);

    public static boolean existsFile(String file) {
        return exists(file) != 0;
    }

    private static native int exists(String file);

    public static void unmountArchive(String file) {
        int res = unmount(file);
        if (res == 0) {
            throw new PhysFSException("while unmounting file " + file);
        }
    }

    private static native int unmount(String file);

    private static void loadNative() {
        System.loadLibrary("physfs_jni");
    }

    static {
        loadNative();
    }

    private PhysFS() {
    }
}
