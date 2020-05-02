plugins {
    java
}

repositories {
    mavenCentral()
}

version = "0.1.0-SNAPSHOT"

tasks.compileJava {
    sourceCompatibility = "1.8"
    targetCompatibility = "1.8"
}

tasks.jar {
    archiveFileName.set("${project.name}.jar")

    manifest {
        attributes(mapOf(
                "Agent-Class" to "ClassLoadAgent",
                "Premain-Class" to "ClassLoadAgent",
                "Implementation-Title" to "XorTransformer",
                "Implementation-Version" to rootProject.version
        ))
    }
}