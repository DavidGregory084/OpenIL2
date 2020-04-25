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

        assertEquals(13, fileLength, "File length was not as expected for the test data file");
    }

    @Test
    void inputStreamReadBuffered() throws IOException {
        try (BufferedReader rdr = new BufferedReader(new InputStreamReader(new PhysFSInputStream("test.ini")))) {
            assertEquals("[foo]", rdr.readLine(), "Data read from first line was not as expected");
            assertEquals("bar=baz", rdr.readLine(), "Data read from second line was not as expected");
            assertNull(rdr.readLine(), "Data unexpectedly read after end of file");
        }
    }

    @Test
    void inputStreamReadBytes() throws IOException {
        try (PhysFSInputStream is = new PhysFSInputStream("test.ini")) {
            byte[] buf = new byte[13];
            is.read(buf);
            assertEquals("[foo]\nbar=baz", new String(buf), "Data read from file was not as expected");
        }
    }

    @Test
    void inputStreamReadBytesOffset() throws IOException {
        try (PhysFSInputStream is = new PhysFSInputStream("test.ini")) {
            byte asciiSpace = (byte)'\u0020';
            byte[] buf = new byte[15];
            buf[0] = asciiSpace;
            buf[1] = asciiSpace;
            is.read(buf, 2, 13);
            assertEquals("  [foo]\nbar=baz", new String(buf), "Data read from file was not as expected");
        }
    }
}
