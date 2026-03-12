// swift-tools-version:5.3

import Foundation
import PackageDescription

let packageDirectory = URL(fileURLWithPath: #filePath).deletingLastPathComponent()
let generatedTauriApiPath = packageDirectory
  .appendingPathComponent("../.tauri/tauri-api")
  .standardizedFileURL
  .path
let codeQlStubTauriApiPath = packageDirectory
  .appendingPathComponent("CodeQLStubs/Tauri")
  .standardizedFileURL
  .path
let tauriDependencyPath = FileManager.default.fileExists(atPath: generatedTauriApiPath)
  ? generatedTauriApiPath
  : codeQlStubTauriApiPath

let package = Package(
  name: "tauri-plugin-android-intent",
  platforms: [
    .iOS(.v13),
    .macOS(.v10_13),
  ],
  products: [
    .library(
      name: "tauri-plugin-android-intent",
      type: .static,
      targets: ["tauri-plugin-android-intent"])
  ],
  dependencies: [
    // CodeQL autobuild runs this package without generating the Tauri Swift API.
    // Fall back to a tiny local stub so static analysis can compile the plugin.
    .package(name: "Tauri", path: tauriDependencyPath)
  ],
  targets: [
    .target(
      name: "tauri-plugin-android-intent",
      dependencies: [
        .byName(name: "Tauri")
      ],
      path: "Sources"
    )
  ]
)
