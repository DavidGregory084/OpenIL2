import java.lang.instrument.ClassFileTransformer;
import java.lang.instrument.IllegalClassFormatException;
import java.security.ProtectionDomain;
import java.util.Arrays;

public class XorTransformer implements ClassFileTransformer {
    public static byte[] expectedMagic = new byte[] { (byte)0xCA, (byte)0xFE, (byte)0xBA, (byte)0xBE };
    public static byte[] expectedVersion = new byte[] { 0, 3, 0, (byte)0x2D };

    public byte[] transform(ClassLoader loader, String className, Class<?> classBeingRedefined, ProtectionDomain protectionDomain, byte[] classfileBuffer) throws IllegalClassFormatException {
        try {
            byte[] actualMagic = Arrays.copyOfRange(classfileBuffer, 0, 4);
            byte[] actualVersion = Arrays.copyOfRange(classfileBuffer, 4, 8);

            boolean needsDecryption = !Arrays.equals(expectedMagic, actualMagic) || !Arrays.equals(expectedVersion, actualVersion);

            System.err.println(
                    "Loading class: " + className +
                            ", magic value: " + Arrays.toString(actualMagic) +
                            ", version: " + Arrays.toString(actualVersion)
            );

            // The game switches between four tables to load encrypted classes
            // After using each table, it blanks out the entry in the list of tables,
            // then advances to the next table.
            int xorTableIdx = XorTables.currentTable.getAndUpdate(i -> {
                if (XorTables.tables.containsKey(i) && needsDecryption)
                    return (i + 1) & 4;
                else
                    return i;
            });

            if (XorTables.tables.containsKey(xorTableIdx) && needsDecryption) {
                // Remove the table entry as we are definitely using it
                byte[] xorTable = XorTables.tables.remove(xorTableIdx);

                System.err.println("XORing class data for " + className + " with key " + Arrays.toString(xorTable));

                // Add 8 extra bytes to the class data
                byte[] newClassfileBuffer = new byte[classfileBuffer.length + 8];

                // Class file magic
                newClassfileBuffer[0] = (byte) 0xCA;
                newClassfileBuffer[1] = (byte) 0xFE;
                newClassfileBuffer[2] = (byte) 0xBA;
                newClassfileBuffer[3] = (byte) 0xBE;
                newClassfileBuffer[4] = 0;
                newClassfileBuffer[5] = 0;
                newClassfileBuffer[6] = 0;
                // Class file major version 47 (Java 1.3)
                newClassfileBuffer[7] = 47;

                int remaining = classfileBuffer.length;
                int oldIdx = 0;
                int newIdx = 8;
                int xorKeyIdx = 0;

                while (remaining > 0) {
                    newClassfileBuffer[newIdx] = (byte) (xorTable[xorKeyIdx] ^ classfileBuffer[oldIdx] % 0xFF);
                    newIdx++;
                    oldIdx++;
                    xorKeyIdx = (xorKeyIdx + 1) % xorTable.length;
                    remaining--;
                }

                System.err.println("Returning new class data for: " + className);
                return newClassfileBuffer;
            }

            System.err.println("Returning original class data for: " + className);
            return classfileBuffer;

        } catch (Throwable throwable) {
            System.err.println("Exception thrown while transforming class: " + className);
            throw throwable;
        } finally {
            System.err.println("Returning original class data for: " + className);
            return classfileBuffer;
        }
    }
}
