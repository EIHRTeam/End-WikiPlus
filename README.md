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

> [!NOTE]
> 森空岛终末地 Wiki 已于 2026 年 5 月 14 日上线移动版，鉴于此，本项目开发将暂缓进行，期限不定。请访问 [wiki.skland.com](httos://wiki.skland.com/endfield)，或查看[更新公告](https://www.skland.com/article?id=5853075)了解更多信息。

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
   | **Android** | JDK 21 | - |

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

## 技术栈

框架：Tauri v2

前端：Vue 3 & TypeScript

后端：Rust / Kotlin / Swift

## 开发计划

[Milestone: Coming S∞n™](https://github.com/EIHRTeam/End-WikiPlus/milestone/2)

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

## 法律信息

### 免责声明

本软件系依据 Apache License 2.0 协议开源发布的免费、非官方同人软件。本软件及其开发者与**上海市鹰角网络科技有限公司及其关联实体**（以下统称“鹰角网络”）不存在任何资金或组织上的关系；本软件亦不代表鹰角网络的立场或行为。

本软件内提供的全部 Wiki 内容均来源于[森空岛终末地 Wiki](https://wiki.skland.com/endfield)，并按原始状态提供。本软件及其开发者不对相关内容的真实性、准确性、完整性、合法性、时效性或由此产生的后果承担责任。

除实现本软件功能所必需外，本软件不会向鹰角网络以外的任何个人、组织或第三方收集、传输或上传用户信息。

用户通过本软件上传至鹰角网络服务器的信息，适用[《森空岛个人信息保护政策》](https://assets.skland.com/protocols/privacy.html)及[《森空岛使用许可及服务协议》](https://assets.skland.com/protocols/agreement.html)；用户在登录“鹰角通行证”账号时上传至鹰角网络服务器的信息，适用[《个人信息保护政策》](https://user.hypergryph.com/protocol/privacy)、[《用户注册协议》](https://user.hypergryph.com/protocol/registration)与[《儿童个人信息保护政策》](https://user.hypergryph.com/protocol/children_privacy)。为实现特定功能，本软件可能要求用户登录“鹰角通行证”账号；在该等情况下，本软件仅会在必要范围内向鹰角网络服务器发送相关账号信息，并采用通行的加密方式在用户设备本地存储相关信息。

本软件按“现状”提供。除法律法规另有强制性规定外，本软件及其开发者不作任何明示或默示保证。

本免责声明及本软件的用户协议、隐私政策等说明，仅适用于本软件开发者在该 GitHub 仓库的 Release 页面提供的二进制软件包：https://github.com/EIHRTeam/End-WikiPlus/releases

对于自其他渠道获取的本软件副本、修改版本、再分发版本或衍生版本，本软件开发者不作任何保证，亦不承担任何责任。

### License / 授权

Copyright © 2026 Endfield Industries Human Resources Team. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License"). You may not use, copy, modify, distribute, or otherwise exploit this software except in compliance with the License. You may obtain a copy of the License at:

https://www.apache.org/licenses/LICENSE-2.0

A copy of the License is also provided in the [LICENSE](LICENSE) file distributed with this software.

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

This software may include code, libraries, or other components from third parties that are licensed under separate terms. For a complete list of such third-party components and their license information, see [THIRD-PARTY_LICENSES.md](THIRD-PARTY_LICENSES.md).
