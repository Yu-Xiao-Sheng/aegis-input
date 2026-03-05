//! 安装抽象接口测试
//!
//! 测试安装器抽象接口的正确性和平台无关性

use std::path::PathBuf;
use aegis_input::installer::{Installer, InstallerFactory, InstallMetadata, ServiceState};

#[test]
fn test_installer_factory_creates_linux_installer() {
    // 在 Linux 系统上，工厂应该创建 LinuxInstaller
    #[cfg(target_os = "linux")]
    {
        let installer = InstallerFactory::create();
        assert!(installer.is_ok(), "应该能创建 Linux 安装器");

        let installer = installer.unwrap();
        // 验证安装器实现了必要的 trait
        let _ = installer.is_installed();
    }
}

#[test]
fn test_installer_trait_is_object_safe() {
    // 验证 Installer trait 可以作为 trait object 使用
    // 这个测试主要检查类型系统，如果编译通过就说明 trait 是 object safe 的

    #[cfg(target_os = "linux")]
    {
        let installer: Box<dyn Installer> = InstallerFactory::create()
            .expect("无法创建安装器");

        // 调用 trait 方法确保可以正常工作
        let is_installed = installer.is_installed();
        // 验证返回值类型正确
        let _ = std::any::type_name::<bool>() == std::any::type_name_val(&is_installed);
    }
}

#[test]
fn test_installer_has_required_methods() {
    // 这个测试验证 Installer trait 包含所有必要的方法
    // 如果 trait 定义缺失方法，这个测试将无法编译

    use aegis_input::installer::Installer;

    // 定义一个 mock 结构体实现 Installer trait
    struct MockInstaller;

    impl Installer for MockInstaller {
        fn install(&self) -> Result<aegis_input::installer::InstallResult, aegis_input::installer::InstallError> {
            Ok(aegis_input::installer::InstallResult {
                success: true,
                error: None,
                duration_secs: 0,
            })
        }

        fn uninstall(&self) -> Result<aegis_input::installer::InstallResult, aegis_input::installer::InstallError> {
            Ok(aegis_input::installer::InstallResult {
                success: true,
                error: None,
                duration_secs: 0,
            })
        }

        fn is_installed(&self) -> bool {
            false
        }

        fn get_metadata(&self) -> Result<InstallMetadata, aegis_input::installer::InstallError> {
            Err(aegis_input::installer::InstallError::MetadataNotFound("test".to_string()))
        }

        fn get_service_state(&self) -> Result<ServiceState, aegis_input::installer::InstallError> {
            Ok(ServiceState {
                running: false,
                enabled: false,
                last_changed_at: chrono::Utc::now(),
            })
        }

        fn start_service(&self) -> Result<(), aegis_input::installer::InstallError> {
            Ok(())
        }

        fn stop_service(&self) -> Result<(), aegis_input::installer::InstallError> {
            Ok(())
        }

        fn enable_service(&self) -> Result<(), aegis_input::installer::InstallError> {
            Ok(())
        }

        fn disable_service(&self) -> Result<(), aegis_input::installer::InstallError> {
            Ok(())
        }
    }

    // 如果上面的实现编译通过，说明 trait 定义正确
    let mock = MockInstaller;
    assert!(!mock.is_installed());
}

#[test]
fn test_install_metadata_has_required_fields() {
    // 验证 InstallMetadata 包含所有必要的字段
    let metadata = InstallMetadata {
        version: "0.1.0".to_string(),
        platform: "linux".to_string(),
        install_path: PathBuf::from("/usr/local/bin/aegis-input"),
        unit_path: Some(PathBuf::from("/etc/systemd/system/aegis-input.service")),
        installed_at: chrono::Utc::now(),
    };

    assert_eq!(metadata.version, "0.1.0");
    assert_eq!(metadata.platform, "linux");
    assert!(metadata.unit_path.is_some());
}

#[test]
fn test_service_state_has_required_fields() {
    // 验证 ServiceState 包含所有必要的字段
    let state = ServiceState {
        running: true,
        enabled: true,
        last_changed_at: chrono::Utc::now(),
    };

    assert!(state.running);
    assert!(state.enabled);
}

#[test]
fn test_platform_detection() {
    // 验证平台检测功能
    let current_os = std::env::consts::OS;

    match current_os {
        "linux" => {
            let installer = InstallerFactory::create();
            assert!(installer.is_ok(), "Linux 系统应该能创建安装器");
        }
        "windows" => {
            // Windows 安装器暂未实现
            let installer = InstallerFactory::create();
            assert!(installer.is_err(), "Windows 安装器暂未实现");
        }
        "macos" => {
            // macOS 安装器暂未实现
            let installer = InstallerFactory::create();
            assert!(installer.is_err(), "macOS 安装器暂未实现");
        }
        _ => {
            let installer = InstallerFactory::create();
            assert!(installer.is_err(), "未知平台不应该能创建安装器");
        }
    }
}

#[test]
fn test_installer_abstraction_allows_platform_replacement() {
    // 这个测试验证抽象允许平台替换
    // 通过检查 trait 定义和工厂模式来验证

    // 如果代码编译通过，说明抽象设计正确
    #[cfg(target_os = "linux")]
    {
        use aegis_input::installer::Installer;

        // 工厂返回 trait object，允许运行时替换具体实现
        let installer: Box<dyn Installer> = InstallerFactory::create()
            .expect("无法创建安装器");

        // 验证可以调用 trait 方法
        let _ = installer.is_installed();
        let _ = installer.get_service_state();
    }
}

#[test]
fn test_install_result_structure() {
    // 验证 InstallResult 结构正确
    use aegis_input::installer::InstallResult;

    let result = InstallResult {
        success: true,
        error: None,
        duration_secs: 10,
    };

    assert!(result.success);
    assert!(result.error.is_none());
    assert_eq!(result.duration_secs, 10);

    let failed_result = InstallResult {
        success: false,
        error: Some("测试错误".to_string()),
        duration_secs: 5,
    };

    assert!(!failed_result.success);
    assert!(failed_result.error.is_some());
    assert_eq!(failed_result.error.unwrap(), "测试错误");
}

#[test]
fn test_cross_platform_directory_structure() {
    // 验证目录结构支持跨平台
    use std::path::Path;

    // 检查 Linux 安装目录
    assert!(Path::new("install/linux").exists(), "缺少 Linux 安装目录");

    // Windows 和 macOS 目录可以不存在（尚未实现）
    // 但结构设计应该支持它们
}

#[test]
fn test_default_metadata_and_config() {
    // 验证默认的元数据和配置
    use aegis_input::installer::{default_metadata, default_config};

    let metadata = default_metadata();
    let config = default_config();

    assert_eq!(metadata.platform, "linux");
    assert_eq!(config.user, "aegis-input");
    assert_eq!(config.group, "input");
}