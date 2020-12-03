package com.maddox.instrument;

import io.sigpipe.jbsdiff.Patch;
import org.objectweb.asm.ClassReader;
import org.objectweb.asm.ClassWriter;
import org.objectweb.asm.Type;
import org.objectweb.asm.commons.ClassRemapper;
import org.objectweb.asm.commons.Remapper;
import org.objectweb.asm.commons.SimpleRemapper;
import org.objectweb.asm.util.CheckClassAdapter;

import java.io.ByteArrayOutputStream;
import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.IllegalClassFormatException;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.security.ProtectionDomain;
import java.util.Base64;
import java.util.Map;
import java.util.Set;

public class SFSTransformer implements ClassFileTransformer {
    // Patches
    static String AIRCRAFT_HASH = "BZhTkHgIo6ueEQMxEhogLrsZECbQuJJEGSdV/mbqDz8=";
    static String FLIGHT_MODEL_MAIN_HASH = "/qOb3QVBSaCHDPAbpd5Rz2WAgnflPcvJnLl+cu7yyVc=";
    static String IN_OUT_STREAMS_HASH = "EDZ8V7U1Wk5Ay8+3jQt0hzH0+t4x2Sm3th2eFT3u+1o=";
    static String INPUT_STREAM_OF_INPUT_STREAM_HASH = "F8ZE8WEJzhYQmqyxTQD9hkATOcczUcBY+rz/bOkM/2U=";
    static String MAIN_HASH = "SrYyadsKERh5f/4brV40GlSnwR+mbDYjdtd6AKK3gG8=";
    static String RTS_HASH = "jdlTl7o2LFAdbw9j+i6hoyjAKYaEJwpRfIKCpHQc/0Y=";

    MessageDigest messageDigest = MessageDigest.getInstance("SHA3-256");
    Base64.Encoder base64Encoder = Base64.getEncoder();

    static Map<String, String> patches = Map.ofEntries(
            Map.entry(AIRCRAFT_HASH, "/Aircraft.patch"),
            Map.entry(FLIGHT_MODEL_MAIN_HASH, "/FlightModelMain.patch"),
            Map.entry(IN_OUT_STREAMS_HASH, "/InOutStreams.patch"),
            Map.entry(INPUT_STREAM_OF_INPUT_STREAM_HASH, "/InputStreamOfInputStream.patch"),
            Map.entry(MAIN_HASH, "/Main.patch"),
            Map.entry(RTS_HASH, "/RTS.patch")
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
        skipClasses = Set.of(
                PHYSFS_TYPE.getClassName(),
                PHYSFS_READER_TYPE.getClassName(),
                PHYSFS_EXCEPTION_TYPE.getClassName(),
                PHYSFS_INPUT_STREAM_TYPE.getClassName()
        );

        mapping = Map.ofEntries(
                Map.entry(SFS_TYPE.getInternalName(), PHYSFS_TYPE.getInternalName()),
                Map.entry(SFS_READER_TYPE.getInternalName(), PHYSFS_READER_TYPE.getInternalName()),
                Map.entry(SFS_EXCEPTION_TYPE.getInternalName(), PHYSFS_EXCEPTION_TYPE.getInternalName()),
                Map.entry(SFS_INPUT_STREAM_TYPE.getInternalName(), PHYSFS_INPUT_STREAM_TYPE.getInternalName()),
                Map.entry(SFS_TYPE.getInternalName() + "." + "mount" + SFS_MOUNT_DESCRIPTOR1, "mountArchive"),
                Map.entry(SFS_TYPE.getInternalName() + "." + "mount" + SFS_MOUNT_DESCRIPTOR2, "mountArchive"),
                Map.entry(SFS_TYPE.getInternalName() + "." + "mountAs" + SFS_MOUNT_AS_DESCRIPTOR1, "mountArchiveAt"),
                Map.entry(SFS_TYPE.getInternalName() + "." + "mountAs" + SFS_MOUNT_AS_DESCRIPTOR2, "mountArchiveAt"),
                Map.entry(SFS_TYPE.getInternalName() + "." + "unMount" + SFS_UNMOUNT_DESCRIPTOR, "unmountArchive")
        );

        remapper = new SimpleRemapper(mapping);
    }

    public SFSTransformer() throws NoSuchAlgorithmException {
    }

    public byte[] transform(ClassLoader classLoader, String className, Class<?> classBeingRedefined, ProtectionDomain protectionDomain, byte[] classFileBuffer) throws IllegalClassFormatException {
        try {
            if (!className.startsWith("com.maddox") || skipClasses.contains(className)) {
                return classFileBuffer;
            } else {
                var hashBytes = messageDigest.digest(classFileBuffer);
                var hashString = base64Encoder.encodeToString(hashBytes);
                var patchedBuffer = classFileBuffer;

                // Apply patches
                if (patches.containsKey(hashString)) {
                    var patchName = patches.get(hashString);
                    var patch = getClass().getResourceAsStream(patchName);
                    if (patch != null) {
                        var outputStream = new ByteArrayOutputStream();
                        Patch.patch(classFileBuffer, patch.readAllBytes(), outputStream);
                        patchedBuffer = outputStream.toByteArray();
                    } else {
                        System.err.println("Unable to retrieve patch file for class: " + className + " with hash: " + hashString);
                    }
                }

                // Transform references to SFS code
                var reader = new ClassReader(patchedBuffer);
                var writer = new ClassWriter(ClassWriter.COMPUTE_MAXS);
                var visitor = new CheckClassAdapter(new ClassRemapper(writer, remapper));
                reader.accept(visitor, ClassReader.SKIP_FRAMES);

                return writer.toByteArray();
            }
        } catch (Throwable throwable) {
            System.err.println("Exception thrown while transforming class: " + className);
            throwable.printStackTrace(System.err);
            return classFileBuffer;
        }
    }
}
