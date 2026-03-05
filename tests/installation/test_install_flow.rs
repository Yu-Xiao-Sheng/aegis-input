//! 安装流程集成测试
//!
//! 测试完整的安装流程，包括权限检查、服务启动等

use std::process::Command;
use std::path::Path;
use std::fs;
use tempfile::TempDir;

/// 检查是否为 root 用户
fn is_root() -> bool {
    unsafe { libc::getuid() == 0 }
}

/// 检查 systemd 是否可用
fn has_systemd() -> bool {
    Command::new("which")
        .arg("systemctl")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[test]
#[ignore] // 需要root权限，默认跳过
fn test_install_flow_with_root() {
    // 跳过条件：不是root用户
    if !is_root() {
        println!("跳过测试：需要 root 权限");
        return;
    }

    // 跳过条件：没有systemd
    if !has_systemd() {
        println!("跳过测试：systemd 不可用");
        return;
    }

    // 执行安装脚本
    let output = Command::new("sudo")
        .arg("./install/linux/install.sh")
        .output()
        .expect("执行安装脚本失败");

    // 检查安装是否成功
    assert!(output.status.success(), "安装脚本执行失败: {:?}", output);

    // 检查服务是否正在运行
    let status = Command::new("systemctl")
        .arg("is-active")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务状态");

    assert!(status.status.success(), "服务未启动");

    // 检查服务是否已启用
    let enabled = Command::new("systemctl")
        .arg("is-enabled")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务启用状态");

    assert!(enabled.status.success(), "服务未启用开机自启");

    // 检查二进制文件是否存在
    assert!(Path::new("/usr/local/bin/aegis-input").exists(), "二进制文件不存在");

    // 检查 systemd unit 文件是否存在
    assert!(
        Path::new("/etc/systemd/system/aegis-input.service").exists(),
        "systemd unit 文件不存在"
    );

    // 清理：卸载
    let _ = Command::new("sudo")
        .arg("./install/linux/uninstall.sh")
        .output();
}

#[test]
#[ignore]
fn test_install_fails_without_root() {
    // 这个测试验证没有 root 权限时安装失败
    if is_root() {
        println!("跳过测试：当前是 root 用户");
        return;
    }

    let output = Command::new("./install/linux/install.sh")
        .output()
        .expect("执行安装脚本失败");

    // 应该失败
    assert!(!output.status.success(), "没有 root 权限时安装应该失败");

    // 检查错误信息
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("权限") || stderr.contains("root"), "错误信息应该包含权限相关提示");
}

#[test]
#[ignore]
fn test_install_creates_metadata() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    // 执行安装
    let _ = Command::new("sudo")
        .arg("./install/linux/install.sh")
        .output();

    // 检查元数据文件是否创建
    let metadata_path = Path::new("/var/lib/aegis-input/install.toml");
    assert!(metadata_path.exists(), "安装元数据文件不存在");

    // 读取并验证元数据
    let content = fs::read_to_string(metadata_path)
        .expect("无法读取元数据文件");

    assert!(content.contains("version"), "元数据应该包含版本信息");
    assert!(content.contains("platform"), "元数据应该包含平台信息");

    // 清理
    let _ = Command::new("sudo")
        .arg("./install/linux/uninstall.sh")
        .output();
}

#[test]
fn test_install_script_exists() {
    // 检查安装脚本是否存在
    assert!(Path::new("./install/linux/install.sh").exists(), "安装脚本不存在");
    assert!(Path::new("./install/linux/uninstall.sh").exists(), "卸载脚本不存在");
    assert!(
        Path::new("./install/linux/aegis-input.service").exists(),
        "systemd unit 文件不存在"
    );
}

#[test]
fn test_install_scripts_are_executable() {
    // 检查脚本是否有执行权限
    let install_metadata = fs::metadata("./install/linux/install.sh")
        .expect("无法获取安装脚本元数据");

    let uninstall_metadata = fs::metadata("./install/linux/uninstall.sh")
        .expect("无法获取卸载脚本元数据");

    use std::os::unix::fs::PermissionsExt;
    let install_mode = install_metadata.permissions().mode();
    let uninstall_mode = uninstall_metadata.permissions().mode();

    // 检查是否有执行权限 (0o111)
    assert!(install_mode & 0o111 != 0, "安装脚本没有执行权限");
    assert!(uninstall_mode & 0o111 != 0, "卸载脚本没有执行权限");
}

#[test]
fn test_systemd_unit_content() {
    // 检查 systemd unit 文件内容
    let content = fs::read_to_string("./install/linux/aegis-input.service")
        .expect("无法读取 systemd unit 文件");

    // 检查必要的字段
    assert!(content.contains("[Unit]"), "缺少 [Unit] 段");
    assert!(content.contains("[Service]"), "缺少 [Service] 段");
    assert!(content.contains("[Install]"), "缺少 [Install] 段");
    assert!(content.contains("ExecStart"), "缺少 ExecStart 配置");
    assert!(content.contains("WantedBy=multi-user.target"), "缺少 WantedBy 配置");
    assert!(content.contains("User=aegis-input"), "缺少 User 配置");
    assert!(content.contains("Group=input"), "缺少 Group 配置");
}

#[test]
fn test_rust_installer_compiles() {
    // 这个测试确保 Rust 安装器代码可以编译
    // 如果编译失败，这个测试本身就不会运行
    use aegis_input::installer::LinuxInstaller;

    // 创建安装器实例（不实际安装）
    let result = LinuxInstaller::new();
    assert!(result.is_ok(), "LinuxInstaller 创建失败");

    let installer = result.unwrap();
    assert!(!installer.is_installed(), "不应该已经安装");
}

#[cfg(test)]
mod install_performance_tests {
    use super::*;
    use std::time::Instant;

    #[test]
    #[ignore]
    fn test_install_performance() {
        if !is_root() || !has_systemd() {
            return;
        }

        let start = Instant::now();

        let output = Command::new("sudo")
            .arg("./install/linux/install.sh")
            .output()
            .expect("执行安装脚本失败");

        let duration = start.elapsed();

        assert!(output.status.success(), "安装失败");
        assert!(
            duration.as_secs() < 180,
            "安装时间超过 3 分钟: {:?}",
            duration
        );

        // 清理
        let _ = Command::new("sudo")
            .arg("./install/linux/uninstall.sh")
            .output();
    }
}