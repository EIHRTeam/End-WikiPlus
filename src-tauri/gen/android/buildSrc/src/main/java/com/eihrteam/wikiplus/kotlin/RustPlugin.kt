import com.android.build.api.dsl.ApplicationExtension
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.configure
import org.gradle.kotlin.dsl.register

const val TASK_GROUP = "rust"

open class Config {
    lateinit var rootDirRel: String
}

open class RustPlugin : Plugin<Project> {
    private lateinit var config: Config

    override fun apply(project: Project) = with(project) {
        config = extensions.create("rust", Config::class.java)

        val defaultAbiList = listOf("arm64-v8a", "armeabi-v7a", "x86", "x86_64")
        val abiList = (findProperty("abiList") as? String)?.split(',') ?: defaultAbiList

        val defaultArchList = listOf("arm64", "arm", "x86", "x86_64")
        val archList = (findProperty("archList") as? String)?.split(',') ?: defaultArchList

        val targetsList = (findProperty("targetList") as? String)?.split(',') ?: listOf("aarch64", "armv7", "i686", "x86_64")

        extensions.configure<ApplicationExtension> {
            @Suppress("UnstableApiUsage")
            flavorDimensions.add("abi")
            productFlavors {
                create("universal") {
                    dimension = "abi"
                    ndk {
                        abiFilters += abiList
                    }
                }
                defaultArchList.forEachIndexed { index, arch ->
                    create(arch) {
                        dimension = "abi"
                        ndk {
                            abiFilters.add(defaultAbiList[index])
                        }
                    }
                }
            }
        }

        afterEvaluate {
            for (profile in listOf("debug", "release")) {
                val profileCapitalized = profile.replaceFirstChar { it.uppercase() }
                val buildTask = tasks.register("rustBuildUniversal$profileCapitalized") {
                    group = TASK_GROUP
                    description = "Build dynamic library in $profile mode for all targets"
                }

                tasks.named("mergeUniversal${profileCapitalized}JniLibFolders").configure {
                    dependsOn(buildTask)
                }

                for (targetPair in targetsList.withIndex()) {
                    val targetName = targetPair.value
                    val targetArch = archList[targetPair.index]
                    val targetArchCapitalized = targetArch.replaceFirstChar { it.uppercase() }
                    val targetBuildTask = tasks.register<BuildTask>("rustBuild$targetArchCapitalized$profileCapitalized") {
                        group = TASK_GROUP
                        description = "Build dynamic library in $profile mode for $targetArch"
                        workingDirPath.set(project.layout.projectDirectory.dir(config.rootDirRel).asFile.absolutePath)
                        target.set(targetName)
                        release.set(profile == "release")
                    }

                    buildTask.configure {
                        dependsOn(targetBuildTask)
                    }

                    tasks.named("merge$targetArchCapitalized${profileCapitalized}JniLibFolders").configure {
                        dependsOn(targetBuildTask)
                    }
                }
            }
        }
    }
}
