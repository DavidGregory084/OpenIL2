package com.maddox.rts;

import java.io.File;
import java.io.InputStreamReader;
import java.io.UnsupportedEncodingException;

public class PhysFSReader extends InputStreamReader {
    public PhysFSReader(String file) {
        super(new PhysFSInputStream(file));
    }

    public PhysFSReader(File file) {
        super(new PhysFSInputStream(file));
    }

    public PhysFSReader(String file, String charset) throws UnsupportedEncodingException {
        super(new PhysFSInputStream(file), charset);
    }

    public PhysFSReader(File file, String charset) throws UnsupportedEncodingException {
        super(new PhysFSInputStream(file), charset);
    }

    public PhysFSReader(String var1, int[] var2)  {
        super(new PhysFSInputStream(var1));
    }

    public PhysFSReader(File var1, int[] var2) {
        super(new PhysFSInputStream(var1));
    }

    public PhysFSReader(String var1, String var2, int[] var3) throws UnsupportedEncodingException {
        super(new PhysFSInputStream(var1), var2);
    }

    public PhysFSReader(File var1, String var2, int[] var3) throws UnsupportedEncodingException {
        super(new PhysFSInputStream(var1), var2);
    }

    public static void _loadNative() {
        System.loadLibrary("physfs_rts");
    }

    static {
        _loadNative();
    }
}
