<!-- markdownlint-disable -->

<div align="center">

<img alt="LOGO" src="./src-tauri/icons/icon.png" width="256" height="256" />

# Endfield Wikiplus

<!-- version-badge:start -->
![Version](https://img.shields.io/badge/version-0.2.3--alpha-blue?style=for-the-badge)<br><!-- version-badge:end -->
![GitHub Repo stars](https://img.shields.io/github/stars/EIHRTeam/End-WikiPlus)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/EIHRTeam/End-WikiPlus/total?style=social)
[![star](https://gitee.com/EIHRTeam/End-WikiPlus/badge/star.svg?theme=dark)](https://gitee.com/EIHRTeam/End-WikiPlus)<br>
![Vue 3](https://img.shields.io/badge/Vue-3-%234FC08D?logo=vuedotjs)
![Tauri v2](https://img.shields.io/badge/Tauri-v2-%2324C8D8?logo=tauri)
![Quasar v2](https://img.shields.io/badge/Quasar-v2-%234695EB?logo=quasar)<br>
[![Ask DeepWiki](https://deepwiki.com/badge.svg)](https://deepwiki.com/EIHRTeam/End-WikiPlus)<br>
A cross-platform, open-source toolbox for the SKLAND/SKPORT Endfield Wiki<br>
一款跨平台的开源森空岛 Wiki 工具箱

简体中文版: [README.md](README.md)

</div>

<!-- markdownlint-restore -->

> [!NOTE]
> The Skland Endfield Wiki of launched its mobile version on May 14, 2026. In light of this, development on this project will be temporarily paused for an indefinite period. Please visit [wiki.skland.com](https://wiki.skland.com/endfield) or see the [update announcement](https://www.skland.com/article?id=5853075) for more information.

## Quick Start

### Download and Usage

Please visit the [Release](https://github.com/EIHRTeam/End-WikiPlus/releases/latest) page to download the application.

Supported platforms: Windows, Linux, macOS, Android, iOS.

### Project Structure

- For repository structure and mobile source code attribution, see [docs/project-structure.md](docs/project-structure.md)
- For Android maintenance project notes, see [src-tauri/gen/android/README.md](src-tauri/gen/android/README.md)
- For vendored dependency notes, see [src-tauri/vendor/README.md](src-tauri/vendor/README.md)

### Building from Source

0. Install platform-specific prerequisites:

   | Platform | Prerequisites | URL |
   |----|----|----|
   | **Windows** | Visual Studio Build Tools | https://aka.ms/vs/stable/vs_BuildTools.exe |
   | **macOS / iOS** | Xcode | https://developer.apple.com/cn/xcode/resources |
   | **Android** | Android Studio with Android SDK (API Level 36+), Android SDK Build-Tools, NDK, and Gradle 9.4.1 | https://developer.android.com/studio |
   | **Android** | JDK 21 | - |

2. Install Node.js 20 LTS or later: https://nodejs.org/en/download

3. Install pnpm: https://pnpm.io/installation

4. Install the Rust toolchain: https://rust-lang.org/tools/install/

5. Install dependencies

   ```bash
   pnpm install
   ```

6. Start the development build

   ```bash
   pnpm tauri dev
   ```

7. Compile the production build

   ```bash
   pnpm tauri build
   ```

## Tech Stack

Framework: Tauri v2

Frontend: Vue 3 & TypeScript

Backend: Rust / Kotlin / Swift

## Roadmap

[Milestone: Coming S∞n™](https://github.com/EIHRTeam/End-WikiPlus/milestone/2)

## Contributors

<a href="https://github.com/EIHRTeam/End-WikiPlus/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=EIHRTeam/End-WikiPlus" />
</a>

## Acknowledgements

(Listed in no particular order)
- [AndreaFrederica](https://github.com/AndreaFrederica) - JEI-Web
- [梦溪潇](https://github.com/mengxixiao) - Early iOS / macOS adaptation and API support
- [Delta Bot](https://www.skland.com/profile?id=7532688806929) - SKLAND Endfield Wiki / Hypergryph
- All editors of the SKLAND Endfield Wiki

## Legal Information

### Disclaimer

This software is a free, unofficial fan-made application distributed as open-source software under the Apache License 2.0. This software and its developers have no financial or organizational affiliation with **Shanghai Hypergryph Network Technology Co., Ltd. or any of its affiliated entities** (hereinafter collectively referred to as "Hypergryph"). This software does not represent the position or actions of Hypergryph.

All Wiki content provided within this software is sourced from the [SKLAND Endfield Wiki](https://wiki.skland.com/endfield) and is provided as-is. This software and its developers assume no responsibility for the authenticity, accuracy, completeness, legality, timeliness, or any consequences arising from such content.

Except as strictly necessary for the operation of this software, this software does not collect, transmit, or upload user information to any individual, organization, or third party other than Hypergryph.

Information uploaded to Hypergryph's servers through this software is subject to the [SKLAND Personal Information Protection Policy](https://assets.skland.com/protocols/privacy.html) and the [SKLAND License and Service Agreement](https://assets.skland.com/protocols/agreement.html). Information uploaded to Hypergryph's servers upon login to a "Hypergryph Passport" account is subject to the [Personal Information Protection Policy](https://user.hypergryph.com/protocol/privacy), the [User Registration Agreement](https://user.hypergryph.com/protocol/registration), and the [Children's Personal Information Protection Policy](https://user.hypergryph.com/protocol/children_privacy). To enable certain features, this software may require users to log in to their "Hypergryph Passport" account. In such cases, this software will only transmit the relevant account information to Hypergryph's servers to the extent necessary, and will store such information locally on the user's device using standard encryption methods.

This software is provided "as is." Except as required by applicable law, this software and its developers make no warranties, express or implied.

This disclaimer, along with the end user license agreement, privacy policy, and other notices pertaining to this software, applies solely to the binary packages provided by the software developers on the Release page of this GitHub repository: https://github.com/EIHRTeam/End-WikiPlus/releases

The software developers make no representations and assume no liability with respect to any copies, modified versions, redistributions, or derivative works of this software obtained through any other channel.

### License

Copyright © 2026 Endfield Industries Human Resources Team. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License"). You may not use, copy, modify, distribute, or otherwise exploit this software except in compliance with the License. You may obtain a copy of the License at:

https://www.apache.org/licenses/LICENSE-2.0

A copy of the License is also provided in the [LICENSE](LICENSE) file distributed with this software.

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

This software may include code, libraries, or other components from third parties that are licensed under separate terms. For a complete list of such third-party components and their license information, see [THIRD-PARTY_LICENSES.md](THIRD-PARTY_LICENSES.md).
