# OpenClaw 中文安装助手

面向国内小白用户的跨平台 OpenClaw 安装器骨架。

## 目标

- Windows 提供单个 `Setup.exe` 分发包，内部只走官方推荐的 `WSL2 + Ubuntu` 路线。
- macOS 提供 `.dmg` 安装包，走本机依赖检查与安装路线。
- 前端全部中文，收口检测、安装、修复、日志查看。

## 当前已完成

- `Tauri + Vue` 项目骨架。
- 中文 GUI 首页与状态面板。
- Tauri 后端命令：环境检测、安装计划读取、平台脚本调度。
- macOS 已接入 OpenClaw 官方 `install.sh` 安装链路。
- Windows 已接入 `WSL2 + Ubuntu + 官方 install.sh` 的自动化脚本骨架。

## 目录

- `src/`: Vue 前端界面
- `src-tauri/src/`: Tauri Rust 后端
- `src-tauri/scripts/`: 平台安装/修复脚本

## 本地开发

先安装依赖：

```bash
npm install
```

然后启动前端：

```bash
npm run dev
```

如果本机已经安装 Rust 与 Tauri 依赖，再启动桌面应用：

```bash
npm run tauri:dev
```

## 打包

Windows:

```bash
npm run tauri:build
```

默认会生成 NSIS 安装包，可作为单个 `Setup.exe` 对外分发。

macOS:

```bash
npm run tauri:build
```

默认可生成 `.app` 与 `.dmg`。

当前仓库已在 macOS 上验证出包成功，产物路径：

```text
src-tauri/target/release/bundle/macos/OpenClaw中文安装助手.app
```

## GitHub Actions 自动出包

仓库已经包含工作流文件：

[`build-tauri.yml`](/Users/snake/openclaw-cn-installer/.github/workflows/build-tauri.yml)

推到 GitHub 后会自动执行：

- `windows-latest` 产出 NSIS 安装包 `exe`
- `macos-13` 产出 Intel 版 `.app`
- `macos-latest` 产出 Apple Silicon 版 `.app`

使用方式：

1. 在项目目录执行 `git init`
2. 新建 GitHub 仓库并推送到 `main`
3. 打开 GitHub 的 `Actions`
4. 手动运行 `Build OpenClaw Installer`，或者直接推送到 `main`

构建完成后，在每次工作流的 `Artifacts` 里下载：

- `openclaw-cn-installer-windows`
- `openclaw-cn-installer-macos-intel`
- `openclaw-cn-installer-macos-apple-silicon`

## 下一步建议

1. 给 Windows 脚本补“首次安装 Ubuntu 用户初始化”引导页。
2. 增加日志落盘、错误码、下载源切换和版本校验。
3. 补齐应用图标、签名、自动更新和 Windows 安装包实机验证。
