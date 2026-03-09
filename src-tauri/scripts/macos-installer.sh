#!/bin/zsh

set -euo pipefail

ACTION="${1:-detect}"
INSTALL_URL="https://openclaw.ai/install.sh"

log() {
  echo "$1"
}

ensure_homebrew() {
  if command -v brew >/dev/null 2>&1; then
    return 0
  fi

  log "未检测到 Homebrew。请先安装 Homebrew，再继续。"
  exit 1
}

ensure_node() {
  if command -v node >/dev/null 2>&1; then
    return 0
  fi

  log "未检测到 Node.js，尝试通过 Homebrew 安装 Node 22+。"
  brew install node
}

ensure_pnpm() {
  if command -v pnpm >/dev/null 2>&1; then
    return 0
  fi

  if command -v corepack >/dev/null 2>&1; then
    log "未检测到 pnpm，尝试启用 Corepack。"
    corepack enable
    corepack prepare pnpm@latest --activate
    return 0
  fi

  log "未检测到 pnpm，尝试通过 npm 全局安装。"
  npm install -g pnpm
}

print_detect() {
  log "macOS 环境检测完成。"
  log "Homebrew: $(command -v brew >/dev/null 2>&1 && echo 已安装 || echo 未安装)"
  log "Node.js: $(command -v node >/dev/null 2>&1 && node -v || echo 未安装)"
  log "pnpm: $(command -v pnpm >/dev/null 2>&1 && pnpm -v || echo 未安装)"
  log "OpenClaw: $(command -v openclaw >/dev/null 2>&1 && echo 已安装 || echo 未安装)"
}

print_install() {
  ensure_homebrew
  ensure_node
  ensure_pnpm

  log "开始执行 OpenClaw 官方安装脚本。"
  curl -fsSL "$INSTALL_URL" | bash -s -- --no-onboard

  if command -v openclaw >/dev/null 2>&1; then
    log "开始注册 OpenClaw daemon。"
    openclaw onboard --install-daemon
    openclaw doctor || true
    openclaw status || true
    log "OpenClaw macOS 安装完成。"
  else
    log "OpenClaw 安装脚本执行后，未找到 openclaw 命令。请检查 PATH。"
    exit 1
  fi
}

print_repair() {
  ensure_homebrew
  ensure_node

  if command -v openclaw >/dev/null 2>&1; then
    log "开始执行 OpenClaw 诊断。"
    openclaw doctor || true
    openclaw status || true
    log "如果命令不可用，请检查 PATH 是否包含 $(npm prefix -g)/bin。"
  else
    log "未检测到 openclaw 命令，尝试重新执行官方安装脚本。"
    curl -fsSL "$INSTALL_URL" | bash -s -- --no-onboard
  fi
}

case "$ACTION" in
  detect)
    print_detect
    ;;
  install)
    print_install
    ;;
  repair)
    print_repair
    ;;
  *)
    echo "未知操作: $ACTION"
    exit 1
    ;;
esac
