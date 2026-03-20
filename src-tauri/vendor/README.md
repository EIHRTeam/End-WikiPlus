# Vendored Sources

This directory contains checked-in upstream sources that the build depends on directly.

## `android/`

These directories are vendored Android modules that the maintained project under `src-tauri/gen/android` points to through workspace-relative overrides in `settings.gradle`.

- Treat `scripts/sync_tauri_android_vendor.sh` as the refresh path for these modules.
- Keep behavior fixes that belong to the vendored Android modules here instead of patching cargo-cache output or regenerated Android settings files.

## `rust/wry`

This is a vendored `wry` patch consumed through `[patch.crates-io]`.

- Android template fixes for generated files such as `WryActivity.kt` and `RustWebView.kt` must start here.
- The tracked generated Kotlin files under `src-tauri/gen/android/app/src/main/java/.../generated` are outputs of this source, not the maintenance source themselves.
