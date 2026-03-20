# Android Project Ownership

`src-tauri/gen/android` is a maintained generated project: tracked files in this directory are intentionally versioned and may carry project-specific Gradle and wiring changes.

## What To Edit Here

- Project wiring, Gradle settings, wrapper versions, and module path overrides that belong to the Android project itself.
- Shared Java or Gradle configuration that should apply to the maintained Android workspace.

## What Not To Treat As Source Of Truth

- `tauri.settings.gradle` is ignored and disposable regeneration output.
- `.gradle`, `.kotlin`, `build`, `.cxx`, `.externalNativeBuild`, and similar directories are local build state.
- Generated WebView and activity template fixes should start in `src-tauri/vendor/rust/wry/src/android/kotlin`, not directly in the generated app sources.
