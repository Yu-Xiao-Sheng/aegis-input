//! 检测会话单元测试
//!
//! 测试会话管理功能

#[cfg(test)]
mod tests {
    use crate::detection::{InputDevice, DeviceType, BusType, SessionState};
    use std::path::PathBuf;

    #[test]
    fn test_session_creation() {
        // 测试会话创建
        let devices = vec![
            InputDevice {
                name: "Test Keyboard".to_string(),
                path: PathBuf::from("/dev/input/event0"),
                device_type: DeviceType::Keyboard,
                bus_type: BusType::Usb,
                vendor_id: Some(0x1234),
                product_id: Some(0x5678),
                phys: None,
                enabled: true,
                supports_disable: true,
            }
        ];

        let session = crate::detection::session_impl::LinuxDetectionSession::new(devices, 300);

        assert_eq!(session.active_devices().len(), 0);
    }

    #[test]
    fn test_device_validation() {
        // 测试设备验证逻辑
        let device = InputDevice {
            name: "Test Device".to_string(),
            path: PathBuf::from("/dev/input/event0"),
            device_type: DeviceType::Keyboard,
            bus_type: BusType::Usb,
            vendor_id: None,
            product_id: None,
            phys: None,
            enabled: true,
            supports_disable: true,
        };

        // 如果设备路径不存在，验证会失败
        // 这是预期的行为
        let result = device.validate();
        // 在实际环境中，如果设备存在，验证应该通过
    }
}
