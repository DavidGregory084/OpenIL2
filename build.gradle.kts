plugins {
    java
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(files("data"))
    testImplementation("org.junit.jupiter", "junit-jupiter", "5.6.2")
}

tasks.test {
    useJUnitPlatform()
    testLogging {
        events("passed", "skipped", "failed")
        showStackTraces = true
    }
}

tasks.withType<Test> {
    systemProperty("java.library.path", "lib")
}