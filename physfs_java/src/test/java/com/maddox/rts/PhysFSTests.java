package com.maddox.rts;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

import java.io.*;
import java.nio.file.Path;
import java.nio.file.Paths;

import static org.junit.jupiter.api.Assertions.*;

@Disabled
class PhysFSTests {
    private Path testFilesPath = Paths.get("test-data").resolve("test-files.sfs");
    private Path testFiles2Path = Paths.get("test-data").resolve("test-files2.sfs");
    private Path nonExistentPath = Paths.get("test-data").resolve("test-files-missing.sfs");

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
        assertTrue(PhysFS.existsFile("test.ini"), "A file from a mounted archive did not show as existing");
    }

    @Test
    void mountNonExistentArchive() {
        assertThrows(
                PhysFSException.class,
                () -> PhysFS.mountArchive(nonExistentPath.toString()),
                "Attempting to mount a nonexistent path didn't throw an exception");
    }

    @Test
    void unmountNonMountedArchive() {
        assertThrows(
                PhysFSException.class,
                () -> PhysFS.unmountArchive(testFiles2Path.toString()),
                "Attempting to unmount a non-mounted archive didn't throw an exception");
    }

    @Test
    void unmountedFileNotExists() {
        assertFalse(PhysFS.existsFile("test2.ini"), "A file from an unmounted archive showed as existing");
    }

    @Test
    void fileExistsOnceMounted() {
        try {
            PhysFS.mountArchive(testFiles2Path.toString());
            assertTrue(PhysFS.existsFile("test2.ini"), "A file did not show as existing after its archive was mounted");
        } finally {
            PhysFS.unmountArchive(testFiles2Path.toString());
        }
    }

    @Test
    void fileExistsOnceMountedAtPath() {
        try {
            PhysFS.mountArchiveAt(testFiles2Path.toString(), "data");
            assertTrue(PhysFS.existsFile("data/test2.ini"), "A file did not show as existing after its archive was mounted at a mountpoint");
        } finally {
            PhysFS.unmountArchive(testFiles2Path.toString());
        }
    }

    @Test
    void inputStreamOpenUnmountedFile() {
        assertThrows(
                PhysFSException.class,
                () -> new PhysFSInputStream("test2.ini"),
                "Attempting to open a file from an unmounted archive didn't throw an exception");
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
            assertThrows(
                    PhysFSException.class,
                    () -> is.seek(is.fileLength() + 1),
                    "Seeking after end of file didn't throw exception");
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
    void inputStreamReadBytesWithExcessBuffer() throws IOException {
        try (PhysFSInputStream is = new PhysFSInputStream("test.ini")) {
            byte[] buf = new byte[15];
            assertEquals(13, is.read(buf), "Data length read from file was not as expected");
        }
    }

    @Test
    void inputStreamReadBytesBufferTooSmall() throws IOException {
        try (PhysFSInputStream is = new PhysFSInputStream("test.ini")) {
            byte[] buf = new byte[10];
            assertThrows(
                    IndexOutOfBoundsException.class,
                    () -> is.read(buf, 0, 13),
                    "Reading too many bytes for the buffer length didn't throw exception");
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

    @Test
    void inputStreamReadSecondFileBuffered() throws IOException {
        try {
            PhysFS.mountArchive(testFiles2Path.toString());
            try (BufferedReader rdr = new BufferedReader(new InputStreamReader(new PhysFSInputStream("test2.ini")))) {
                assertEquals("[test]", rdr.readLine(), "Data read from first line was not as expected");
                assertEquals("data=here", rdr.readLine(), "Data read from second line was not as expected");
                assertNull(rdr.readLine(), "Data unexpectedly read after end of file");
            }
        } finally {
            PhysFS.unmountArchive(testFiles2Path.toString());
        }
    }
}
