import com.android.build.api.dsl.ApplicationExtension
import com.android.build.api.dsl.LibraryExtension
import org.gradle.api.JavaVersion
import org.gradle.kotlin.dsl.configure

subprojects {
    pluginManager.withPlugin("com.android.application") {
        extensions.configure<ApplicationExtension> {
            compileOptions {
                sourceCompatibility = JavaVersion.VERSION_21
                targetCompatibility = JavaVersion.VERSION_21
            }
        }
    }

    pluginManager.withPlugin("com.android.library") {
        extensions.configure<LibraryExtension> {
            compileOptions {
                sourceCompatibility = JavaVersion.VERSION_21
                targetCompatibility = JavaVersion.VERSION_21
            }
        }
    }
}

tasks.register("clean").configure {
    delete(layout.buildDirectory)
}
