# OpenIL2

[![Build status](https://ci.appveyor.com/api/projects/status/6e8s7mp6oq80p46k/branch/main?svg=true)](https://ci.appveyor.com/project/dgregory084/openil2/branch/main)
![GitHub release (latest SemVer including pre-releases)](https://img.shields.io/github/v/release/DavidGregory084/OpenIL2?include_prereleases&sort=semver)
[![License](https://img.shields.io/badge/license-BSD%2BPatent-blue)](https://spdx.org/licenses/BSD-2-Clause-Patent.html)

*OpenIL2* is an unofficial, modernised launcher for the 2006 combat flight simulation game [IL-2 Sturmovik 1946](https://en.wikipedia.org/wiki/IL-2_Sturmovik:_1946).

This launcher enables two major changes:

* Replacement of the Java SE 1.3 Runtime Environment bundled with the game with a Java 11 runtime image built using [jlink](https://docs.oracle.com/en/java/javase/11/tools/jlink.html).
* Replacement of the game's asset loading system with [PhysicsFS](https://icculus.org/physfs/), enabling assets to be loaded from archive formats such as .zip and .7z.

## Installation Instructions

* Create a fresh installation of IL-2 Sturmovik 1946, patched to version 4.14.1.
* Download the latest version of the OpenIL2 installer
* Point the installer to the directory of your new IL-2 installation

The installation process will take several minutes. As part of the installation it will repack all known content in the game's SFS archive files into new archives in ZIP format.

After installation you must launch the game using *openil2.exe* in the installation directory rather than the original *il2fb.exe*.

NOTE: This installation process is not reversible by the uninstaller due to the replacement of the Java virtual machine folders! Don't use this installer on any installation of the game that you cannot bear to lose.

## Here be dragons

This project is at a very early stage. At present it's not recommended for use except by adventurous users who are happy to report the inevitable issues that will arise from such major changes to the game.

## Why does this project exist?

IL-2 Sturmovik 1946 still has many dedicated players and an active modding community which has greatly expanded the lifespan of the game.

However, working with the proprietary archive format used in the original game is onerous, and the largest modpacks are reaching the limits of the 32-bit JVM bundled with the game once all of the required classes are loaded.

Using a new asset loading system enables game mods to be created without using special tools for working with the SFS format.

Using a newer version of the Java Virtual Machine unlocks many improvements to garbage collection, JIT compilation, tooling and extensions to the Java standard library since version 1.3. Of special note is the [Shenandoah GC](https://wiki.openjdk.java.net/display/shenandoah/Main), a concurrent compacting garbage collector ideal for a simulation game like IL-2 where low GC pause times are critical.
