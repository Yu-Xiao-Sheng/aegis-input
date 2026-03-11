//! 检测模块
//!
//! 提供交互式输入设备检测功能

use std::path::PathBuf;

pub mod session;
pub mod monitor;
pub mod selector;

/// 输入设备
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDevice {
    /// 设备名称
    pub name: String,
    /// 设备文件路径
    pub path: PathBuf,
    /// 设备类型
    pub device_type: DeviceType,
    /// 总线类型
    pub bus_type: BusType,
    /// 供应商 ID
    pub vendor_id: Option<u16>,
    /// 产品 ID
    pub product_id: Option<u16>,
    /// 物理连接位置
    pub phys: Option<String>,
    /// 当前是否启用
    pub enabled: bool,
    /// 是否支持禁用
    pub supports_disable: bool,
}

/// 设备类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    /// 键盘
    Keyboard,
    /// 鼠标
    Mouse,
    /// 未知
    Unknown,
}

/// 总线类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusType {
    /// USB
    Usb,
    /// 蓝牙
    Bluetooth,
    /// PS/2
    Ps2,
    /// I2C
    I2c,
    /// 平台设备
    Platform,
    /// 未知
    Unknown,
}
