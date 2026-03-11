//! 设备选择器
//!
//! 定义交互式设备选择接口

use anyhow::Result;

/// 设备选择器接口
pub trait DeviceSelector: Send + Sync {
    /// 显示设备列表并获取用户选择
    fn prompt_selection(
        &self,
        all_devices: &[super::InputDevice],
        active_devices: &std::collections::HashSet<std::path::PathBuf>,
    ) -> Result<std::collections::HashSet<std::path::PathBuf>>;

    /// 验证用户选择
    fn validate_selection(
        &self,
        selection: &std::collections::HashSet<std::path::PathBuf>,
        available: &[super::InputDevice],
    ) -> Result<()>;
}
