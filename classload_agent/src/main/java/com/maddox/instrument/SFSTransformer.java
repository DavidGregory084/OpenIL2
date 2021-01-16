package com.maddox.instrument;

import io.sigpipe.jbsdiff.Patch;
import org.objectweb.asm.*;
import org.objectweb.asm.commons.ClassRemapper;
import org.objectweb.asm.commons.Remapper;
import org.objectweb.asm.commons.SimpleRemapper;
import org.objectweb.asm.util.CheckClassAdapter;

import java.io.ByteArrayInputStream;
import java.io.ByteArrayOutputStream;
import java.io.IOException;
import java.io.PipedInputStream;
import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.IllegalClassFormatException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardOpenOption;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.security.ProtectionDomain;
import java.util.Arrays;
import java.util.Base64;
import java.util.Map;
import java.util.Set;

public class SFSTransformer implements ClassFileTransformer {
    // Patches
    static String AIRCRAFT_HASH = "BZhTkHgIo6ueEQMxEhogLrsZECbQuJJEGSdV/mbqDz8=";
    static String CONFIG_HASH = "kcAnaWNw35+0V+v5U40bQTkCbTmySyiqQOPpwiQd9dU=";
    static String FLIGHT_MODEL_MAIN_HASH = "/qOb3QVBSaCHDPAbpd5Rz2WAgnflPcvJnLl+cu7yyVc=";
    static String IN_OUT_STREAMS_HASH = "EDZ8V7U1Wk5Ay8+3jQt0hzH0+t4x2Sm3th2eFT3u+1o=";
    static String INPUT_STREAM_OF_INPUT_STREAM_HASH = "F8ZE8WEJzhYQmqyxTQD9hkATOcczUcBY+rz/bOkM/2U=";
    static String MAIN_HASH = "SrYyadsKERh5f/4brV40GlSnwR+mbDYjdtd6AKK3gG8=";
    static String RTS_HASH = "jdlTl7o2LFAdbw9j+i6hoyjAKYaEJwpRfIKCpHQc/0Y=";
    static String SFS_INPUT_STREAM_HASH = "aME1E8f4JebUs+j+NLPiXy5AMgjgnoS9sEzn4dwcGFk=";

    MessageDigest messageDigest = MessageDigest.getInstance("SHA3-256");
    Base64.Encoder base64Encoder = Base64.getEncoder();

    static Map<String, String> patches = Map.ofEntries(
            Map.entry(AIRCRAFT_HASH, "/Aircraft.patch"),
            Map.entry(CONFIG_HASH, "/Config.patch"),
            Map.entry(FLIGHT_MODEL_MAIN_HASH, "/FlightModelMain.patch"),
            Map.entry(IN_OUT_STREAMS_HASH, "/InOutStreams.patch"),
            Map.entry(INPUT_STREAM_OF_INPUT_STREAM_HASH, "/InputStreamOfInputStream.patch"),
            Map.entry(MAIN_HASH, "/Main.patch"),
            Map.entry(RTS_HASH, "/RTS.patch"),
            Map.entry(SFS_INPUT_STREAM_HASH, "/SFSInputStream.patch")
    );

    // Remappings
    static Set<String> skipTransformClasses;

    static Map<String, String> rewriteMappings;
    static Remapper rewriteRemapper;

    static Type SFS_INPUT_STREAM_TYPE = Type.getType("Lcom/maddox/rts/SFSInputStream;");
    static Type PHYSFS_INPUT_STREAM_TYPE = Type.getType("Lcom/maddox/rts/PhysFSInputStream;");

    static Type SFS_READER_TYPE = Type.getType("Lcom/maddox/rts/SFSReader;");
    static Type PHYSFS_READER_TYPE = Type.getType("Lcom/maddox/rts/PhysFSReader;");

    static Type SFS_EXCEPTION_TYPE = Type.getType("Lcom/maddox/rts/SFSException;");
    static Type PHYSFS_EXCEPTION_TYPE = Type.getType("Lcom/maddox/rts/PhysFSException;");

    static Type SFS_TYPE = Type.getType("Lcom/maddox/rts/SFS;");
    static Type PHYSFS_TYPE = Type.getType("Lcom/maddox/rts/PhysFS;");

    static Type KRYPTO_INPUT_FILTER_TYPE = Type.getType("Lcom/maddox/rts/KryptoInputFilter;");
    static Type KRYPTO_OUTPUT_FILTER_TYPE = Type.getType("Lcom/maddox/rts/KryptoOutputFilter;");
    static Type DUMMY_INPUT_FILTER_TYPE = Type.getType("Lcom/maddox/rts/DummyInputFilter;");
    static Type DUMMY_OUTPUT_FILTER_TYPE = Type.getType("Lcom/maddox/rts/DummyOutputFilter;");

    static Type FLIGHT_MODEL_MAIN_TYPE = Type.getType("Lcom/maddox/il2/fm/FlightModelMain;");

    static Type SFS_MOUNT_DESCRIPTOR1 = Type.getMethodType(Type.VOID_TYPE, Type.getType(String.class));
    static Type SFS_MOUNT_DESCRIPTOR2 = Type.getMethodType(Type.VOID_TYPE, Type.getType(String.class), Type.INT_TYPE);
    static Type SFS_MOUNT_AS_DESCRIPTOR1 = Type.getMethodType(Type.VOID_TYPE, Type.getType(String.class), Type.getType(String.class));
    static Type SFS_MOUNT_AS_DESCRIPTOR2 = Type.getMethodType(Type.VOID_TYPE, Type.getType(String.class), Type.getType(String.class), Type.INT_TYPE);
    static Type SFS_UNMOUNT_DESCRIPTOR = Type.getMethodType(Type.VOID_TYPE, Type.getType(String.class));

    static Type SFS_FILECHECK_DESCRIPTOR = Type.getMethodType(Type.BOOLEAN_TYPE);

    static {
        skipTransformClasses = Set.of(
                PHYSFS_TYPE.getInternalName(),
                PHYSFS_READER_TYPE.getInternalName(),
                PHYSFS_EXCEPTION_TYPE.getInternalName(),
                PHYSFS_INPUT_STREAM_TYPE.getInternalName(),
                DUMMY_INPUT_FILTER_TYPE.getInternalName(),
                DUMMY_OUTPUT_FILTER_TYPE.getInternalName(),

                // Don't transform classes that are rewrite targets
                SFS_TYPE.getInternalName(),
                SFS_READER_TYPE.getInternalName(),
                SFS_EXCEPTION_TYPE.getInternalName(),
                SFS_INPUT_STREAM_TYPE.getInternalName(),
                KRYPTO_INPUT_FILTER_TYPE.getInternalName(),
                KRYPTO_OUTPUT_FILTER_TYPE.getInternalName(),

                // Don't transform FlightModelMain because it decrypts FMs using KryptoInputFilter
                FLIGHT_MODEL_MAIN_TYPE.getInternalName()
        );

        rewriteMappings = Map.ofEntries(
                Map.entry(SFS_TYPE.getInternalName(), PHYSFS_TYPE.getInternalName()),
                Map.entry(SFS_READER_TYPE.getInternalName(), PHYSFS_READER_TYPE.getInternalName()),
                Map.entry(SFS_EXCEPTION_TYPE.getInternalName(), PHYSFS_EXCEPTION_TYPE.getInternalName()),
                Map.entry(SFS_INPUT_STREAM_TYPE.getInternalName(), PHYSFS_INPUT_STREAM_TYPE.getInternalName()),
                Map.entry(KRYPTO_INPUT_FILTER_TYPE.getInternalName(), DUMMY_INPUT_FILTER_TYPE.getInternalName()),
                Map.entry(KRYPTO_OUTPUT_FILTER_TYPE.getInternalName(), DUMMY_OUTPUT_FILTER_TYPE.getInternalName()),

                Map.entry(SFS_TYPE.getInternalName() + "." + "mount" + SFS_MOUNT_DESCRIPTOR1, "mountArchive"),
                Map.entry(SFS_TYPE.getInternalName() + "." + "mount" + SFS_MOUNT_DESCRIPTOR2, "mountArchive"),
                Map.entry(SFS_TYPE.getInternalName() + "." + "mountAs" + SFS_MOUNT_AS_DESCRIPTOR1, "mountArchiveAt"),
                Map.entry(SFS_TYPE.getInternalName() + "." + "mountAs" + SFS_MOUNT_AS_DESCRIPTOR2, "mountArchiveAt"),
                Map.entry(SFS_TYPE.getInternalName() + "." + "unMount" + SFS_UNMOUNT_DESCRIPTOR, "unmountArchive"),

                Map.entry(SFS_INPUT_STREAM_TYPE.getInternalName() + "." + "CheckComFiles" + SFS_FILECHECK_DESCRIPTOR, "dummyCheck"),
                Map.entry(SFS_INPUT_STREAM_TYPE.getInternalName() + "." + "CheckClientExeFiles" + SFS_FILECHECK_DESCRIPTOR, "dummyCheck"),
                Map.entry(SFS_INPUT_STREAM_TYPE.getInternalName() + "." + "StartClientDllFiles" + SFS_FILECHECK_DESCRIPTOR, "dummyCheck"),
                Map.entry(SFS_INPUT_STREAM_TYPE.getInternalName() + "." + "StartClientSfsCheck" + SFS_FILECHECK_DESCRIPTOR, "dummyCheck")
        );

        rewriteRemapper = new SimpleRemapper(rewriteMappings);
    }

    public SFSTransformer() throws NoSuchAlgorithmException {
    }

    public byte[] transform(ClassLoader classLoader, String className, Class<?> classBeingRedefined, ProtectionDomain protectionDomain, byte[] classFileBuffer) throws IllegalClassFormatException {
        return transform(className, classFileBuffer);
    }

    public byte[] transform(String className, byte[] classFileBuffer) {
        try {
            if (!className.startsWith("com/maddox/")) {
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
                if (!skipTransformClasses.contains(className)) {
                    var reader = new ClassReader(patchedBuffer);
                    var writer = new ClassWriter(ClassWriter.COMPUTE_MAXS);
                    var visitor = new CheckClassAdapter(new ClassRemapper(writer, rewriteRemapper));
                    reader.accept(visitor, 0);
                    patchedBuffer = writer.toByteArray();
                }

                if (Arrays.equals(classFileBuffer, patchedBuffer)) {
                    return classFileBuffer;
                } else {
                    return patchedBuffer;
                }
            }
        } catch (Throwable throwable) {
            System.err.println("Exception thrown while transforming class: " + className);
            throwable.printStackTrace(System.err);
            return classFileBuffer;
        }
    }

    public static void main(String[] args) throws IOException, NoSuchAlgorithmException {
        byte[] classFileBytes;
        var buffer = new byte[4096];

        if (args.length > 0) {
            classFileBytes = Files.readAllBytes(Paths.get(args[0]));
        } else {
            var outputStream = new ByteArrayOutputStream();
            while (System.in.read(buffer) > 0) {
                outputStream.write(buffer);
            }
            classFileBytes = outputStream.toByteArray();
        }

        var className = new ClassReader(classFileBytes).getClassName();
        var transformedBytes = new SFSTransformer().transform(className, classFileBytes);

        if (args.length > 0) {
            Files.write(Paths.get(args[0]), transformedBytes, StandardOpenOption.WRITE, StandardOpenOption.TRUNCATE_EXISTING);
        } else {
            var inputStream = new ByteArrayInputStream(transformedBytes);
            while (inputStream.read(buffer) > 0) {
                System.out.write(buffer);
            }
        }
    }
}
