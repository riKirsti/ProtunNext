plugins {
    id("com.android.library") version "9.1.1"
}
android {
    namespace = "io.github.rikirsti.protunnext.android"
    compileSdk = 37
    defaultConfig {
        minSdk = 36
        targetSdk = 37
        consumerProguardFiles("consumer-rules.pro")
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    sourceSets["main"].apply {
        kotlin.directories.add("../target/android-bindings")
        jniLibs.srcDir("../target/android-jni")
    }
}
dependencies {
    api("net.java.dev.jna:jna:5.16.0@aar")
    api("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.11.0")
    testImplementation("junit:junit:4.13.2")
}
