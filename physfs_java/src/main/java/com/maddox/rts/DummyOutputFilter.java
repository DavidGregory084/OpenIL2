package com.maddox.rts;

import java.io.FilterOutputStream;
import java.io.OutputStream;

public class DummyOutputFilter extends FilterOutputStream {
    private static int[] emptyKey = new int[0];

    public DummyOutputFilter(OutputStream out) {
        super(out);
    }

    public DummyOutputFilter(OutputStream out, int[] key) {
        super(out);
    }

    public void kryptoResetSwitch() {}

    public int[] kryptoGetKey() {
        return emptyKey;
    }

    public void kryptoSetKey(int[] var1) {}
}
