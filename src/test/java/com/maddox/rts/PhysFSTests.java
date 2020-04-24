package com.maddox.rts;

import org.junit.jupiter.api.AfterEach;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.io.InputStream;
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
        PhysFSInputStream is = new PhysFSInputStream("test.ini");
        long seekPosition = -1;

        try {
            seekPosition = is.tell();
        } finally {
            is.close();
        }

        assertEquals(0L, seekPosition, "Seek position was not zero for a newly opened file");
    }

    @Test
    void inputStreamFileLength() throws IOException {
        PhysFSInputStream is = new PhysFSInputStream("test.ini");
        long fileLength = -1;

        try {
            fileLength = is.fileLength();
        } finally {
            is.close();
        }

        assertTrue(fileLength > 0, "File length was not greater than zero for a test data file");
    }

    @Test
    void inputStreamReadBytes() throws IOException {
        PhysFSInputStream is = new PhysFSInputStream("test.ini");
        byte[] buf = new byte[Math.min(Integer.MAX_VALUE, (int)is.fileLength())];
        String expected = "[foo]\nbar=baz";

        try {
            is.read(buf);
        } finally {
            is.close();
        }

        String bufString = new String(buf);
        assertFalse(bufString.isEmpty(), "Test file data was empty");
        assertEquals(expected, bufString, "Test file data was not as expected");
    }
}
