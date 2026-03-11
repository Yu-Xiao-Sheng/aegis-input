//! 检测模块错误类型
//!
//! 定义所有检测相关的错误

/// 检测错误
#[derive(Debug, thiserror::Error)]
pub enum DetectionError {
    /// 未检测到输入设备
    #[error("未检测到输入设备")]
    NoDevicesFound,

    /// 权限不足
    #[error("权限不足: {0}")]
    PermissionDenied(String),

    /// 设备访问失败
    #[error("设备访问失败: {0}")]
    DeviceAccessFailed(String),

    /// 无效的用户输入
    #[error("无效的用户输入: {0}")]
    InvalidInput(String),

    /// 超时
    #[error("操作超时")]
    Timeout,

    /// I/O 错误
    #[error("I/O 错误: {0}")]
    IoError(#[from] std::io::Error),

    /// 设备不支持禁用
    #[error("设备不支持禁用: {0}")]
    DeviceNotSupported(String),

    /// 配置验证失败
    #[error("配置验证失败: {0}")]
    ConfigValidationError(String),
}
