package com.maddox.rts;

import java.io.ByteArrayOutputStream;
import java.net.MalformedURLException;
import java.net.URL;

public class PhysFSLoader extends ClassLoader {
    private static PhysFSLoader loader = new PhysFSLoader();

    public void preload() {
        try {
            Class.forName("com.maddox.il2.game.Main", true, this);
        } catch (Exception exc) {
            System.err.println("Exception while preloading classes:");
            exc.printStackTrace(System.err);
        }
    }

    public static final ClassLoader loader() {
        return loader;
    }

    @Override
    protected URL findResource(String name) {
        try {
            return new URL("physfs:" + name);
        } catch (MalformedURLException e) {
            return null;
        }
    }

    @Override
    public URL getResource(String name) {
        return findResource(name);
    }

    @Override
    protected Class<?> findClass(String name) throws ClassNotFoundException {
        var filePath = name.replaceAll("\\.", "/");
        var fileName = String.format("%s.class", filePath);

        try (PhysFSInputStream classStream = new PhysFSInputStream(fileName)) {
            var outputStream = new ByteArrayOutputStream();
            var buffer = new byte[4096];

            var bytesRead = 0;
            while ((bytesRead = classStream.read(buffer)) > 0) {
                outputStream.write(buffer, 0, bytesRead);
            }

            var classBytes = outputStream.toByteArray();

            if (classBytes.length > 0) {
                return defineClass(name, classBytes, 0, classBytes.length);
            } else {
                throw new ClassNotFoundException(name);
            }
        } catch (Exception exc) {
            throw new ClassNotFoundException(name, exc);
        }
    }
}
