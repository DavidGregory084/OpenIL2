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
    void inputStreamSeek() {
        long expectedPosition = -1;
        long actualPosition = -1;

        try (PhysFSInputStream is = new PhysFSInputStream("test.ini")) {
            expectedPosition = is.fileLength() / 2;
            is.seek(expectedPosition);
            actualPosition = is.tell();
        }

        assertTrue(actualPosition > 0, "Seek position remains at start of file after seeking");
        assertEquals(expectedPosition, actualPosition, "Seek position was not set as required");
    }

    @Test
    void inputStreamSeekEof() {
        long seekPosition = -1;
        long fileLength = 0;

        try (PhysFSInputStream is = new PhysFSInputStream("test.ini")) {
            fileLength = is.fileLength();
            is.seek(fileLength);
            seekPosition = is.tell();
        }

        assertEquals(seekPosition, fileLength, "Seek position was not set to end of file on request");
    }

    @Test
    void inputStreamSeekAfterEof() {
        try (PhysFSInputStream is = new PhysFSInputStream("test.ini")) {
            assertThrows(PhysFSException.class, () -> is.seek(is.fileLength() + 1));
        }
    }

    @Test
    void inputStreamEndOfFile() {
        try (PhysFSInputStream is = new PhysFSInputStream("test.ini")) {
            is.seek(is.fileLength());
            assertTrue(is.endOfFile(), "End of file condition was not set when seeking to end of file");
        }
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
