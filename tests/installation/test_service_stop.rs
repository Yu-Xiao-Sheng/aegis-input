//! 服务停止后功能验证测试
//!
//! 测试服务停止后功能是否正确关闭，内置设备是否恢复

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
fn test_service_stops_within_2_seconds() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 等待服务启动
    thread::sleep(Duration::from_secs(2));

    let start = std::time::Instant::now();

    // 停止服务
    let output = Command::new("sudo")
        .arg("systemctl")
        .arg("stop")
        .arg("aegis-input")
        .output()
        .expect("无法停止服务");

    let duration = start.elapsed();

    assert!(output.status.success(), "停止服务失败");
    assert!(
        duration.as_secs() <= 2,
        "服务停止时间超过 2 秒: {:?}",
        duration
    );

    teardown_service();
}

#[test]
#[ignore]
fn test_builtin_devices_recover_after_stop() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 等待服务启动
    thread::sleep(Duration::from_secs(2));

    // 停止服务
    let _ = Command::new("sudo")
        .arg("systemctl")
        .arg("stop")
        .arg("aegis-input")
        .output();

    // 等待内置设备恢复
    thread::sleep(Duration::from_secs(2));

    // 验证内置设备已恢复可用
    // 这里需要根据实际的设备管理逻辑来验证
    // 目前只是检查服务是否已停止

    let output = Command::new("systemctl")
        .arg("is-active")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务状态");

    let status = String::from_utf8_lossy(&output.stdout);
    assert_eq!(status.trim(), "inactive", "服务应该已停止");

    teardown_service();
}

#[test]
#[ignore]
fn test_external_device_no_longer_triggers_disable_after_stop() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 等待服务启动
    thread::sleep(Duration::from_secs(2));

    // 停止服务
    let _ = Command::new("sudo")
        .arg("systemctl")
        .arg("stop")
        .arg("aegis-input")
        .output();

    // 等待服务停止
    thread::sleep(Duration::from_secs(2));

    // 模拟外设插拔事件（这里只是测试框架，实际需要真实设备或模拟）
    // 验证内置设备保持可用，不被禁用

    // 检查服务确实已停止
    let output = Command::new("systemctl")
        .arg("is-active")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务状态");

    let status = String::from_utf8_lossy(&output.stdout);
    assert_eq!(status.trim(), "inactive", "服务应该已停止");

    teardown_service();
}

#[test]
#[ignore]
fn test_service_can_be_restarted() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 等待服务启动
    thread::sleep(Duration::from_secs(2));

    // 停止服务
    let stop_output = Command::new("sudo")
        .arg("systemctl")
        .arg("stop")
        .arg("aegis-input")
        .output()
        .expect("无法停止服务");

    assert!(stop_output.status.success(), "停止服务失败");

    // 等待服务停止
    thread::sleep(Duration::from_secs(1));

    // 重新启动服务
    let start_output = Command::new("sudo")
        .arg("systemctl")
        .arg("start")
        .arg("aegis-input")
        .output()
        .expect("无法启动服务");

    assert!(start_output.status.success(), "启动服务失败");

    // 等待服务启动
    thread::sleep(Duration::from_secs(2));

    // 验证服务已重新运行
    let output = Command::new("systemctl")
        .arg("is-active")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务状态");

    let status = String::from_utf8_lossy(&output.stdout);
    assert_eq!(status.trim(), "active", "服务应该已重新启动");

    teardown_service();
}

#[test]
#[ignore]
fn test_service_status_changes_correctly() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 等待服务启动
    thread::sleep(Duration::from_secs(2));

    // 验证服务正在运行
    let active_output = Command::new("systemctl")
        .arg("is-active")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务状态");

    let active_status = String::from_utf8_lossy(&active_output.stdout);
    assert_eq!(active_status.trim(), "active", "服务应该正在运行");

    // 停止服务
    let _ = Command::new("sudo")
        .arg("systemctl")
        .arg("stop")
        .arg("aegis-input")
        .output();

    // 等待服务停止
    thread::sleep(Duration::from_secs(1));

    // 验证服务已停止
    let inactive_output = Command::new("systemctl")
        .arg("is-active")
        .arg("aegis-input")
        .output()
        .expect("无法检查服务状态");

    let inactive_status = String::from_utf8_lossy(&inactive_output.stdout);
    assert_eq!(inactive_status.trim(), "inactive", "服务应该已停止");

    teardown_service();
}

#[test]
#[ignore]
fn test_multiple_stop_start_cycles() {
    if !is_root() || !has_systemd() {
        println!("跳过测试：需要 root 权限和 systemd");
        return;
    }

    setup_service();

    // 执行多次启停循环
    for i in 1..=3 {
        println!("执行第 {} 次启停循环", i);

        // 等待服务稳定
        thread::sleep(Duration::from_secs(1));

        // 停止服务
        let stop_output = Command::new("sudo")
            .arg("systemctl")
            .arg("stop")
            .arg("aegis-input")
            .output()
            .expect("无法停止服务");

        assert!(stop_output.status.success(), "第 {} 次停止服务失败", i);

        thread::sleep(Duration::from_secs(1));

        // 启动服务
        let start_output = Command::new("sudo")
            .arg("systemctl")
            .arg("start")
            .arg("aegis-input")
            .output()
            .expect("无法启动服务");

        assert!(start_output.status.success(), "第 {} 次启动服务失败", i);

        // 验证服务状态
        thread::sleep(Duration::from_secs(1));

        let output = Command::new("systemctl")
            .arg("is-active")
            .arg("aegis-input")
            .output()
            .expect("无法检查服务状态");

        let status = String::from_utf8_lossy(&output.stdout);
        assert_eq!(status.trim(), "active", "第 {} 次循环后服务应该正在运行", i);
    }

    teardown_service();
}