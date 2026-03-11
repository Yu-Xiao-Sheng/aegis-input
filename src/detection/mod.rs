//! 检测模块
//!
//! 提供交互式输入设备检测功能

use std::path::PathBuf;

pub mod error;
pub mod monitor;
pub mod selector;
pub mod selector_impl;
pub mod session;
pub mod session_impl;

pub use error::DetectionError;
pub use monitor::InputEvent;
pub use monitor::InputEventType;
pub use selector::DeviceSelector;
pub use selector_impl::CliDeviceSelector;
pub use session::{DetectionSession, SessionStartInfo, SessionResult, CompletionReason, SessionState};

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

impl InputDevice {
    /// 验证设备信息
    pub fn validate(&self) -> Result<(), DetectionError> {
        // 检查名称非空
        if self.name.is_empty() {
            return Err(DetectionError::ConfigValidationError(
                "设备名称不能为空".into(),
            ));
        }

        // 检查路径是绝对路径
        if !self.path.is_absolute() {
            return Err(DetectionError::ConfigValidationError(
                format!("设备路径必须是绝对路径: {:?}", self.path),
            ));
        }

        // 检查路径存在
        if !self.path.exists() {
            return Err(DetectionError::DeviceAccessFailed(format!(
                "设备文件不存在: {:?}",
                self.path
            )));
        }

        // 检查设备类型不是 Unknown
        if matches!(self.device_type, DeviceType::Unknown) {
            return Err(DetectionError::ConfigValidationError(
                "设备类型未知".into(),
            ));
        }

        Ok(())
    }
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
