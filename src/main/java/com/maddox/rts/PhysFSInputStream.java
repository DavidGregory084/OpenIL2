package com.maddox.rts;

import java.io.InputStream;

public class PhysFSInputStream extends InputStream {
    int fd;

    public PhysFSInputStream(String file) {
        this.fd = openRead(file);
    }

    @Override
    public int read() {
        byte[] buf = new byte[1];
        int result = read(buf);

        if (result > 0) {
            return buf[0];
        } else {
            int lastError = PhysFS.getLastErrorCode();

            if (lastError == PhysFS.ERR_OK) {
                return -1;
            } else {
                throw new PhysFSException(lastError);
            }
        }
    }

    @Override
    public int available() {
        long remaining = this.fileLength(this.fd) - this.tell(this.fd);
        long truncated = Math.min(remaining, Integer.MAX_VALUE);
        return (int) truncated;
    }

    @Override
    public long skip(long n) {
        long currentPos = tell(this.fd);
        long fileLength = fileLength(this.fd);
        long desiredPos = currentPos + n;
        long newPos = Math.min(fileLength, desiredPos);
        long skipped = newPos - currentPos;
        seek(this.fd, newPos);
        return skipped;
    }

    @Override
    public int read(byte[] b) {
        return readBytes(this.fd, b, b.length);
    }

    @Override
    public void close() {
        if (this.fd != -1) {
            this.close(this.fd);
            this.fd = -1;
        }
    }

    private native long fileLength(int fileDescriptor);

    private native long tell(int fileDescriptor);

    private native int readBytes(int fileDescriptor, byte[] buf, int len);

    private native int openRead(String file);

    private native int seek(int fileDescriptor, long pos);

    private native int close(int fileDescriptor);

    protected void finalize() {
        if (this.fd != -1) {
            this.close();
        }
    }

    private static void loadNative() {
        System.loadLibrary("physfs_jni");
    }

    static {
        loadNative();
    }
}
