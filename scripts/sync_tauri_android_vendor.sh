#!/usr/bin/env bash

set -euo pipefail

# Refresh the vendored Android dependency modules consumed by
# src-tauri/gen/android/settings.gradle. Behavioral fixes that belong to these
# upstream Android modules should be encoded here so the refresh path stays
# deterministic across contributors.

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
src_tauri="$repo_root/src-tauri"
vendor_root="$src_tauri/vendor/android"
cargo_lock="$src_tauri/Cargo.lock"
cargo_home="${CARGO_HOME:-$HOME/.cargo}"

python3 - "$cargo_lock" "$cargo_home" "$vendor_root" <<'PY'
import os
import re
import shutil
import sys
from pathlib import Path

cargo_lock = Path(sys.argv[1])
cargo_home = Path(sys.argv[2])
vendor_root = Path(sys.argv[3])

if not cargo_lock.exists():
    raise SystemExit(f"Cargo.lock not found: {cargo_lock}")

lock_text = cargo_lock.read_text()
packages = {
    "tauri": "tauri-android",
    "tauri-plugin-fs": "tauri-plugin-fs",
    "tauri-plugin-notification": "tauri-plugin-notification",
    "tauri-plugin-shell": "tauri-plugin-shell",
}

versions = {}
for crate_name in packages:
    match = re.search(
        rf'\[\[package\]\]\nname = "{re.escape(crate_name)}"\nversion = "([^"]+)"',
        lock_text,
    )
    if not match:
        raise SystemExit(f"Could not find {crate_name} in {cargo_lock}")
    versions[crate_name] = match.group(1)

registry_roots = [
    path for path in (cargo_home / "registry" / "src").glob("*") if path.is_dir()
]
if not registry_roots:
    raise SystemExit(f"No cargo registry source directories found under {cargo_home}")

def find_registry_path(relative_path: Path) -> Path:
    for registry_root in registry_roots:
        candidate = registry_root / relative_path
        if candidate.exists():
            return candidate
    raise SystemExit(f"Could not locate cargo source path: {relative_path}")

vendor_root.mkdir(parents=True, exist_ok=True)

for crate_name, vendor_name in packages.items():
    version = versions[crate_name]
    if crate_name == "tauri":
        relative_path = Path(f"tauri-{version}") / "mobile" / "android"
    else:
        relative_path = Path(f"{crate_name}-{version}") / "android"

    source_path = find_registry_path(relative_path)
    destination_path = vendor_root / vendor_name

    if destination_path.exists():
        shutil.rmtree(destination_path)
    shutil.copytree(source_path, destination_path)
    build_file = destination_path / "build.gradle.kts"
    if build_file.exists():
        build_text = build_file.read_text()
        build_text = build_text.replace('    id("org.jetbrains.kotlin.android")\n', "")
        for block_name in ("compileOptions", "kotlinOptions"):
            build_text = re.sub(
                rf'\n    {block_name} \{{\n(?:        .*\n)*?    \}}',
                "",
                build_text,
            )
        build_file.write_text(build_text)
        if 'consumerProguardFiles("consumer-rules.pro")' in build_text:
            consumer_rules = destination_path / "consumer-rules.pro"
            consumer_rules.touch(exist_ok=True)
    source_rewrites = {
        Path("src/main/java/app/tauri/AppPlugin.kt"): [
            (
                "            this@AppPlugin.activity.onBackPressed()",
                "            (this@AppPlugin.activity as AppCompatActivity).onBackPressedDispatcher.onBackPressed()",
            ),
        ],
        Path(".tauri/tauri-api/src/main/java/app/tauri/AppPlugin.kt"): [
            (
                "            this@AppPlugin.activity.onBackPressed()",
                "            (this@AppPlugin.activity as AppCompatActivity).onBackPressedDispatcher.onBackPressed()",
            ),
        ],
        Path("src/main/java/app/tauri/plugin/PluginMethodData.kt"): [
            (
                "  val method: Method, methodDecorator: Command",
                "  val method: Method, _methodDecorator: Command",
            ),
        ],
        Path("src/main/java/app/tauri/plugin/PluginHandle.kt"): [
            (
                """    var pluginCursor: Class<*> = instance.javaClass
    while (pluginCursor.name != Any::class.java.name) {
      methods.addAll(listOf(*pluginCursor.declaredMethods))
      pluginCursor = pluginCursor.superclass
    }
""",
                """    var pluginCursor: Class<*>? = instance.javaClass
    while (pluginCursor != null && pluginCursor != Any::class.java) {
      methods.addAll(pluginCursor.declaredMethods)
      pluginCursor = pluginCursor.superclass
    }
""",
            ),
        ],
        Path(".tauri/tauri-api/src/main/java/app/tauri/plugin/PluginMethodData.kt"): [
            (
                "  val method: Method, methodDecorator: Command",
                "  val method: Method, _methodDecorator: Command",
            ),
        ],
        Path(".tauri/tauri-api/src/main/java/app/tauri/plugin/PluginHandle.kt"): [
            (
                """    var pluginCursor: Class<*> = instance.javaClass
    while (pluginCursor.name != Any::class.java.name) {
      methods.addAll(listOf(*pluginCursor.declaredMethods))
      pluginCursor = pluginCursor.superclass
    }
""",
                """    var pluginCursor: Class<*>? = instance.javaClass
    while (pluginCursor != null && pluginCursor != Any::class.java) {
      methods.addAll(pluginCursor.declaredMethods)
      pluginCursor = pluginCursor.superclass
    }
""",
            ),
        ],
        Path("src/main/java/NotificationSchedule.kt"): [
            (
                """  fun isRemovable(): Boolean {
    return when (this) {
      is At -> !repeating
      else -> false
    }
  }
""",
                """  fun isRemovable(): Boolean {
    return when (this) {
      is At -> !repeating
      is Interval -> false
      is Every -> false
    }
  }
""",
            ),
            (
                """  fun allowWhileIdle(): Boolean {
    return when (this) {
      is At -> allowWhileIdle
      is Interval -> allowWhileIdle
      is Every -> allowWhileIdle
      else -> false
    }
  }
""",
                """  fun allowWhileIdle(): Boolean {
    return when (this) {
      is At -> allowWhileIdle
      is Interval -> allowWhileIdle
      is Every -> allowWhileIdle
    }
  }
""",
            ),
        ],
    }
    for relative_path, replacements in source_rewrites.items():
        source_file = destination_path / relative_path
        if not source_file.exists():
            continue
        source_text = source_file.read_text()
        for old, new in replacements:
            source_text = source_text.replace(old, new)
        source_file.write_text(source_text)
    print(f"Synchronized {vendor_name} from {source_path}")
PY
