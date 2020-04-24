package com.maddox.rts;

import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Arrays;

import static org.junit.jupiter.api.Assertions.*;

class PhysFSTests {
    @Test
    void mountUnmount() {
        Path testFilesPath = Paths.get("test-data").resolve("test-files.sfs");
        assertDoesNotThrow(() -> PhysFS.mountArchive(testFilesPath.toString()));
        assertDoesNotThrow(() -> PhysFS.unmountArchive(testFilesPath.toString()));
    }

    @Test
    void inputStream() throws IOException {
        Path testFilesPath = Paths.get("test-data").resolve("test-files.sfs");
        PhysFS.mountArchive(testFilesPath.toString());
        PhysFSInputStream is = new PhysFSInputStream("test.ini");
        assertEquals(0L, is.tell(), "Seek position was not zero for a newly opened file");
        assertTrue(is.fileLength() >= 0L, "File length was zero for the test data file");
    }
}
