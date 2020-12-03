package com.maddox.instrument;

import java.lang.instrument.Instrumentation;
import java.security.NoSuchAlgorithmException;

public class ClassLoadAgent {
    public static void premain(String agentArgs, Instrumentation inst) throws NoSuchAlgorithmException {
        inst.addTransformer(new SFSTransformer());
    }
}
