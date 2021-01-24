package com.maddox.rts;

import java.io.ByteArrayOutputStream;
import java.io.IOException;

public class PhysFSLoader extends ClassLoader {

    public PhysFSLoader(ClassLoader parent) {
        super(parent);
    }

    @Override
    protected Class<?> findClass(String name) throws ClassNotFoundException {
        var filePath = name.replaceAll("\\.", "/");
        var fileName = String.format("%s.class", filePath);
        try (PhysFSInputStream classStream = new PhysFSInputStream(fileName)) {
            var outputStream = new ByteArrayOutputStream();
            var buffer = new byte[classStream.available()];

            while (classStream.read(buffer) > 0) {
                outputStream.write(buffer);
                buffer = new byte[classStream.available()];
            }

            var classBytes = outputStream.toByteArray();

            if (classBytes.length == 0) {
                throw new ClassNotFoundException(name);
            } else {
                return defineClass(name, classBytes, 0, classBytes.length);
            }
        } catch (PhysFSException | IOException exc) {
            throw new ClassNotFoundException(name, exc);
        }
    }
}
