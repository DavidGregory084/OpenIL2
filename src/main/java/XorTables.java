import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;
import java.util.concurrent.atomic.AtomicInteger;

public class XorTables {
    public static AtomicInteger currentTable = new AtomicInteger(0);
    public static ConcurrentMap<Integer, byte[]> tables = new ConcurrentHashMap<>();

    static {
        // Initial table at offset 420840
        tables.put(0, new byte[] {
                (byte)0x6E, (byte)0x47, (byte)0xED, (byte)0x02,
                (byte)0x4D, (byte)0x3F, (byte)0x60, (byte)0x5C,
                (byte)0x28, (byte)0xB6, (byte)0xF7, (byte)0xBE,
                (byte)0x0B, (byte)0xCE
        });

        // Next table at offset 420860
        tables.put(1, new byte[] {
                (byte)0x73, (byte)0xBA, (byte)0x2F, (byte)0xCC,
                (byte)0xFE, (byte)0xE4, (byte)0x4A, (byte)0x45,
                (byte)0x69, (byte)0x06, (byte)0x69, (byte)0x3D,
                (byte)0xE4, (byte)0x58, (byte)0x01, (byte)0x56
        });

        // Next table at offset 420850
        tables.put(2, new byte[] {
                (byte)0x79, (byte)0xA1, (byte)0x92, (byte)0x70,
                (byte)0x5A, (byte)0xD9, (byte)0x1F, (byte)0x2E,
                (byte)0x3F, (byte)0x50, (byte)0x88, (byte)0xCC,
                (byte)0x1C, (byte)0x28
        });

        // Next table at offset 420874
        tables.put(3, new byte[] {
                (byte)0x1A, (byte)0x92, (byte)0x2C, (byte)0x32,
                (byte)0x7F, (byte)0x1B, (byte)0xBB, (byte)0xD0,
                (byte)0x5C, (byte)0x63, (byte)0x36, (byte)0x8E,
                (byte)0x7D, (byte)0xCD, (byte)0xBC, (byte)0xE6,
                (byte)0x5E, (byte)0xB5
        });

        // Next table at offset 420888
        tables.put(4, new byte[] {
                (byte)0x69, (byte)0xAB, (byte)0x82, (byte)0x60,
                (byte)0x4A, (byte)0xD3, (byte)0x0F, (byte)0x3E,
                (byte)0x22, (byte)0xB8, (byte)0xAD, (byte)0xA4,
                (byte)0x01, (byte)0xC0, (byte)0x20, (byte)0xFA,
                (byte)0x64, (byte)0x49, (byte)0xB7, (byte)0x18,
                (byte)0x47, (byte)0x31
        });
    }
}
