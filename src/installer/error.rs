//! 安装器错误处理模块
//!
//! 提供统一的错误处理功能

use thiserror::Error;

/// 安装器错误类型
#[derive(Error, Debug)]
pub enum InstallError {
    #[error("安装元数据文件不存在: {0}")]
    MetadataNotFound(String),

    #[error("权限不足，请使用 root 用户执行")]
    PermissionDenied,

    #[error("systemd 未安装或不可用")]
    SystemdNotAvailable,

    #[error("安装失败: {0}")]
    InstallationFailed(String),

    #[error("卸载失败: {0}")]
    UninstallationFailed(String),

    #[error("服务操作失败: {0}")]
    ServiceOperationFailed(String),

    #[error("IO 错误: {0}")]
    IoError(#[from] std::io::Error),

    #[error("TOML 序列化错误: {0}")]
    TomlError(#[from] toml::ser::Error),

    #[error("TOML 反序列化错误: {0}")]
    TomlDeError(#[from] toml::de::Error),

    #[error("进程执行错误: {0}")]
    ProcessError(String),

    #[error("路径错误: {0}")]
    PathError(String),

    #[error("配置错误: {0}")]
    ConfigError(String),
}

/// 错误转换
impl InstallError {
    /// 从错误字符串创建错误
    pub fn new<S: Into<String>>(message: S) -> Self {
        InstallError::InstallationFailed(message.into())
    }

    /// 从进程错误转换
    pub fn from_process_status(status: std::process::ExitStatus) -> Self {
        InstallError::ProcessError(format!(
            "Process failed with exit code: {}",
            status.code().unwrap_or(-1)
        ))
    }

    /// 从路径错误转换
    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Self {
        InstallError::PathError(format!("Invalid path: {:?}", path.as_ref()))
    }
}

/// 错误处理工具
pub struct ErrorHandler;

impl ErrorHandler {
    /// 格式化错误信息
    pub fn format_error(error: &InstallError) -> String {
        match error {
            InstallError::PermissionDenied => {
                "权限不足。请使用 root 用户执行安装脚本。\n".to_string()
                    + "运行命令: sudo ./install/linux/install.sh"
            }
            InstallError::SystemdNotAvailable => {
                "systemd 未安装或不可用。\n".to_string() + "请确保您的系统支持 systemd。"
            }
            InstallError::InstallationFailed(msg) => {
                format!(
                    "安装失败: {}\n\n可能的解决方案:\n- 检查磁盘空间\n- 检查网络连接\n- 确保有足够的权限",
                    msg
                )
            }
            InstallError::ServiceOperationFailed(msg) => {
                format!(
                    "服务操作失败: {}\n\n请检查:\n- systemd 服务状态\n- 日志: journalctl -u aegis-input",
                    msg
                )
            }
            _ => format!("错误: {}", error),
        }
    }

    /// 检查是否为可恢复错误
    pub fn is_recoverable(error: &InstallError) -> bool {
        matches!(
            error,
            InstallError::InstallationFailed(_)
                | InstallError::ServiceOperationFailed(_)
                | InstallError::IoError(_)
                | InstallError::ProcessError(_)
        )
    }

    /// 获取错误代码
    pub fn error_code(error: &InstallError) -> i32 {
        match error {
            InstallError::MetadataNotFound(_) => 0,
            InstallError::PermissionDenied => 1,
            InstallError::SystemdNotAvailable => 2,
            InstallError::InstallationFailed(_) => 3,
            InstallError::UninstallationFailed(_) => 4,
            InstallError::ServiceOperationFailed(_) => 5,
            InstallError::IoError(_) => 6,
            InstallError::TomlError(_) => 7,
            InstallError::TomlDeError(_) => 8,
            InstallError::ProcessError(_) => 9,
            InstallError::PathError(_) => 10,
            InstallError::ConfigError(_) => 11,
        }
    }
}
