# Endfield Wikiplus

![Version](https://img.shields.io/badge/version-0.1.0--alpha-blue?style=for-the-badge)<br>
一款跨平台的开源森空岛 Wiki 工具箱<br>
A cross-platform open-source toolbox for SKLAND/SKPORT Endfield Wiki

## Quick Start / 快速开始

### Download / 下载与使用

请至 [Release](https://github.com/EIHRTeam/End-WikiPlus/releases/latest) 页面下载该程序。<br>
Download the software from [Release](https://github.com/EIHRTeam/End-WikiPlus/releases/latest) page.

### Build From Source / 从源码构建

1. 安装 Node.js 20 LTS 或更高版本: https://nodejs.org/en/download <br>
   Install Node.js 20 LTS or higer version: https://nodejs.org/en/download

3. 安装 pnpm<br>
   Install pnpm
   - Windows (Powershell): `Invoke-WebRequest https://get.pnpm.io/install.ps1 -UseBasicParsing | Invoke-Expression`
   - POSIX 系统: `curl -fsSL https://get.pnpm.io/install.sh | sh -`

4. 安装依赖<br>
   Install dependence
   
   ```bash
   pnpm install
   ```

6. 启动开发版<br>
   Launch debug build
   
   ```bash
   pnpm tauri dev
   ```

8. 编译正式版<br>
   Build release version
   
   ```bash
   pnpm tauri build
   ```



## Contributors / 贡献者

<a href="https://github.com/EIHRTeam/End-WikiPlus/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=EIHRTeam/End-WikiPlus" />
</a>

## License / 授权

Copyright © 2026 Endfield Industries Human Resources Team. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License"). You may not use, copy, modify, distribute, or otherwise exploit this software except in compliance with the License. You may obtain a copy of the License at:

https://www.apache.org/licenses/LICENSE-2.0

A copy of the License is also provided in the [LICENSE](LICENSE) file distributed with this software.

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

This software may include code, libraries, or other components from third parties that are licensed under separate terms. For a complete list of such third-party components and their license information, see [THIRD-PARTY_LICENSES.md](THIRD-PARTY_LICENSES.md).
