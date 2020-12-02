plugins {
    java
}

repositories {
    mavenCentral()
}

version = "0.1.0-SNAPSHOT"

dependencies {
    implementation("org.ow2.asm:asm-commons:9.0")
    implementation("org.ow2.asm:asm-util:9.0")
    implementation("io.sigpipe:jbsdiff:1.0")
    testImplementation("org.junit.jupiter", "junit-jupiter", "5.6.2")
}

tasks.compileJava {
    sourceCompatibility = "11"
    targetCompatibility = "11"
}

tasks.test {
    useJUnitPlatform()
    testLogging {
        exceptionFormat = org.gradle.api.tasks.testing.logging.TestExceptionFormat.FULL
    }
}

tasks.jar {
    archiveFileName.set("${project.name}.jar")

    from(configurations.runtimeClasspath.get().map {
        if (it.isDirectory) it else zipTree(it)
    })

    manifest {
        attributes(mapOf(
                "Agent-Class" to "com.maddox.instrument.ClassLoadAgent",
                "Premain-Class" to "com.maddox.instrument.ClassLoadAgent",
                "Implementation-Title" to "SFSTransformer",
                "Implementation-Version" to rootProject.version
        ))
    }
}