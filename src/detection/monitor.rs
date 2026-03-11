//! 设备事件监听
//!
//! 定义输入设备事件监听接口

use anyhow::Result;

/// 设备事件监听器接口
#[async_trait::async_trait]
pub trait DeviceMonitor: Send + Sync {
    /// 创建监听器
    async fn new(device: &super::InputDevice) -> Result<Self>
    where
        Self: Sized;

    /// 开始监听事件
    async fn start(&mut self) -> Result<()>;

    /// 获取下一个事件
    async fn next_event(&mut self) -> Result<InputEvent>;

    /// 停止监听
    async fn stop(&mut self) -> Result<()>;

    /// 检查设备是否支持禁用
    fn supports_disable(&self) -> bool;
}

/// 输入事件
#[derive(Debug, Clone)]
pub struct InputEvent {
    pub device_path: std::path::PathBuf,
    pub event_type: InputEventType,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// 输入事件类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEventType {
    /// 键盘事件
    Keyboard { key_code: u16 },
    /// 鼠标事件
    Mouse { x: i32, y: i32 },
    /// 未知事件
    Unknown,
}
