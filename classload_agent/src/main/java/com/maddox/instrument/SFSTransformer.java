package com.maddox.instrument;

import io.sigpipe.jbsdiff.Patch;
import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassVisitor;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.Type;
import org.objectweb.asm.commons.ClassRemapper;
import org.objectweb.asm.commons.Remapper;
import org.objectweb.asm.commons.SimpleRemapper;
import org.objectweb.asm.util.CheckClassAdapter;

import java.io.ByteArrayOutputStream;
import java.io.InputStream;
import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.IllegalClassFormatException;
import java.net.URL;
import java.security.ProtectionDomain;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class SFSTransformer implements ClassFileTransformer {
    // Patches
    static Type RTS_TYPE = Type.getType("Lcom/maddox/rts/RTS;");
    static Type MAIN_TYPE = Type.getType("Lcom/maddox/il2/game/Main;");
    static Type AIRCRAFT_TYPE = Type.getType("Lcom/maddox/il2/objects/air/Aircraft;");

    static Map<String, String> patches = Map.ofEntries(
            Map.entry(RTS_TYPE.getClassName(), "/RTS.patch"),
            Map.entry(MAIN_TYPE.getClassName(), "/Main.patch"),
            Map.entry(AIRCRAFT_TYPE.getClassName(), "/Aircraft.patch")
    );

    // Remappings
    static Set<String> skipClasses;

    static Map<String, String> mapping;
    static Remapper remapper;

    static Type SFS_INPUT_STREAM_TYPE = Type.getType("Lcom/maddox/rts/SFSInputStream;");
    static Type PHYSFS_INPUT_STREAM_TYPE = Type.getType("Lcom/maddox/rts/PhysFSInputStream;");

    static Type SFS_READER_TYPE = Type.getType("Lcom/maddox/rts/SFSReader;");
    static Type PHYSFS_READER_TYPE = Type.getType("Lcom/maddox/rts/PhysFSReader;");

    static Type SFS_EXCEPTION_TYPE = Type.getType("Lcom/maddox/rts/SFSException;");
    static Type PHYSFS_EXCEPTION_TYPE = Type.getType("Lcom/maddox/rts/PhysFSException;");

    static Type SFS_TYPE = Type.getType("Lcom/maddox/rts/SFS;");
    static Type PHYSFS_TYPE = Type.getType("Lcom/maddox/rts/PhysFS;");

    static Type SFS_MOUNT_DESCRIPTOR1 = Type.getMethodType(Type.VOID_TYPE, Type.getType(String.class));
    static Type SFS_MOUNT_DESCRIPTOR2 = Type.getMethodType(Type.VOID_TYPE, Type.getType(String.class), Type.INT_TYPE);
    static Type SFS_MOUNT_AS_DESCRIPTOR1 = Type.getMethodType(Type.VOID_TYPE, Type.getType(String.class), Type.getType(String.class));
    static Type SFS_MOUNT_AS_DESCRIPTOR2 = Type.getMethodType(Type.VOID_TYPE, Type.getType(String.class), Type.getType(String.class), Type.INT_TYPE);
    static Type SFS_UNMOUNT_DESCRIPTOR = Type.getMethodType(Type.VOID_TYPE, Type.getType(String.class));

    static {
        skipClasses = new HashSet<>();
        skipClasses.add(PHYSFS_INPUT_STREAM_TYPE.getClassName());
        skipClasses.add(PHYSFS_READER_TYPE.getClassName());
        skipClasses.add(PHYSFS_EXCEPTION_TYPE.getClassName());
        skipClasses.add(PHYSFS_TYPE.getClassName());

        mapping = new HashMap<>();
        mapping.put(SFS_INPUT_STREAM_TYPE.getInternalName(), PHYSFS_INPUT_STREAM_TYPE.getInternalName());
        mapping.put(SFS_READER_TYPE.getInternalName(), PHYSFS_READER_TYPE.getInternalName());
        mapping.put(SFS_EXCEPTION_TYPE.getInternalName(), PHYSFS_EXCEPTION_TYPE.getInternalName());
        mapping.put(SFS_TYPE.getInternalName(), PHYSFS_TYPE.getInternalName());
        mapping.put(SFS_TYPE.getInternalName() + "." + "mount" + SFS_MOUNT_DESCRIPTOR1, "mountArchive");
        mapping.put(SFS_TYPE.getInternalName() + "." + "mount" + SFS_MOUNT_DESCRIPTOR2, "mountArchive");
        mapping.put(SFS_TYPE.getInternalName() + "." + "mountAs" + SFS_MOUNT_AS_DESCRIPTOR1, "mountArchiveAt");
        mapping.put(SFS_TYPE.getInternalName() + "." + "mountAs" + SFS_MOUNT_AS_DESCRIPTOR2, "mountArchiveAt");
        mapping.put(SFS_TYPE.getInternalName() + "." + "unMount" + SFS_UNMOUNT_DESCRIPTOR, "unmountArchive");
        remapper = new SimpleRemapper(mapping);
    }

    public byte[] transform(ClassLoader classLoader, String className, Class<?> classBeingRedefined, ProtectionDomain protectionDomain, byte[] classFileBuffer) throws IllegalClassFormatException {
        try {
            var outputStream = new ByteArrayOutputStream();
            var patchedBuffer = classFileBuffer;

            if (patches.containsKey(className)) {
                var patchName = patches.get(className);
                var patch = getClass().getClassLoader().getResourceAsStream(patchName);
                if (patch != null) {
                    Patch.patch(classFileBuffer, patch.readAllBytes(), outputStream);
                    patchedBuffer = outputStream.toByteArray();
                } else {
                    System.err.println("Unable to retrieve patch file for class: " + className);
                }
            }

            if (!className.startsWith("com.maddox") || skipClasses.contains(className)) {
               return patchedBuffer;
            } else {
                var reader = new ClassReader(patchedBuffer);
                var writer = new ClassWriter(0);
                var visitor = new CheckClassAdapter(new ClassRemapper(writer, remapper));
                reader.accept(visitor, 0);
                return writer.toByteArray();
            }
        } catch (Throwable throwable) {
            System.err.println("Exception thrown while transforming class: " + className);
            throwable.printStackTrace(System.err);
            return classFileBuffer;
        }
    }
}
