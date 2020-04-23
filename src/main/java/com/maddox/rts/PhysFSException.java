package com.maddox.rts;

public class PhysFSException extends RuntimeException {
    private final int code;

    public int getCode() {
        return code;
    }

    public PhysFSException(int code) {
        super(getErrorMessage(code));
        this.code = code;
    }

    public static String getErrorMessage(int code) {
        switch (code) {
            case PhysFS.ERR_OK:
                return "OK";
            case PhysFS.ERR_OTHER_ERROR:
                return "Other Error";
            case PhysFS.ERR_OUT_OF_MEMORY:
                return "Out Of Memory";
            case PhysFS.ERR_NOT_INITIALIZED:
                return "Not Initialized";
            case PhysFS.ERR_IS_INITIALIZED:
                return "Is Initialized";
            case PhysFS.ERR_ARGV0_IS_NULL:
                return "Argv0 Is Null";
            case PhysFS.ERR_UNSUPPORTED:
                return "Unsupported";
            case PhysFS.ERR_PAST_EOF:
                return "Past EOF";
            case PhysFS.ERR_FILES_STILL_OPEN:
                return "Files Still Open";
            case PhysFS.ERR_INVALID_ARGUMENT:
                return "Invalid Argument";
            case PhysFS.ERR_NOT_MOUNTED:
                return "Not Mounted";
            case PhysFS.ERR_NOT_FOUND:
                return "Not Found";
            case PhysFS.ERR_SYMLINK_FORBIDDEN:
                return "Symlink Forbidden";
            case PhysFS.ERR_NO_WRITE_DIR:
                return "No Write Directory";
            case PhysFS.ERR_OPEN_FOR_READING:
                return "Open For Reading";
            case PhysFS.ERR_OPEN_FOR_WRITING:
                return "Open For Writing";
            case PhysFS.ERR_NOT_A_FILE:
                return "Not A File";
            case PhysFS.ERR_READ_ONLY:
                return "Read Only";
            case PhysFS.ERR_CORRUPT:
                return "Corrupt";
            case PhysFS.ERR_SYMLINK_LOOP:
                return "Symlink Loop";
            case PhysFS.ERR_IO:
                return "I/O Error";
            case PhysFS.ERR_PERMISSION:
                return "Permission Error";
            case PhysFS.ERR_NO_SPACE:
                return "No Disk Space";
            case PhysFS.ERR_BAD_FILENAME:
                return "Bad Filename";
            case PhysFS.ERR_BUSY:
                return "Busy";
            case PhysFS.ERR_DIR_NOT_EMPTY:
                return "Directory Not Empty";
            case PhysFS.ERR_OS_ERROR:
                return "OS Error";
            case PhysFS.ERR_DUPLICATE:
                return "Duplicate";
            case PhysFS.ERR_BAD_PASSWORD:
                return "Bad Password";
            case PhysFS.ERR_APP_CALLBACK:
                return "App Callback";
            default:
                return "Unknown Error Code";
        }
    }
}
