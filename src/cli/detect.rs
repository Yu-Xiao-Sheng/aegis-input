//! Detect 子命令
//!
//! 交互式设备检测

use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// 输出格式
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputFormat {
    /// 自动检测终端能力
    Auto,
    /// 纯文本输出（无 ANSI 转义序列）
    Plain,
    /// JSON 格式输出
    Json,
}

/// Detect 子命令参数
#[derive(Parser, Debug)]
pub struct DetectArgs {
    /// 检测超时时间（秒）
    #[arg(short = 't', long, default_value = "300")]
    pub timeout: u64,

    /// 输出格式
    #[arg(short = 'o', long, default_value = "auto")]
    pub output: OutputFormat,

    /// 仅配置模式（跳过检测，直接选择设备）
    #[arg(long)]
    pub config_only: bool,
}

/// 运行检测命令
pub async fn run(args: DetectArgs) -> Result<i32> {
    // 扫描输入设备
    println!("正在扫描输入设备...\n");

    let devices = crate::platform::linux::input::scan_input_devices()?;

    if devices.is_empty() {
        eprintln!("错误: 未检测到输入设备");
        eprintln!();
        eprintln!("请确认:");
        eprintln!("  - 输入设备已连接");
        eprintln!("  - 设备驱动正常工作");
        eprintln!("  - 您有权限访问 /dev/input/event*");
        return Ok(1);
    }

    // 显示设备列表
    display_devices(&devices)?;

    // TODO: 实现检测模式和设备选择

    Ok(0)
}

/// 显示设备列表
fn display_devices(devices: &[crate::detection::InputDevice]) -> Result<()> {
    println!("检测到 {} 个输入设备:\n", devices.len());

    for (i, device) in devices.iter().enumerate() {
        println!("  [{}] {}", i + 1, device.name);
        println!("      路径: {:?}", device.path.display());
        println!("      类型: {} | 总线: {}",
            format_device_type(device.device_type),
            format_bus_type(device.bus_type)
        );
        println!();
    }

    Ok(())
}

/// 格式化设备类型
fn format_device_type(device_type: crate::detection::DeviceType) -> &'static str {
    match device_type {
        crate::detection::DeviceType::Keyboard => "键盘",
        crate::detection::DeviceType::Mouse => "鼠标",
        crate::detection::DeviceType::Unknown => "未知",
    }
}

/// 格式化总线类型
fn format_bus_type(bus_type: crate::detection::BusType) -> &'static str {
    match bus_type {
        crate::detection::BusType::Usb => "USB",
        crate::detection::BusType::Bluetooth => "蓝牙",
        crate::detection::BusType::Ps2 => "PS2",
        crate::detection::BusType::I2c => "I2C",
        crate::detection::BusType::Platform => "平台",
        crate::detection::BusType::Unknown => "未知",
    }
}
