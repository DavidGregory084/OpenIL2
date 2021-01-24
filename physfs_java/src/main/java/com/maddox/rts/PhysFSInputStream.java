package com.maddox.rts;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.sql.SQLException;
import java.util.Arrays;
import java.util.Locale;
import java.util.Objects;
import java.util.concurrent.atomic.AtomicBoolean;
import java.util.concurrent.atomic.AtomicInteger;

public class PhysFSInputStream extends InputStream {

    private static boolean libLoaded = false;

    private long fd;
    private final String fileName;

    public PhysFSInputStream(String file) {
        this.fd = openRead(file);
        this.fileName = file;
        if (this.fd == 0) {
            var exc = new PhysFSException("while opening file " + file);
            var stackTrace = Thread.currentThread().getStackTrace();
            var stackTraceElems = Arrays.stream(stackTrace);
            if (stackTraceElems.noneMatch(elem -> "com.maddox.rts.ObjIO".equals(elem.getClassName()))) {
                PhysFS.logMissing(file);
            }
            throw exc;
        }
    }

    public PhysFSInputStream(File file) {
        this(file.getPath());
    }

    private native int openRead(String file);

    public void close() {
        if (this.fd != 0) {
            this.close(this.fd);
            this.fd = -1;
        }
    }

    private native int close(long fileDescriptor);

    public boolean endOfFile() {
        if (this.fd != 0) {
            return eof(this.fd) != 0;
        } else {
            return true;
        }
    }

    private native int eof(long fileDescriptor);

    public int read() {
        if (this.fd != 0 && !endOfFile()) {
            byte[] buf = new byte[1];
            int result = read(buf);

            if (result > 0) {
                return buf[0] & 0xFF;
            } else {
                throw new PhysFSException("while reading from file " + this.fileName);
            }
        } else {
            return -1;
        }
    }

    public int read(byte[] buf) {
        return read(buf, 0, buf.length);
    }

    public int read(byte[] buf, int offset, int len) {
        Objects.checkFromIndexSize(offset, len, buf.length);
        if (len == 0) {
            return 0;
        } else if (this.fd != 0) {
            int res = readBytes(this.fd, buf, offset, len);

            if (res > 0) {
                return res;
            } else if (endOfFile()) {
                return -1;
            } else {
                throw new PhysFSException("while reading from file " + this.fileName);
            }
        } else {
            return -1;
        }
    }

    private native int readBytes(long fileDescriptor, byte[] buf, int offset, int len);

    public long fileLength() {
        if (this.fd != 0) {
            return fileLength(this.fd);
        } else {
            return -1;
        }
    }

    private native long fileLength(long fileDescriptor);

    public void seek(long pos) {
        if (this.fd != 0) {
            int res = seek(this.fd, pos);

            if (res != 0) {
                return;
            } else {
                throw new PhysFSException("while seeking in file " + this.fileName);
            }
        } else {
            return;
        }
    }

    ;

    private native int seek(long fileDescriptor, long pos);

    public long tell() {
        if (this.fd != 0) {
            long pos = tell(this.fd);

            if (pos >= 0) {
                return pos;
            } else {
                throw new PhysFSException("while fetching seek position in file " + this.fileName);
            }
        } else {
            return -1;
        }
    }

    private native long tell(long fileDescriptor);

    public int available() {
        if (this.fd != 0) {
            long remaining = this.fileLength() - this.tell();
            long truncated = Math.min(remaining, Integer.MAX_VALUE);
            return (int) truncated;
        } else {
            return 0;
        }
    }

    public long skip(long n) {
        if (this.fd != 0) {
            long currentPos = tell();
            long fileLength = fileLength();
            long desiredPos = currentPos + n;
            long newPos = Math.min(fileLength, desiredPos);
            long skipped = newPos - currentPos;
            seek(this.fd, newPos);
            return skipped;
        } else {
            return -1;
        }
    }

    protected void finalize() {
        if (this.fd != 0) {
            this.close();
        }
    }

    public static void _loadNative() {
        if (!libLoaded) {
            System.loadLibrary("physfs_jni");
            libLoaded = true;
        }
    }

    public static boolean dummyCheck() {
        return false;
    }
}
