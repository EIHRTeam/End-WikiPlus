// swift-tools-version:5.3

import PackageDescription

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
    .package(name: "Tauri", path: "../.tauri/tauri-api")
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
