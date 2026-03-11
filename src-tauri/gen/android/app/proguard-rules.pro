# Add project specific ProGuard rules here.
# You can control the set of applied configuration files using the
# proguardFiles setting in build.gradle.
#
# For more details, see
#   http://developer.android.com/guide/developing/tools/proguard.html

# If your project uses WebView with JS, uncomment the following
# and specify the fully qualified class name to the JavaScript interface
# class:
#-keepclassmembers class fqcn.of.javascript.interface.for.webview {
#   public *;
#}

# Uncomment this to preserve the line number information for
# debugging stack traces.
#-keepattributes SourceFile,LineNumberTable

# If you keep the line number information, uncomment this to
# hide the original source file name.
#-renamesourcefileattribute SourceFile

# ── Google Tink / androidx.security:security-crypto ──────────────────────
# Tink references JSR-305 / javax.annotation annotations that are not
# present in the Android runtime.  They are compile-time-only and safe
# to ignore at shrink time.
-dontwarn javax.annotation.**
-dontwarn javax.annotation.concurrent.**

# Keep the Tauri plugin classes so they can be loaded via reflection
-keep class com.eihrteam.wikiplus.secure_storage.** { *; }
-keep class com.eihrteam.wikiplus.android_intent.** { *; }