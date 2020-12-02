import org.gradle.api.tasks.testing.logging.TestExceptionFormat

plugins {
    java
    id("jacoco")
}

repositories {
    mavenCentral()
}

dependencies {
    testImplementation("org.junit.jupiter", "junit-jupiter", "5.6.2")
}

tasks.compileJava {
    sourceCompatibility = "11"
    targetCompatibility = "11"
}

tasks.test {
    useJUnitPlatform()
    testLogging {
        exceptionFormat = TestExceptionFormat.FULL
    }
}

tasks.jacocoTestReport {
    reports {
        xml.setEnabled(true)
    }
}
