package com.maddox.rts;

import java.io.FilterInputStream;
import java.io.InputStream;

public class DummyInputFilter extends FilterInputStream {
    private static int[] emptyKey = new int[0];

    protected DummyInputFilter(InputStream in) {
        super(in);
    }

    public DummyInputFilter(InputStream in, int[] key) {
        super(in);
    }

    public void kryptoResetSwitch() {}

    public int[] kryptoGetKey() {
        return emptyKey;
    }

    public void kryptoSetKey(int[] var1) {}

    public boolean markSupported() {
        return false;
    }
}
