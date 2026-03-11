//! Linux 输入设备扫描
//!
//! 使用 evdev 扫描和识别输入设备

use crate::detection::{BusType, DeviceType, InputDevice};
use anyhow::Result;
use std::path::PathBuf;

/// 扫描所有输入设备
pub fn scan_input_devices() -> Result<Vec<InputDevice>> {
    let mut devices = Vec::new();

    // 扫描 /dev/input/event* 设备
    for i in 0..=20 {
        let path = PathBuf::from(format!("/dev/input/event{}", i));
        if !path.exists() {
            continue;
        }

        // 尝试打开设备
        match evdev::Device::open(&path) {
            Ok(device) => {
                // 检查是否是键盘或鼠标
                if let Some(input_device) = parse_evdev_device(&device) {
                    devices.push(input_device);
                }
            }
            Err(_) => {
                // 无法打开设备，跳过
                continue;
            }
        }
    }

    Ok(devices)
}

/// 从 evdev::Device 解析 InputDevice
fn parse_evdev_device(device: &evdev::Device) -> Option<InputDevice> {
    let name = device.name().unwrap_or("Unknown").to_string();

    // 从设备的物理路径获取设备编号
    // 简化处理：使用设备名称的一部分来推断路径
    let path = extract_device_path(&name);

    // 检查设备类型
    let device_type = get_device_type(device);
    if !matches!(device_type, DeviceType::Keyboard | DeviceType::Mouse) {
        return None;
    }

    // 获取总线类型
    let bus_type = get_bus_type(device);

    // 检查是否支持禁用（外置设备可以禁用）
    let supports_disable = matches!(bus_type, BusType::Usb | BusType::Bluetooth);

    Some(InputDevice {
        name,
        path,
        device_type,
        bus_type,
        vendor_id: None,
        product_id: None,
        phys: None,
        enabled: true,
        supports_disable,
    })
}

/// 从设备名称提取设备路径
fn extract_device_path(name: &str) -> PathBuf {
    // 尝试在 /dev/input 下查找匹配的设备
    for i in 0..=20 {
        let path = PathBuf::from(format!("/dev/input/event{}", i));
        if path.exists() {
            return path;
        }
    }
    PathBuf::from("/dev/input/event0")
}

/// 获取设备类型
fn get_device_type(device: &evdev::Device) -> DeviceType {
    // 检查是否支持键盘事件
    let supports_key = device
        .supported_events()
        .iter()
        .any(|event| matches!(event, evdev::EventType::KEY));

    // 检查是否支持相对事件（鼠标）
    let supports_rel = device
        .supported_events()
        .iter()
        .any(|event| matches!(event, evdev::EventType::RELATIVE));

    match (supports_key, supports_rel) {
        (true, false) => DeviceType::Keyboard,
        (false, true) => DeviceType::Mouse,
        (true, true) => {
            // 同时支持键盘和鼠标事件
            // 检查设备名称判断
            let name = device.name().unwrap_or("").to_lowercase();
            if name.contains("keyboard") || name.contains("key") {
                DeviceType::Keyboard
            } else if name.contains("mouse") || name.contains("pointer") {
                DeviceType::Mouse
            } else {
                DeviceType::Unknown
            }
        }
        _ => DeviceType::Unknown,
    }
}

/// 获取总线类型
fn get_bus_type(device: &evdev::Device) -> BusType {
    // 通过设备名称判断总线类型
    let name = device.name().unwrap_or("").to_lowercase();

    if name.contains("usb") {
        BusType::Usb
    } else if name.contains("bluetooth") || name.contains("bluetooth") {
        BusType::Bluetooth
    } else if name.contains("ps/2") || name.contains("i8042") || name.contains("serio") {
        BusType::Ps2
    } else if name.contains("i2c") {
        BusType::I2c
    } else if name.contains("platform") {
        BusType::Platform
    } else {
        BusType::Unknown
    }
}
