plugins {
    java
}

repositories {
    mavenCentral()
}

dependencies {
    implementation(files("data"))
}

tasks.withType<Test> {
    systemProperty("java.library.path", "lib")
}