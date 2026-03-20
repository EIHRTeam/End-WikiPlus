<!-- markdownlint-disable -->

<div align="center">

<img alt="LOGO" src="./src-tauri/icons/icon.png" width="256" height="256" />

# Endfield Wikiplus

<!-- markdownlint-restore -->

<!-- version-badge:start -->
![Version](https://img.shields.io/badge/version-0.2.3--alpha-blue?style=for-the-badge)<br><!-- version-badge:end -->
![GitHub Repo stars](https://img.shields.io/github/stars/EIHRTeam/End-WikiPlus)
![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/EIHRTeam/End-WikiPlus/total?style=social)
[![star](https://gitee.com/EIHRTeam/End-WikiPkus/badge/star.svg?theme=dark)](https://gitee.com/EIHRTeam/End-WikiPkus)<br>
![Vue 3](https://img.shields.io/badge/Vue-3-%234FC08D?logo=vuedotjs)
![Tauri v2](https://img.shields.io/badge/Tauri-v2-%2324C8D8?logo=tauri)
![Quasar v2](https://img.shields.io/badge/Quasar-v2-%234695EB?logo=quasar)<br>
A cross-platform, open-source toolbox for the SKLAND/SKPORT Endfield Wiki<br>
一款跨平台的开源森空岛 Wiki 工具箱

English version: [README_en-US.md](README_en-US.md)

</div>

## 项目结构说明

- 仓库结构与移动端源码归属见 [docs/project-structure.md](docs/project-structure.md)
- Android 维护项目说明见 [src-tauri/gen/android/README.md](src-tauri/gen/android/README.md)
- Vendored 依赖说明见 [src-tauri/vendor/README.md](src-tauri/vendor/README.md)

## 快速开始

### 下载与使用

请至 [Release](https://github.com/EIHRTeam/End-WikiPlus/releases/latest) 页面下载该程序。

该项目支持平台：Windows、Linux、macOS、Android、iOS。

### 从源码构建

1. 安装 Node.js 20 LTS 或更高版本：https://nodejs.org/en/download

2. 安装 pnpm：https://pnpm.io/zh/installation

3. 安装 Rust 工具链：https://rust-lang.org/tools/install/

4. 安装依赖

   ```bash
   pnpm install
   ```

5. 启动开发版

   ```bash
   pnpm tauri dev
   ```

6. 编译正式版

   ```bash
   pnpm tauri build
   ```

## 贡献者

<a href="https://github.com/EIHRTeam/End-WikiPlus/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=EIHRTeam/End-WikiPlus" />
</a>

## 鸣谢

- [AndreaFrederica](https://github.com/AndreaFrederica) 及 [TheJeiWebProject](https://github.com/TheJEIWebProject)
- [梦溪潇](https://github.com/mengxixiao)

## 技术栈

Tauri、Vue 3、TypeScript

## License / 授权

Copyright © 2026 Endfield Industries Human Resources Team. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License"). You may not use, copy, modify, distribute, or otherwise exploit this software except in compliance with the License. You may obtain a copy of the License at:

https://www.apache.org/licenses/LICENSE-2.0

A copy of the License is also provided in the [LICENSE](LICENSE) file distributed with this software.

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

This software may include code, libraries, or other components from third parties that are licensed under separate terms. For a complete list of such third-party components and their license information, see [THIRD-PARTY_LICENSES.md](THIRD-PARTY_LICENSES.md).
