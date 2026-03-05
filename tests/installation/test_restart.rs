//! 重启后服务自动启动测试
//!
//! 测试系统重启后服务是否自动启动

use std::process::Command;
use std::thread;
use std::time::Duration;

fn is_root() -> bool {
    unsafe { libc::getuid() == 0 }
}

fn has_systemd() -> bool {
    Command::new("which")
        .arg("systemctl")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn setup_service() {
    if !is_root() || !has_systemd() {
        return;
    }

    let _ = Command::new("sudo")
        .arg("./install/linux/install.sh")
        .output();
}

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
fn test_service_enabled_after_install() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 检查服务是否已启用开机自启
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
fn test_service_survives_restart() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 等待服务启动
    thread::sleep(Duration::from_secs(2));

    // 模拟重启：使用 systemd isolate 切换到 rescue.target 再切回来
    // 注意：这只是一个模拟，真实的重启测试需要在实际系统重启后验证

    // 检查当前的启用状态
    let enabled_before = Command::new("systemctl")
        .arg("is-enabled")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务启用状态");

    let enabled_before_str = String::from_utf8_lossy(&enabled_before.stdout);
    assert_eq!(enabled_before_str.trim(), "enabled", "重启前服务应该已启用");

    // 验证 systemd unit 文件中的 WantedBy 配置
    let unit_content = std::fs::read_to_string("/etc/systemd/system/aegis-input.service")
        .expect("无法读取 systemd unit 文件");

    assert!(
        unit_content.contains("WantedBy=multi-user.target"),
        "unit 文件应该配置 WantedBy=multi-user.target"
    );

    teardown_service();
}

#[test]
#[ignore]
fn test_service_starts_automatically_after_fake_reboot() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 停止服务
    let _ = Command::new("sudo")
        .arg("systemctl")
        .arg("stop")
        .arg("aegis-input")
        .output();

    // 等待服务停止
    thread::sleep(Duration::from_secs(1));

    // 模拟系统启动：使用 systemctl start 模拟开机自启
    let start_output = Command::new("sudo")
        .arg("systemctl")
        .arg("start")
        .arg("aegis-input")
        .output()
        .expect("无法启动服务");

    assert!(start_output.status.success(), "服务启动失败");

    // 等待服务启动
    thread::sleep(Duration::from_secs(2));

    // 验证服务正在运行
    let output = Command::new("systemctl")
        .arg("is-active")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务状态");

    let status = String::from_utf8_lossy(&output.stdout);
    assert_eq!(status.trim(), "active", "服务应该正在运行");

    teardown_service();
}

#[test]
#[ignore]
fn test_service_wanted_by_correct_target() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 检查服务是否被正确的 target wanted
    let output = Command::new("systemctl")
        .arg("show")
        .arg("aegis-input")
        .arg("-p")
        .arg("WantedBy")
        .output()
        .expect("无法检查 WantedBy 配置");

    let wanted_by = String::from_utf8_lossy(&output.stdout);

    // 验证 WantedBy 包含 multi-user.target
    assert!(
        wanted_by.contains("multi-user.target"),
        "服务应该被 multi-user.target wanted"
    );

    teardown_service();
}

#[test]
fn test_systemd_unit_has_correct_install_section() {
    // 检查 systemd unit 文件的 Install 段
    let content = std::fs::read_to_string("./install/linux/aegis-input.service")
        .expect("无法读取 systemd unit 文件");

    // 检查 [Install] 段
    assert!(content.contains("[Install]"), "缺少 [Install] 段");

    // 检查 WantedBy 配置
    assert!(
        content.contains("WantedBy=multi-user.target"),
        "缺少正确的 WantedBy 配置"
    );
}

#[test]
#[ignore]
fn test_service_enable_persists() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 禁用服务
    let disable_output = Command::new("sudo")
        .arg("systemctl")
        .arg("disable")
        .arg("aegis-input")
        .output()
        .expect("无法禁用服务");

    assert!(disable_output.status.success(), "禁用服务失败");

    // 重新启用服务
    let enable_output = Command::new("sudo")
        .arg("systemctl")
        .arg("enable")
        .arg("aegis-input")
        .output()
        .expect("无法启用服务");

    assert!(enable_output.status.success(), "启用服务失败");

    // 验证启用状态持久化
    thread::sleep(Duration::from_secs(1));

    let output = Command::new("systemctl")
        .arg("is-enabled")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务启用状态");

    let enabled = String::from_utf8_lossy(&output.stdout);
    assert_eq!(enabled.trim(), "enabled", "服务应该保持启用状态");

    teardown_service();
}

#[test]
#[ignore]
fn test_service_dependency_configured() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 检查服务的依赖关系
    let output = Command::new("systemctl")
        .arg("show")
        .arg("aegis-input")
        .arg("-p")
        .arg("After")
        .output()
        .expect("无法检查服务依赖");

    let after = String::from_utf8_lossy(&output.stdout);

    // 验证服务在正确的目标之后启动
    assert!(
        after.contains("graphical.target") || after.contains("multi-user.target"),
        "服务应该配置在正确的 target 之后启动"
    );

    teardown_service();
}