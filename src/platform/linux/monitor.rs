//! Linux 平台的设备监听实现
//!
//! 使用 evdev 实现异步设备事件监听

use anyhow::Result;
use crate::detection::InputDevice;
use crate::detection::{InputEvent, InputEventType};
use std::path::PathBuf;
use tokio::sync::mpsc;
use std::time::Duration;

/// Linux 设备监听器
pub struct LinuxDetectionDeviceMonitor {
    device_path: PathBuf,
    device_name: String,
    _device: evdev::Device,
    shutdown_tx: Option<mpsc::Sender<()>>,
}

#[async_trait::async_trait]
impl crate::detection::monitor::DeviceMonitor for LinuxDetectionDeviceMonitor {
    async fn new(device: &InputDevice) -> Result<Self> {
        let device_path = device.path.clone();
        let device_name = device.name.clone();

        // 打开设备
        let _device = evdev::Device::open(&device_path)?;

        Ok(Self {
            device_path,
            device_name,
            _device,
            shutdown_tx: None,
        })
    }

    async fn start(&mut self) -> Result<()> {
        // 创建关闭通道
        let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
        self.shutdown_tx = Some(shutdown_tx);

        // TODO: 实现事件监听循环
        // 这需要使用 tokio 的异步 I/O 来监听设备事件

        Ok(())
    }

    async fn next_event(&mut self) -> Result<InputEvent> {
        // TODO: 实现事件读取
        // 这需要将 evdev 的同步读取包装为异步

        Ok(InputEvent {
            device_path: self.device_path.clone(),
            event_type: InputEventType::Unknown,
            timestamp: chrono::Utc::now(),
        })
    }

    async fn stop(&mut self) -> Result<()> {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(()).await;
        }
        Ok(())
    }

    fn supports_disable(&self) -> bool {
        // TODO: 根据设备类型判断
        true
    }
}
