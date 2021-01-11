package com.maddox.rts;

public class PhysFSLoader extends ClassLoader {

    public PhysFSLoader(ClassLoader parent) {
        super(parent);
    }

    @Override
    protected Class<?> findClass(String name) throws ClassNotFoundException {
        var filePath = name.replaceAll("\\.", "/");
        var fileName = String.format("%s.class", filePath);
        try (PhysFSInputStream classStream = new PhysFSInputStream(fileName)) {
            var buffer = new byte[classStream.available()];
            var bytesRead = classStream.read(buffer);
            if (bytesRead < 0) {
                throw new ClassNotFoundException(name);
            } else {
                return defineClass(name, buffer, 0, buffer.length);
            }
        } catch (PhysFSException exc) {
            throw new ClassNotFoundException(name, exc);
        }
    }
}
