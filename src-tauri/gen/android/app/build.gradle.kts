import java.util.Properties

plugins {
    id("com.android.application")
    id("rust")
}

val tauriProperties = Properties().apply {
    val propFile = file("tauri.properties")
    if (propFile.exists()) {
        propFile.inputStream().use { load(it) }
    }
}

android {
    compileSdk = 36
    namespace = "com.eihrteam.wikiplus.pub"
    val keystorePathEnv = System.getenv("TAURI_ANDROID_KEYSTORE_PATH")
    val keystorePasswordEnv = System.getenv("TAURI_ANDROID_KEYSTORE_PASSWORD")
    val keyAliasEnv = System.getenv("TAURI_ANDROID_KEY_ALIAS")
    val keyPasswordEnv = System.getenv("TAURI_ANDROID_KEY_PASSWORD")
    val hasReleaseSigning =
        keystorePathEnv != null &&
            keystorePasswordEnv != null &&
            keyAliasEnv != null &&
            keyPasswordEnv != null
    defaultConfig {
        manifestPlaceholders["usesCleartextTraffic"] = "false"
        applicationId = "com.eihrteam.wikiplus.pub"
        minSdk = 24
        targetSdk = 36
        versionCode = tauriProperties.getProperty("tauri.android.versionCode", "1").toInt()
        versionName = tauriProperties.getProperty("tauri.android.versionName", "1.0")
    }
    buildTypes {
        getByName("debug") {
            manifestPlaceholders["usesCleartextTraffic"] = "true"
            isDebuggable = true
            isJniDebuggable = true
            isMinifyEnabled = false
            packaging {                jniLibs.keepDebugSymbols.add("*/arm64-v8a/*.so")
                jniLibs.keepDebugSymbols.add("*/armeabi-v7a/*.so")
                jniLibs.keepDebugSymbols.add("*/x86/*.so")
                jniLibs.keepDebugSymbols.add("*/x86_64/*.so")
            }
        }
        val signingConfigName = "release"
        signingConfigs {
            create(signingConfigName) {
                if (hasReleaseSigning) {
                    storeFile = file(keystorePathEnv)
                    storePassword = keystorePasswordEnv
                    keyAlias = keyAliasEnv
                    keyPassword = keyPasswordEnv
                }
            }
        }
        getByName("release") {
            isMinifyEnabled = true
            isShrinkResources = true
            signingConfig =
                if (hasReleaseSigning) {
                    signingConfigs.getByName(signingConfigName)
                } else {
                    signingConfigs.getByName("debug")
                }
            proguardFiles(
                *fileTree(".") { include("**/*.pro") }
                    .plus(getDefaultProguardFile("proguard-android-optimize.txt"))
                    .toList().toTypedArray()
            )
        }
    }
    buildFeatures {
        buildConfig = true
    }
    lint {
        checkReleaseBuilds = false
    }
}

rust {
    rootDirRel = "../../../"
}

dependencies {
    implementation("androidx.webkit:webkit:1.14.0")
    implementation("androidx.appcompat:appcompat:1.7.1")
    implementation("androidx.activity:activity-ktx:1.10.1")
    implementation("com.google.android.material:material:1.12.0")
    testImplementation("junit:junit:4.13.2")
    androidTestImplementation("androidx.test.ext:junit:1.1.4")
    androidTestImplementation("androidx.test.espresso:espresso-core:3.5.0")
}

apply(from = "tauri.build.gradle.kts")
