fn main() {
    tauri_plugin::Builder::new(&[
        "openLink",
        "saveMedia",
        "saveMediaFromUrl",
        "saveMediaFromFile",
        "checkPermissions",
        "requestPermissions",
        "openAppSettings",
    ])
    .android_path("android")
    .ios_path("ios")
    .build();
}
