//! Linux 平台安装器实现
//!
//! 提供在 Linux 系统上安装 aegis-input 的具体实现，包括 systemd 服务管理。

use super::*;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use which::which;

/// Linux 安装器
pub struct LinuxInstaller {
    /// 安装元数据
    metadata: InstallMetadata,
    /// 安装配置
    config: InstallConfig,
    /// 系统信息
    system_info: SystemInfo,
}

/// 系统信息
#[derive(Debug, Clone)]
struct SystemInfo {
    pub is_root: bool,
    pub has_systemd: bool,
    pub has_input_group: bool,
}

impl LinuxInstaller {
    /// 创建新的 Linux 安装器
    pub fn new() -> Result<Self, InstallError> {
        let system_info = Self::collect_system_info()?;

        let metadata = default_metadata();
        let config = default_config();

        Ok(Self {
            metadata,
            config,
            system_info,
        })
    }

    /// 收集系统信息
    fn collect_system_info() -> Result<SystemInfo, InstallError> {
        // 检查是否为 root 用户
        let is_root = unsafe { libc::getuid() == 0 };

        // 检查 systemd 是否可用
        let has_systemd = which("systemctl").is_ok();

        // 检查是否存在 input 组
        let has_input_group = Command::new("getent")
            .arg("group")
            .arg("input")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

        Ok(SystemInfo {
            is_root,
            has_systemd,
            has_input_group,
        })
    }

    /// 获取 systemd 状态
    fn get_systemctl_status(&self) -> Result<(bool, bool), InstallError> {
        if !self.system_info.has_systemd {
            return Err(InstallError::SystemdNotAvailable);
        }

        let output = Command::new("systemctl")
            .arg("is-active")
            .arg("aegis-input")
            .output()
            .map_err(|e| InstallError::ServiceOperationFailed(e.to_string()))?;

        let running = output.status.success();
        let output = Command::new("systemctl")
            .arg("is-enabled")
            .arg("aegis-input")
            .output()
            .map_err(|e| InstallError::ServiceOperationFailed(e.to_string()))?;

        let enabled = output.status.success();

        Ok((running, enabled))
    }

    /// 确保 systemd 服务文件存在
    fn ensure_systemd_unit(&self) -> Result<(), InstallError> {
        let unit_path = self.metadata.unit_path.as_ref()
            .ok_or_else(|| InstallError::InstallationFailed("Unit path not set".to_string()))?;

        if !unit_path.exists() {
            std::fs::write(
                unit_path,
                include_str!("../../install/linux/aegis-input.service")
            ).map_err(|e| InstallError::InstallationFailed(format!("Failed to write unit file: {}", e)))?;

            // 设置正确的权限
            fs::set_permissions(
                unit_path,
                PermissionsExt::from_mode(0o644)
            ).map_err(|e| InstallError::InstallationFailed(format!("Failed to set permissions: {}", e)))?;
        }

        Ok(())
    }

    /// 确保系统用户存在
    fn ensure_system_user(&self) -> Result<(), InstallError> {
        if !self.system_info.is_root {
            return Err(InstallError::PermissionDenied);
        }

        // 检查用户是否存在
        let output = Command::new("id")
            .arg("-u")
            .arg(&self.config.user)
            .output()
            .map_err(|e| InstallError::InstallationFailed(format!("Failed to check user: {}", e)))?;

        if !output.status.success() {
            // 创建用户
            Command::new("useradd")
                .arg("--system")
                .arg("--no-create-home")
                .arg("--shell")
                .arg("/usr/sbin/nologin")
                .arg(&self.config.user)
                .output()
                .map_err(|e| InstallError::InstallationFailed(format!("Failed to create user: {}", e)))?;
        }

        // 添加到 input 组
        if !self.system_info.has_input_group {
            Command::new("groupadd")
                .arg("input")
                .output()
                .map_err(|e| InstallError::InstallationFailed(format!("Failed to create group: {}", e)))?;
        }

        Command::new("usermod")
            .arg("-aG")
            .arg("input")
            .arg(&self.config.user)
            .output()
            .map_err(|e| InstallError::InstallationFailed(format!("Failed to add user to group: {}", e)))?;

        Ok(())
    }

    /// 确保安装目录存在
    fn ensure_install_dir(&self) -> Result<(), InstallError> {
        let install_dir = self.metadata.install_path.parent()
            .ok_or_else(|| InstallError::InstallationFailed("Invalid install path".to_string()))?;

        fs::create_dir_all(install_dir)
            .map_err(|e| InstallError::InstallationFailed(format!("Failed to create install dir: {}", e)))?;

        // 设置正确的权限
        fs::set_permissions(
            install_dir,
            PermissionsExt::from_mode(0o755)
        ).map_err(|e| InstallError::InstallationFailed(format!("Failed to set install dir permissions: {}", e)))?;

        Ok(())
    }

    /// 保存安装元数据
    fn save_metadata(&self) -> Result<(), InstallError> {
        let metadata_dir = self.metadata.install_path.parent()
            .unwrap_or(Path::new("/"))
            .join("var/lib/aegis-input");

        fs::create_dir_all(&metadata_dir)
            .map_err(|e| InstallError::InstallationFailed(format!("Failed to create metadata dir: {}", e)))?;

        let metadata_path = metadata_dir.join("install.toml");
        let metadata_str = toml::to_string_pretty(&self.metadata)
            .map_err(|e| InstallError::InstallationFailed(format!("Failed to serialize metadata: {}", e)))?;

        fs::write(&metadata_path, metadata_str)
            .map_err(|e| InstallError::InstallationFailed(format!("Failed to write metadata: {}", e)))?;

        // 设置权限
        fs::set_permissions(
            &metadata_path,
            PermissionsExt::from_mode(0o644)
        ).map_err(|e| InstallError::InstallationFailed(format!("Failed to set metadata permissions: {}", e)))?;

        Ok(())
    }

    /// 删除安装元数据
    fn remove_metadata(&self) -> Result<(), InstallError> {
        let metadata_dir = self.metadata.install_path.parent()
            .unwrap_or(Path::new("/"))
            .join("var/lib/aegis-input");

        let metadata_path = metadata_dir.join("install.toml");
        if metadata_path.exists() {
            fs::remove_file(&metadata_path)
                .map_err(|e| InstallError::UninstallationFailed(format!("Failed to remove metadata: {}", e)))?;
        }

        // 如果目录为空，删除它
        if metadata_dir.exists() {
            if let Ok(entries) = fs::read_dir(&metadata_dir) {
                if entries.count() == 0 {
                    fs::remove_dir(&metadata_dir)
                        .map_err(|e| InstallError::UninstallationFailed(format!("Failed to remove metadata dir: {}", e)))?;
                }
            }
        }

        Ok(())
    }
}

impl Installer for LinuxInstaller {
    fn install(&self) -> Result<InstallResult, InstallError> {
        let start_time = std::time::Instant::now();

        // 前置检查
        if !self.system_info.is_root {
            return Err(InstallError::PermissionDenied);
        }

        if !self.system_info.has_systemd {
            return Err(InstallError::SystemdNotAvailable);
        }

        // 1. 确保安装目录存在
        self.ensure_install_dir()?;

        // 2. 确保系统用户存在
        self.ensure_system_user()?;

        // 3. 确保 systemd unit 文件存在
        self.ensure_systemd_unit()?;

        // 4. 保存安装元数据
        self.save_metadata()?;

        // 5. 启动并启用服务
        self.start_service()?;
        self.enable_service()?;

        let duration = start_time.elapsed().as_secs();

        Ok(InstallResult {
            success: true,
            error: None,
            duration_secs: duration,
        })
    }

    fn uninstall(&self) -> Result<InstallResult, InstallError> {
        let start_time = std::time::Instant::now();

        if !self.system_info.is_root {
            return Err(InstallError::PermissionDenied);
        }

        // 1. 停止并禁用服务
        self.stop_service()?;
        self.disable_service()?;

        // 2. 删除 systemd unit 文件
        if let Some(unit_path) = &self.metadata.unit_path {
            if unit_path.exists() {
                fs::remove_file(unit_path)
                    .map_err(|e| InstallError::UninstallationFailed(format!("Failed to remove unit file: {}", e)))?;
            }
        }

        // 3. 删除安装元数据
        self.remove_metadata()?;

        // 4. 删除系统用户
        Command::new("userdel")
            .arg(&self.config.user)
            .output()
            .map_err(|e| InstallError::UninstallationFailed(format!("Failed to delete user: {}", e)))?;

        let duration = start_time.elapsed().as_secs();

        Ok(InstallResult {
            success: true,
            error: None,
            duration_secs: duration,
        })
    }

    fn is_installed(&self) -> bool {
        self.metadata.unit_path.as_ref()
            .map(|p| p.exists())
            .unwrap_or(false) &&
        self.config.config_path.exists()
    }

    fn get_metadata(&self) -> Result<InstallMetadata, InstallError> {
        let metadata_path = Path::new("/var/lib/aegis-input/install.toml");
        if !metadata_path.exists() {
            return Err(InstallError::MetadataNotFound(metadata_path.to_path_buf()));
        }

        let content = fs::read_to_string(metadata_path)
            .map_err(|_e| InstallError::MetadataNotFound(metadata_path.to_path_buf()))?;

        let metadata: InstallMetadata = toml::from_str(&content)
            .map_err(|_e| InstallError::MetadataNotFound(metadata_path.to_path_buf()))?;

        Ok(metadata)
    }

    fn get_service_state(&self) -> Result<ServiceState, InstallError> {
        let (running, enabled) = self.get_systemctl_status()?;

        Ok(ServiceState {
            running,
            enabled,
            last_changed_at: chrono::Utc::now(),
        })
    }

    fn start_service(&self) -> Result<(), InstallError> {
        if !self.system_info.has_systemd {
            return Err(InstallError::SystemdNotAvailable);
        }

        Command::new("systemctl")
            .arg("start")
            .arg("aegis-input")
            .output()
            .map_err(|e| InstallError::ServiceOperationFailed(e.to_string()))?;

        Ok(())
    }

    fn stop_service(&self) -> Result<(), InstallError> {
        if !self.system_info.has_systemd {
            return Err(InstallError::SystemdNotAvailable);
        }

        Command::new("systemctl")
            .arg("stop")
            .arg("aegis-input")
            .output()
            .map_err(|e| InstallError::ServiceOperationFailed(e.to_string()))?;

        Ok(())
    }

    fn enable_service(&self) -> Result<(), InstallError> {
        if !self.system_info.has_systemd {
            return Err(InstallError::SystemdNotAvailable);
        }

        Command::new("systemctl")
            .arg("enable")
            .arg("aegis-input")
            .output()
            .map_err(|e| InstallError::ServiceOperationFailed(e.to_string()))?;

        Ok(())
    }

    fn disable_service(&self) -> Result<(), InstallError> {
        if !self.system_info.has_systemd {
            return Err(InstallError::SystemdNotAvailable);
        }

        Command::new("systemctl")
            .arg("disable")
            .arg("aegis-input")
            .output()
            .map_err(|e| InstallError::ServiceOperationFailed(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_installer_creation() {
        let installer = LinuxInstaller::new();
        assert!(installer.is_ok());
    }

    #[test]
    fn test_system_info_collection() {
        let system_info = LinuxInstaller::collect_system_info();
        // 测试应该能成功收集系统信息
        // 注意：在测试环境中可能没有 root 权限
        println!("System info: {:?}", system_info);
    }
}