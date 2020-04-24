package com.maddox.rts;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.io.*;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Arrays;

import static org.junit.jupiter.api.Assertions.*;

class PhysFSTests {
    Path testFilesPath = Paths.get("test-data").resolve("test-files.sfs");

    @BeforeEach
    void beforeEach() {
        PhysFS.mountArchive(testFilesPath.toString());
    }

    @AfterEach
    void afterEach() {
        PhysFS.unmountArchive(testFilesPath.toString());
    }

    @Test
    void existsFile() {
        assertTrue(PhysFS.existsFile("test.ini"));
    }

    @Test
    void inputStreamTell() throws IOException {
        long seekPosition = -1;

        try (PhysFSInputStream is = new PhysFSInputStream("test.ini")) {
            seekPosition = is.tell();
        }

        assertEquals(0L, seekPosition, "Seek position was not zero for a newly opened file");
    }

    @Test
    void inputStreamFileLength() throws IOException {
        long fileLength = -1;

        try (PhysFSInputStream is = new PhysFSInputStream("test.ini")) {
            fileLength = is.fileLength();
        }

        assertTrue(fileLength > 0, "File length was not greater than zero for a test data file");
    }

    @Test
    void inputStreamReadBytes() throws IOException {
        try (BufferedReader rdr = new BufferedReader(new InputStreamReader(new PhysFSInputStream("test.ini")))) {
            assertEquals("[foo]", rdr.readLine());
            assertEquals("bar=baz", rdr.readLine());
            assertNull(rdr.readLine());
        }
    }
}
