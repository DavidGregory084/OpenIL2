import org.gradle.api.tasks.testing.logging.TestExceptionFormat

plugins {
    java
    id("jacoco")
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(files("data"))
    testImplementation("org.junit.jupiter", "junit-jupiter", "5.6.2")
}

tasks.compileJava {
    sourceCompatibility = "1.3"
    targetCompatibility = "1.3"
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
