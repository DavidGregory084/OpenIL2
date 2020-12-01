package com.maddox.instrument;

import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.Type;
import org.objectweb.asm.commons.ClassRemapper;
import org.objectweb.asm.commons.Remapper;
import org.objectweb.asm.commons.SimpleRemapper;
import org.objectweb.asm.util.CheckClassAdapter;

import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.IllegalClassFormatException;
import java.security.ProtectionDomain;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class SFSTransformer implements ClassFileTransformer {

    static Set<String> skipClasses;

    static Map<String, String> mapping;
    static Remapper remapper;

    static Type SFS_INPUT_STREAM_TYPE = Type.getType("com/maddox/rts/SFSInputStream");
    static Type PHYSFS_INPUT_STREAM_TYPE = Type.getType("com/maddox/rts/PhysFSInputStream");

    static Type SFS_READER_TYPE = Type.getType("com/maddox/rts/SFSReader");
    static Type PHYSFS_READER_TYPE = Type.getType("com/maddox/rts/PhysFSReader");

    static Type SFS_TYPE = Type.getType("com/maddox/rts/SFS");
    static Type PHYSFS_TYPE = Type.getType("com/maddox/rts/PhysFS");

    static {
        skipClasses = new HashSet<>();
        skipClasses.add(PHYSFS_INPUT_STREAM_TYPE.getClassName());
        skipClasses.add(PHYSFS_READER_TYPE.getClassName());
        skipClasses.add(PHYSFS_TYPE.getClassName());

        mapping = new HashMap<>();
        mapping.put(SFS_INPUT_STREAM_TYPE.getInternalName(), PHYSFS_INPUT_STREAM_TYPE.getInternalName());
        mapping.put(SFS_READER_TYPE.getInternalName(), PHYSFS_READER_TYPE.getInternalName());
        mapping.put(SFS_TYPE.getInternalName(), PHYSFS_TYPE.getInternalName());
        remapper = new SimpleRemapper(mapping);
    }

    public byte[] transform(ClassLoader classLoader, String className, Class<?> classBeingRedefined, ProtectionDomain protectionDomain, byte[] classFileBuffer) throws IllegalClassFormatException {
        try {
            if (!className.startsWith("com.maddox") || skipClasses.contains(className)) {
               return classFileBuffer;
            } else {
                ClassReader reader = new ClassReader(classFileBuffer);
                ClassWriter writer = new ClassWriter(ClassWriter.COMPUTE_MAXS | ClassWriter.COMPUTE_FRAMES);
                ClassVisitor visitor = new CheckClassAdapter(new ClassRemapper(writer, remapper));
                reader.accept(visitor, ClassReader.EXPAND_FRAMES);
                return writer.toByteArray();
            }
        } catch (Throwable throwable) {
            System.err.println("Exception thrown while transforming class: " + className);
            throwable.printStackTrace(System.err);
            return classFileBuffer;
        }
    }
}
