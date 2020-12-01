package com.maddox.instrument;

import java.lang.instrument.Instrumentation;

public class ClassLoadAgent {
    public static void agentmain(String agentArgs, Instrumentation inst) {
        inst.addTransformer(new SFSTransformer());
    }
    public static void premain(String agentArgs, Instrumentation inst) {
        inst.addTransformer(new SFSTransformer());
    }
}
