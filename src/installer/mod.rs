//! 安装器模块 - 提供跨平台安装抽象
//!
//! 本模块定义了平台无关的安装接口，支持 Linux、Windows 和 macOS 的安装方式。

pub mod error;
pub mod logging;
pub mod linux;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 安装元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallMetadata {
    /// 安装版本
    pub version: String,
    /// 平台标识（linux/windows/macos）
    pub platform: String,
    /// 二进制安装路径
    pub install_path: PathBuf,
    /// systemd unit 文件路径
    pub unit_path: Option<PathBuf>,
    /// 安装时间
    pub installed_at: chrono::DateTime<chrono::Utc>,
}

/// 服务状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceState {
    /// 是否运行
    pub running: bool,
    /// 是否开机自启
    pub enabled: bool,
    /// 最近变更时间
    pub last_changed_at: chrono::DateTime<chrono::Utc>,
}

/// 安装配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallConfig {
    /// 配置文件路径
    pub config_path: PathBuf,
    /// 状态文件路径
    pub status_path: PathBuf,
    /// 运行用户
    pub user: String,
    /// 运行组（需包含 input）
    pub group: String,
}

/// 安装错误类型
#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    #[error("安装元数据文件不存在: {0}")]
    MetadataNotFound(PathBuf),
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
}

/// 安装结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallResult {
    /// 是否成功
    pub success: bool,
    /// 错误信息（如果有）
    pub error: Option<String>,
    /// 安装耗时（秒）
    pub duration_secs: u64,
}

/// 平台特定的安装器 trait
pub trait Installer: Send + Sync {
    /// 安装软件
    fn install(&self) -> Result<InstallResult, InstallError>;

    /// 卸载软件
    fn uninstall(&self) -> Result<InstallResult, InstallError>;

    /// 检查安装状态
    fn is_installed(&self) -> bool;

    /// 获取安装元数据
    fn get_metadata(&self) -> Result<InstallMetadata, InstallError>;

    /// 检查服务状态
    fn get_service_state(&self) -> Result<ServiceState, InstallError>;

    /// 启动服务
    fn start_service(&self) -> Result<(), InstallError>;

    /// 停止服务
    fn stop_service(&self) -> Result<(), InstallError>;

    /// 启用服务（开机自启）
    fn enable_service(&self) -> Result<(), InstallError>;

    /// 禁用服务（开机不自启）
    fn disable_service(&self) -> Result<(), InstallError>;
}

/// 安装器工厂
pub struct InstallerFactory;

impl InstallerFactory {
    /// 根据当前平台创建安装器
    pub fn create() -> Result<Box<dyn Installer>, InstallError> {
        let platform = detect_platform()?;

        match platform.as_str() {
            "linux" => Ok(Box::new(linux::LinuxInstaller::new()?)),
            _ => Err(InstallError::InstallationFailed(
                format!("不支持的平台: {}", platform)
            )),
        }
    }
}

/// 检测当前运行平台
fn detect_platform() -> Result<String, InstallError> {
    let os = std::env::consts::OS;
    match os {
        "linux" => Ok("linux".to_string()),
        "windows" => Ok("windows".to_string()),
        "macos" => Ok("macos".to_string()),
        _ => Err(InstallError::InstallationFailed(
            format!("不支持的操作系统: {}", os)
        )),
    }
}

/// 默认安装元数据
pub fn default_metadata() -> InstallMetadata {
    InstallMetadata {
        version: env!("CARGO_PKG_VERSION").to_string(),
        platform: "linux".to_string(),
        install_path: PathBuf::from("/usr/local/bin/aegis-input"),
        unit_path: Some(PathBuf::from("/etc/systemd/system/aegis-input.service")),
        installed_at: chrono::Utc::now(),
    }
}

/// 默认安装配置
pub fn default_config() -> InstallConfig {
    InstallConfig {
        config_path: PathBuf::from("/etc/aegis-input/config.toml"),
        status_path: PathBuf::from("/var/lib/aegis-input/status.toml"),
        user: "aegis-input".to_string(),
        group: "input".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_metadata() {
        let metadata = default_metadata();
        assert_eq!(metadata.platform, "linux");
        assert_eq!(metadata.install_path, PathBuf::from("/usr/local/bin/aegis-input"));
    }

    #[test]
    fn test_default_config() {
        let config = default_config();
        assert_eq!(config.user, "aegis-input");
        assert_eq!(config.group, "input");
    }
}