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

English version: [README_en-US.md](README_en-US.md)

</div>

<!-- markdownlint-restore -->

## 快速开始

### 下载与使用

请至 [Release](https://github.com/EIHRTeam/End-WikiPlus/releases/latest) 页面下载该程序。

该项目支持平台：Windows、Linux、macOS、Android、iOS。

### 项目结构说明

- 仓库结构与移动端源码归属见 [docs/project-structure.md](docs/project-structure.md)
- Android 维护项目说明见 [src-tauri/gen/android/README.md](src-tauri/gen/android/README.md)
- Vendored 依赖说明见 [src-tauri/vendor/README.md](src-tauri/vendor/README.md)

### 从源码构建

0. 安装特定平台所需依赖：

   | 平台 | 依赖项 | URL |
   |----|----|----|
   | **Windows** | Visual Studio 生成工具 | https://aka.ms/vs/stable/vs_BuildTools.exe |
   | **macOS / iOS** | Xcode | https://developer.apple.com/cn/xcode/resources |
   | **Android** | Android Studio 及 Android SDK (API Level 36+) & Android SDK Build-Tools & NDK & Gradle 9.4.1 | https://developer.android.com/studio |
   | **Android** | JDK 21 | [Oracle](https://www.oracle.com/cn/java/technologies/downloads/#java21) 或 [JetBrains](https://github.com/JetBrains/JetBrainsRuntime) |

2. 安装 Node.js 20 LTS 或更高版本：https://nodejs.org/en/download

3. 安装 pnpm：https://pnpm.io/zh/installation

4. 安装 Rust 工具链：https://rust-lang.org/tools/install/

5. 安装依赖

   ```bash
   pnpm install
   ```

6. 启动开发版

   ```bash
   pnpm tauri dev
   ```

7. 编译正式版

   ```bash
   pnpm tauri build
   ```

## 贡献者

<a href="https://github.com/EIHRTeam/End-WikiPlus/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=EIHRTeam/End-WikiPlus" />
</a>

## 鸣谢

（排名不分先后）
- [AndreaFrederica](https://github.com/AndreaFrederica) - JEI-Web
- [梦溪潇](https://github.com/mengxixiao) - 早期 iOS / macOS 适配与 API 支持
- [终末地Delta机器人](https://www.skland.com/profile?id=7532688806929) - 森空岛终末地 Wiki / 鹰角网络
- 森空岛终末地 Wiki 全体编者

## 技术栈

框架：Tauri v2

前端：Vue 3 & TypeScript

后端：Rust / Kotlin / Swift

## 免责声明

本软件为开源免费、非营利性质的同人作品，本软件及其开发者与**上海市鹰角网络科技有限公司及其关联实体**（下称“鹰角网络”）没有任何资金或组织上的联系。

本软件内所提供的全部 Wiki 内容均来自[森空岛终末地 Wiki](https://wiki.skland.com/endfield)，所有内容均按原样提供，本软件不会对其进行任何修改，亦不对其负任何责任。

本软件不会向除鹰角网络外的任何个人或组织收集并上传用户的任何信息。

本软件上传至鹰角网络服务器的用户信息适用[《森空岛个人信息保护政策》](https://assets.skland.com/protocols/privacy.html)与[《森空岛使用许可及服务协议》](https://assets.skland.com/protocols/agreement.html)。

该免责声明及本软件之用户协议及隐私政策适用且仅适用于本软件之开发者在**该 GitHub 仓库的 Release 页面** (https://github.com/EIHRTeam/End-WikiPlus/releases) 提供的版本，本软件之开发者不对从他处获取的本软件做任何担保。

## License / 授权

Copyright © 2026 Endfield Industries Human Resources Team. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License"). You may not use, copy, modify, distribute, or otherwise exploit this software except in compliance with the License. You may obtain a copy of the License at:

https://www.apache.org/licenses/LICENSE-2.0

A copy of the License is also provided in the [LICENSE](LICENSE) file distributed with this software.

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

This software may include code, libraries, or other components from third parties that are licensed under separate terms. For a complete list of such third-party components and their license information, see [THIRD-PARTY_LICENSES.md](THIRD-PARTY_LICENSES.md).
