import java.lang.instrument.Instrumentation;

public class ClassLoadAgent {
    public static void agentmain(String agentArgs, Instrumentation inst) {
        inst.addTransformer(new XorTransformer());
    }
    public static void premain(String agentArgs, Instrumentation inst) {
        inst.addTransformer(new XorTransformer());
    }
}
