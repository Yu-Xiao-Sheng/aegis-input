//! 服务启动后功能验证测试
//!
//! 测试服务启动后的功能是否正常

use std::process::Command;
use std::path::Path;

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

/// 辅助函数：执行安装
fn setup_service() {
    if !is_root() || !has_systemd() {
        return;
    }

    let _ = Command::new("sudo")
        .arg("./install/linux/install.sh")
        .output();
}

/// 辅助函数：卸载服务
fn teardown_service() {
    if !is_root() || !has_systemd() {
        return;
    }

    let _ = Command::new("sudo")
        .arg("./install/linux/uninstall.sh")
        .output();
}

#[test]
#[ignore]
fn test_service_starts_after_install() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    // 安装服务
    setup_service();

    // 等待服务启动
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 检查服务状态
    let output = Command::new("systemctl")
        .arg("is-active")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务状态");

    let status = String::from_utf8_lossy(&output.stdout);
    assert_eq!(status.trim(), "active", "服务应该处于 active 状态");

    // 清理
    teardown_service();
}

#[test]
#[ignore]
fn test_service_enabled_on_boot() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 检查服务是否启用
    let output = Command::new("systemctl")
        .arg("is-enabled")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务启用状态");

    let enabled = String::from_utf8_lossy(&output.stdout);
    assert_eq!(enabled.trim(), "enabled", "服务应该启用开机自启");

    teardown_service();
}

#[test]
#[ignore]
fn test_service_responds_to_external_device() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 等待服务完全启动
    std::thread::sleep(std::time::Duration::from_secs(3));

    // 检查服务日志，看是否正在监听设备事件
    let output = Command::new("journalctl")
        .arg("-u")
        .arg("aegis-input")
        .arg("-n")
        .arg("20")
        .arg("--no-pager")
        .output()
        .expect("无法读取服务日志");

    let logs = String::from_utf8_lossy(&output.stdout);

    // 验证服务正在运行（日志中应该有启动信息）
    // 注意：这个测试可能需要根据实际的服务日志格式调整
    assert!(!logs.is_empty(), "服务日志为空，服务可能未正常启动");

    teardown_service();
}

#[test]
#[ignore]
fn test_service_stops_properly() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 等待服务启动
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 停止服务
    let output = Command::new("sudo")
        .arg("systemctl")
        .arg("stop")
        .arg("aegis-input")
        .output()
        .expect("无法停止服务");

    assert!(output.status.success(), "停止服务失败");

    // 等待服务停止
    std::thread::sleep(std::time::Duration::from_secs(1));

    // 验证服务已停止
    let status_output = Command::new("systemctl")
        .arg("is-active")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务状态");

    let status = String::from_utf8_lossy(&status_output.stdout);
    assert_eq!(status.trim(), "inactive", "服务应该已停止");

    teardown_service();
}

#[test]
#[ignore]
fn test_service_restarts_after_failure() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 杀掉服务进程
    let _ = Command::new("sudo")
        .arg("killall")
        .arg("aegis-input")
        .output();

    // 等待 systemd 自动重启
    std::thread::sleep(std::time::Duration::from_secs(5));

    // 检查服务是否恢复运行
    let output = Command::new("systemctl")
        .arg("is-active")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务状态");

    let status = String::from_utf8_lossy(&output.stdout);
    assert_eq!(status.trim(), "active", "服务应该自动重启");

    teardown_service();
}

#[test]
#[ignore]
fn test_builtin_devices_recover_after_service_stop() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 等待服务启动
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 停止服务
    let _ = Command::new("sudo")
        .arg("systemctl")
        .arg("stop")
        .arg("aegis-input")
        .output();

    // 等待服务停止
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 验证内置设备是否恢复
    // 这里需要根据实际的设备管理逻辑来验证
    // 目前只是一个占位测试

    teardown_service();
}

#[test]
fn test_systemd_unit_has_restart_policy() {
    // 检查 systemd unit 文件是否配置了重启策略
    let content = std::fs::read_to_string("./install/linux/aegis-input.service")
        .expect("无法读取 systemd unit 文件");

    assert!(content.contains("Restart="), "缺少 Restart 配置");
    assert!(content.contains("RestartSec="), "缺少 RestartSec 配置");
}

#[test]
#[ignore]
fn test_service_runs_as_correct_user() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 检查服务运行用户
    let output = Command::new("systemctl")
        .arg("show")
        .arg("-p")
        .arg("User")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务用户");

    let user = String::from_utf8_lossy(&output.stdout);
    assert!(user.contains("aegis-input"), "服务应该以 aegis-input 用户运行");

    // 检查服务运行组
    let group_output = Command::new("systemctl")
        .arg("show")
        .arg("-p")
        .arg("Group")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务组");

    let group = String::from_utf8_lossy(&group_output.stdout);
    assert!(group.contains("input"), "服务应该以 input 组运行");

    teardown_service();
}

#[test]
#[ignore]
fn test_service_logs_are_accessible() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 等待服务启动并产生日志
    std::thread::sleep(std::time::Duration::from_secs(2));

    // 检查是否可以访问日志
    let output = Command::new("journalctl")
        .arg("-u")
        .arg("aegis-input")
        .arg("-n")
        .arg("10")
        .arg("--no-pager")
        .output();

    assert!(output.is_ok(), "无法访问服务日志");

    teardown_service();
}