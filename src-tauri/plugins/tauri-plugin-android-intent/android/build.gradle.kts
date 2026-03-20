plugins {
  id("com.android.library")
}

android {
  compileSdk = 36
  namespace = "com.eihrteam.wikiplus.android_intent"
  defaultConfig {
    minSdk = 24
  }
}

dependencies {
  implementation(project(":tauri-android"))
  implementation("androidx.core:core-ktx:1.13.1")
  implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.9.0")
}
