# Project Structure

This repository keeps a few generated and vendored trees under version control on purpose. The goal is to make Android and mobile builds reproducible without relying on mutable cargo-cache or Tauri-generated paths.

## Directory Roles

- `.github/actions`
  Shared CI building blocks. Android workflow setup lives here and should be reused by workflows instead of duplicated inline.
- `scripts/`
  Repo-owned maintenance scripts. Use these for refresh and update flows that should stay consistent across contributors.
- `src-tauri/gen/android`
  The maintained Android project. Tracked files here are intentional project inputs; ignored files here remain disposable regeneration output.
- `src-tauri/vendor/android`
  Vendored Android dependency modules consumed by the maintained Android project.
- `src-tauri/vendor/rust/wry`
  Vendored Rust dependency patch used as the source of truth for Android WebView and activity template fixes.
- `build-android.*`
  Local-only helper scripts at the repository root. They are not part of the tracked project structure contract.

## Source Of Truth

- Fixes to generated Android Kotlin templates start in `src-tauri/vendor/rust/wry`, not under `src-tauri/gen/android/app/.../generated`.
- Refreshes to vendored Android modules start in `scripts/sync_tauri_android_vendor.sh`.
- Shared Android CI setup starts in `.github/actions/setup-android-build-env`.
