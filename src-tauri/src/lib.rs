use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Serialize)]
struct CheckItem {
    key: String,
    title: String,
    detail: String,
    tone: String,
}

#[derive(Serialize)]
struct Overview {
    platform: String,
    platform_label: String,
    machine_name: String,
    version: String,
    shell: String,
    recommended_channel: String,
    ready_for_install: bool,
    checks: Vec<CheckItem>,
}

#[derive(Serialize)]
struct PlanStep {
    key: String,
    title: String,
    description: String,
    action: String,
    caution: Option<String>,
}

#[derive(Serialize)]
struct InstallPlan {
    headline: String,
    summary: String,
    steps: Vec<PlanStep>,
}

#[tauri::command]
fn get_system_overview() -> Result<Overview, String> {
    let platform = env::consts::OS.to_string();
    let machine_name = hostname();
    let shell = env::var("SHELL").unwrap_or_else(|_| "未知".to_string());

    let (platform_label, version, ready_for_install, checks, recommended_channel) = match platform.as_str() {
        "windows" => windows_overview(),
        "macos" => macos_overview(),
        other => (
            format!("暂不支持: {other}"),
            "未知版本".to_string(),
            false,
            vec![CheckItem {
                key: "unsupported".to_string(),
                title: "平台检查".to_string(),
                detail: "当前版本只提供 Windows 与 macOS 安装流程。".to_string(),
                tone: "danger".to_string(),
            }],
            "请切换到受支持的平台".to_string(),
        ),
    };

    Ok(Overview {
        platform,
        platform_label,
        machine_name,
        version,
        shell,
        recommended_channel,
        ready_for_install,
        checks,
    })
}

#[tauri::command]
fn get_install_plan() -> Result<InstallPlan, String> {
    let platform = env::consts::OS;

    let (headline, summary, steps) = match platform {
        "windows" => (
            "Windows 官方推荐路线".to_string(),
            "启用 WSL2、安装 Ubuntu、完成 systemd 和 OpenClaw 进程注册。".to_string(),
            vec![
                PlanStep {
                    key: "wsl".to_string(),
                    title: "启用 WSL2".to_string(),
                    description: "自动检测管理员权限、虚拟化与 WSL 组件状态，必要时引导重启。".to_string(),
                    action: "powershell 脚本".to_string(),
                    caution: Some("第一次启用系统组件时，Windows 可能要求重启。".to_string()),
                },
                PlanStep {
                    key: "ubuntu".to_string(),
                    title: "部署 Ubuntu 环境".to_string(),
                    description: "安装或修复 Ubuntu 发行版，并开启 systemd。".to_string(),
                    action: "powershell + bash".to_string(),
                    caution: None,
                },
                PlanStep {
                    key: "openclaw".to_string(),
                    title: "安装 OpenClaw".to_string(),
                    description: "在 WSL 内安装 Node、pnpm 与 OpenClaw，执行 daemon 注册。".to_string(),
                    action: "wsl bash".to_string(),
                    caution: Some("默认只走官方推荐路线，不提供原生 Windows 安装。".to_string()),
                },
            ],
        ),
        "macos" => (
            "macOS 本机安装路线".to_string(),
            "检查 Homebrew/Node 环境，安装 OpenClaw，并提供一键修复与日志。".to_string(),
            vec![
                PlanStep {
                    key: "brew".to_string(),
                    title: "检测基础环境".to_string(),
                    description: "检查 Xcode Command Line Tools、Homebrew、Node 和 pnpm。".to_string(),
                    action: "zsh 脚本".to_string(),
                    caution: Some("缺少 Homebrew 时，会给出安装指引或自动安装入口。".to_string()),
                },
                PlanStep {
                    key: "install".to_string(),
                    title: "安装 OpenClaw".to_string(),
                    description: "拉取官方包、执行依赖安装、创建启动命令。".to_string(),
                    action: "zsh 脚本".to_string(),
                    caution: None,
                },
                PlanStep {
                    key: "repair".to_string(),
                    title: "修复与日志".to_string(),
                    description: "复查路径、权限、Node 版本和 daemon 状态，并输出中文日志。".to_string(),
                    action: "zsh 脚本".to_string(),
                    caution: None,
                },
            ],
        ),
        _ => (
            "当前平台暂不支持".to_string(),
            "请在 Windows 10/11 或 macOS 上运行。".to_string(),
            vec![PlanStep {
                key: "unsupported".to_string(),
                title: "平台不受支持".to_string(),
                description: "当前版本未实现 Linux 桌面安装器。".to_string(),
                action: "无".to_string(),
                caution: None,
            }],
        ),
    };

    Ok(InstallPlan {
        headline,
        summary,
        steps,
    })
}

#[tauri::command]
fn run_installer_action(action: &str) -> Result<String, String> {
    match env::consts::OS {
        "macos" => run_script("zsh", "macos-installer.sh", MACOS_INSTALLER, action),
        "windows" => run_script(
            "powershell",
            "windows-installer.ps1",
            WINDOWS_INSTALLER,
            action,
        ),
        other => Err(format!("当前平台不支持执行安装：{other}")),
    }
}

fn run_script(shell: &str, file_name: &str, source: &str, action: &str) -> Result<String, String> {
    let script_path = materialize_script(file_name, source)?;

    let output = if shell == "powershell" {
        Command::new(shell)
            .arg("-ExecutionPolicy")
            .arg("Bypass")
            .arg("-File")
            .arg(&script_path)
            .arg("-Action")
            .arg(action)
            .output()
    } else {
        Command::new(shell).arg(&script_path).arg(action).output()
    }
    .map_err(|error| format!("脚本启动失败：{error}"))?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if output.status.success() {
        Ok(if stdout.is_empty() {
            "操作完成".to_string()
        } else {
            stdout
        })
    } else {
        Err(if stderr.is_empty() {
            "脚本执行失败，但没有返回错误输出。".to_string()
        } else {
            stderr
        })
    }
}

fn materialize_script(file_name: &str, source: &str) -> Result<PathBuf, String> {
    let path = env::temp_dir().join(file_name);
    fs::write(&path, source).map_err(|error| format!("写入临时脚本失败：{error}"))?;
    Ok(path)
}

fn hostname() -> String {
    if cfg!(target_os = "windows") {
        env::var("COMPUTERNAME").unwrap_or_else(|_| "Windows 设备".to_string())
    } else {
        env::var("HOSTNAME").unwrap_or_else(|_| "本机".to_string())
    }
}

fn macos_overview() -> (String, String, bool, Vec<CheckItem>, String) {
    let version = command_output("sw_vers", &["-productVersion"]).unwrap_or_else(|| "未知版本".to_string());
    let has_brew = command_exists("brew");
    let has_node = command_exists("node");
    let has_pnpm = command_exists("pnpm");

    let checks = vec![
        CheckItem {
            key: "os".to_string(),
            title: "系统版本".to_string(),
            detail: format!("已检测到 macOS {version}"),
            tone: "ok".to_string(),
        },
        CheckItem {
            key: "brew".to_string(),
            title: "Homebrew".to_string(),
            detail: if has_brew {
                "已安装 Homebrew，可以继续安装依赖。".to_string()
            } else {
                "未安装 Homebrew，建议先通过图形界面引导安装。".to_string()
            },
            tone: if has_brew { "ok" } else { "warn" }.to_string(),
        },
        CheckItem {
            key: "node".to_string(),
            title: "Node.js".to_string(),
            detail: if has_node {
                "已检测到 Node.js。".to_string()
            } else {
                "未检测到 Node.js，安装流程会尝试补齐。".to_string()
            },
            tone: if has_node { "ok" } else { "warn" }.to_string(),
        },
        CheckItem {
            key: "pnpm".to_string(),
            title: "pnpm".to_string(),
            detail: if has_pnpm {
                "已检测到 pnpm。".to_string()
            } else {
                "未检测到 pnpm，安装流程会自动处理。".to_string()
            },
            tone: if has_pnpm { "ok" } else { "warn" }.to_string(),
        },
    ];

    (
        "macOS".to_string(),
        version,
        has_brew,
        checks,
        "推荐直接执行本机安装流程".to_string(),
    )
}

fn windows_overview() -> (String, String, bool, Vec<CheckItem>, String) {
    let version = command_output("cmd", &["/C", "ver"]).unwrap_or_else(|| "Windows".to_string());
    let wsl_ok = command_output("wsl", &["--status"]).is_some();

    let checks = vec![
        CheckItem {
            key: "os".to_string(),
            title: "系统版本".to_string(),
            detail: format!("已检测到 {version}"),
            tone: "ok".to_string(),
        },
        CheckItem {
            key: "wsl".to_string(),
            title: "WSL2".to_string(),
            detail: if wsl_ok {
                "已检测到 WSL，可继续检查 Ubuntu 与 systemd。".to_string()
            } else {
                "未检测到 WSL 或未完成初始化，建议先执行一键修复。".to_string()
            },
            tone: if wsl_ok { "ok" } else { "warn" }.to_string(),
        },
        CheckItem {
            key: "route".to_string(),
            title: "安装路线".to_string(),
            detail: "本安装器只提供官方推荐的 WSL2 路线。".to_string(),
            tone: "neutral".to_string(),
        },
    ];

    (
        "Windows".to_string(),
        version,
        wsl_ok,
        checks,
        "推荐管理员模式执行一键安装".to_string(),
    )
}

fn command_exists(command: &str) -> bool {
    let probe = if cfg!(target_os = "windows") { "where" } else { "which" };

    Command::new(probe)
        .arg(command)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn command_output(command: &str, args: &[&str]) -> Option<String> {
    Command::new(command)
        .args(args)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                None
            }
        })
}

const MACOS_INSTALLER: &str = include_str!("../scripts/macos-installer.sh");
const WINDOWS_INSTALLER: &str = include_str!("../scripts/windows-installer.ps1");

pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_system_overview,
            get_install_plan,
            run_installer_action
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
