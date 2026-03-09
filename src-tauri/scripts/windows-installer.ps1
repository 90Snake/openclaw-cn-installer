param(
  [string]$Action = "detect"
)

function Write-Step {
  param([string]$Message)
  Write-Output $Message
}

function Assert-Admin {
  $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
  $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
  if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
    throw "请用管理员权限运行安装器。"
  }
}

function Get-DistroName {
  $distros = wsl --list --quiet 2>$null | ForEach-Object { $_.Trim() } | Where-Object { $_ }
  if ($distros.Count -gt 0) {
    return $distros[0]
  }

  return "Ubuntu-24.04"
}

function Ensure-Wsl {
  $status = wsl --status 2>$null
  if ($LASTEXITCODE -ne 0) {
    Write-Step "未检测到 WSL，开始安装 Ubuntu-24.04。"
    wsl --install -d Ubuntu-24.04
    throw "WSL 安装命令已发起。若系统提示重启，请先重启 Windows 后再次运行。"
  }

  Write-Step "已检测到 WSL。"
}

function Enable-Systemd {
  param([string]$Distro)

  Write-Step "开始在 $Distro 中启用 systemd。"
  $script = @"
cat >/etc/wsl.conf <<'EOF'
[boot]
systemd=true
EOF
"@

  wsl -d $Distro -u root -- bash -lc $script
  if ($LASTEXITCODE -ne 0) {
    throw "写入 /etc/wsl.conf 失败。"
  }

  wsl --shutdown
  Write-Step "systemd 配置已写入，WSL 已关闭，重新启动后生效。"
}

function Install-OpenClaw {
  param([string]$Distro)

  Write-Step "开始在 $Distro 中执行 OpenClaw 官方安装脚本。"
  wsl -d $Distro -- bash -lc "curl -fsSL https://openclaw.ai/install.sh | bash -s -- --no-onboard"
  if ($LASTEXITCODE -ne 0) {
    throw "OpenClaw 官方安装脚本执行失败。"
  }

  Write-Step "开始注册 OpenClaw daemon。"
  wsl -d $Distro -- bash -lc "openclaw onboard --install-daemon"
  wsl -d $Distro -- bash -lc "openclaw doctor || true"
}

function Repair-OpenClaw {
  param([string]$Distro)

  Write-Step "开始执行 WSL 与 OpenClaw 诊断。"
  wsl --status
  wsl -d $Distro -- bash -lc "openclaw doctor || true"
  wsl -d $Distro -- bash -lc "openclaw status || true"
}

switch ($Action) {
  "detect" {
    Assert-Admin
    Ensure-Wsl
    $distro = Get-DistroName
    Write-Step "Windows 环境检测完成。"
    Write-Step "当前发行版: $distro"
    Write-Step "下一步将按官方推荐路线执行 WSL2 + Ubuntu + OpenClaw 安装。"
  }
  "install" {
    Assert-Admin
    Ensure-Wsl
    $distro = Get-DistroName
    Enable-Systemd -Distro $distro
    Install-OpenClaw -Distro $distro
    Write-Step "Windows 安装完成。"
  }
  "repair" {
    Assert-Admin
    Ensure-Wsl
    $distro = Get-DistroName
    Repair-OpenClaw -Distro $distro
    Write-Step "Windows 修复流程已执行完毕。"
  }
  default {
    Write-Error "未知操作: $Action"
    exit 1
  }
}
