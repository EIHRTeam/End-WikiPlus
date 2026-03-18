#!/usr/bin/env python3

from __future__ import annotations

import argparse
from pathlib import Path

START_MARKER = "<!-- version-badge:start -->"
END_MARKER = "<!-- version-badge:end -->"


def encode_shields_value(value: str) -> str:
    return value.replace("-", "--").replace("_", "__").replace(" ", "_")


def update_badge(path: Path, tag_name: str) -> bool:
    content = path.read_text(encoding="utf-8")

    if content.count(START_MARKER) != 1 or content.count(END_MARKER) != 1:
        raise SystemExit(f"{path}: expected exactly one version badge marker pair")

    start = content.index(START_MARKER)
    end = content.index(END_MARKER, start)

    encoded_tag = encode_shields_value(tag_name)
    replacement = (
        f"{START_MARKER}\n"
        f"![Version](https://img.shields.io/badge/version-{encoded_tag}-blue?style=for-the-badge)<br>{END_MARKER}"
    )

    updated = content[:start] + replacement + content[end + len(END_MARKER) :]
    if updated == content:
        return False

    path.write_text(updated, encoding="utf-8")
    return True


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("tag_name")
    parser.add_argument("files", nargs="+")
    args = parser.parse_args()

    changed_files: list[str] = []
    for file_name in args.files:
        path = Path(file_name)
        if update_badge(path, args.tag_name):
            changed_files.append(file_name)

    if changed_files:
        print("Updated version badge in:")
        for file_name in changed_files:
            print(f" - {file_name}")
    else:
        print("No README badge changes were necessary.")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
