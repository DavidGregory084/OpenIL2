package com.maddox.rts;

import org.junit.jupiter.api.Test;

import java.nio.file.Path;
import java.nio.file.Paths;

import static org.junit.jupiter.api.Assertions.*;

class PhysFSTests {
    @Test
    void mount() {
        Path testFilesPath = Paths.get("lib").resolve("test-files.sfs");
        int mountResult = PhysFS.mount(testFilesPath.toString());
        assertTrue(mountResult > 0, "An error status was returned from the PhysFS mount method");
    }
}
