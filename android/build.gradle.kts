plugins {
    id("com.android.library") version "8.13.2"
    id("org.jetbrains.kotlin.android") version "2.2.21"
}
android {
    namespace = "io.github.rikirsti.protunnext.android"
    compileSdk = 36
    defaultConfig {
        minSdk = 25
        consumerProguardFiles("consumer-rules.pro")
    }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
        isCoreLibraryDesugaringEnabled = true
    }
    kotlinOptions { jvmTarget = "11" }
    sourceSets["main"].apply {
        java.srcDir("../target/android-bindings")
        jniLibs.srcDir("../target/android-jni")
    }
}
dependencies {
    api("net.java.dev.jna:jna:5.16.0@aar")
    api("org.jetbrains.kotlinx:kotlinx-coroutines-core:1.11.0")
    coreLibraryDesugaring("com.android.tools:desugar_jdk_libs:2.1.5")
    testImplementation("junit:junit:4.13.2")
}
