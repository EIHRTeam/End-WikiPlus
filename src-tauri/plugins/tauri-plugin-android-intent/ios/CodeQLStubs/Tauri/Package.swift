// swift-tools-version:5.3

import PackageDescription

let package = Package(
  name: "Tauri",
  platforms: [
    .iOS(.v13),
    .macOS(.v10_13),
  ],
  products: [
    .library(
      name: "Tauri",
      targets: ["Tauri"])
  ],
  targets: [
    .target(
      name: "Tauri",
      path: "Sources")
  ]
)
