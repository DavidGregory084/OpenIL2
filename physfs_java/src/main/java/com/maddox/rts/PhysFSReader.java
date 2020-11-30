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
}
