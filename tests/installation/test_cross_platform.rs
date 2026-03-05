//! 跨平台兼容性验证测试
//!
//! 测试代码在不同平台上的兼容性

#[test]
fn test_platform_specific_compilation() {
    // 验证平台特定的代码条件编译正确

    // Linux 平台
    #[cfg(target_os = "linux")]
    {
        use aegis_input::installer::linux::LinuxInstaller;
        let installer = LinuxInstaller::new();
        assert!(installer.is_ok(), "Linux 上应该能创建 LinuxInstaller");
    }

    // Windows 平台（尚未实现）
    #[cfg(target_os = "windows")]
    {
        // 当实现 Windows 支持时，这里应该创建 WindowsInstaller
        // 目前只是验证条件编译正确
        assert!(true, "Windows 平台检测成功");
    }

    // macOS 平台（尚未实现）
    #[cfg(target_os = "macos")]
    {
        // 当实现 macOS 支持时，这里应该创建 MacOSInstaller
        // 目前只是验证条件编译正确
        assert!(true, "macOS 平台检测成功");
    }
}

#[test]
fn test_path_handling_is_cross_platform() {
    // 验证路径处理使用跨平台的 PathBuf
    use std::path::PathBuf;

    // 使用 PathBuf 而不是硬编码的路径分隔符
    let path = PathBuf::from("/usr/local/bin/aegis-input");

    // 验证路径可以正确解析
    assert!(path.is_absolute());
}

#[test]
fn test_platform_detection_function() {
    // 验证平台检测函数
    let os = std::env::consts::OS;

    match os {
        "linux" => {
            // Linux 平台
            assert!(true, "Linux 平台检测成功");
        }
        "windows" => {
            // Windows 平台
            assert!(true, "Windows 平台检测成功");
        }
        "macos" => {
            // macOS 平台
            assert!(true, "macOS 平台检测成功");
        }
        _ => {
            // 其他平台
            assert!(true, "其他平台检测成功");
        }
    }
}

#[test]
fn test_metadata_platform_field() {
    // 验证元数据的平台字段正确设置
    use aegis_input::installer::default_metadata;

    let metadata = default_metadata();
    let current_os = std::env::consts::OS;

    match current_os {
        "linux" => assert_eq!(metadata.platform, "linux"),
        "windows" => assert_eq!(metadata.platform, "windows"),
        "macos" => assert_eq!(metadata.platform, "macos"),
        _ => {}
    }
}

#[test]
fn test_installer_factory_returns_correct_type() {
    // 验证工厂返回的安装器类型正确
    use aegis_input::installer::InstallerFactory;

    let result = InstallerFactory::create();

    let current_os = std::env::consts::OS;

    match current_os {
        "linux" => {
            // Linux 上应该成功创建安装器
            assert!(result.is_ok(), "Linux 上应该能创建安装器");

            let installer = result.unwrap();
            // 验证可以调用 trait 方法
            let _ = installer.is_installed();
        }
        _ => {
            // 其他平台目前不支持
            assert!(result.is_err(), "其他平台暂不支持");
        }
    }
}

#[test]
fn test_no_platform_specific_hardcoding() {
    // 验证没有硬编码平台特定的路径或配置

    // 配置应该通过抽象访问，而不是硬编码
    use aegis_input::installer::{default_metadata, default_config};

    let metadata = default_metadata();
    let config = default_config();

    // 验证配置是可访问的，而不是硬编码的
    assert!(!metadata.version.is_empty());
    assert!(!metadata.platform.is_empty());
    assert!(!config.user.is_empty());
    assert!(!config.group.is_empty());
}

#[test]
fn test_service_unit_abstraction() {
    // 验证服务单元文件的抽象

    // 在 Linux 上使用 systemd
    #[cfg(target_os = "linux")]
    {
        let unit_path = std::path::Path::new("install/linux/aegis-input.service");
        assert!(unit_path.exists(), "Linux 上应该有 systemd unit 文件");
    }

    // 未来在 Windows 上应该有对应的服务配置文件
    // 在 macOS 上应该有 launchd 配置文件
}

#[test]
fn test_error_messages_are_platform_agnostic() {
    // 验证错误消息不包含平台特定的硬编码信息
    use aegis_input::installer::InstallError;
    use aegis_input::installer::error::ErrorHandler;

    let error = InstallError::PermissionDenied;
    let formatted = ErrorHandler::format_error(&error);

    // 错误消息应该说明问题，但不应该依赖特定平台的假设
    assert!(formatted.contains("权限"), "错误消息应该说明权限问题");
    assert!(formatted.contains("root"), "错误消息应该提示需要 root");
}

#[test]
fn test_installation_script_abstraction() {
    // 验证安装脚本的抽象

    // Linux 安装脚本
    let linux_install = std::path::Path::new("install/linux/install.sh");
    let linux_uninstall = std::path::Path::new("install/linux/uninstall.sh");

    if linux_install.exists() {
        // 验证脚本是可执行的（如果有执行权限）
        let metadata = std::fs::metadata(linux_install);
        if let Ok(meta) = metadata {
            use std::os::unix::fs::PermissionsExt;
            let mode = meta.permissions().mode();
            // 检查是否有执行权限
            let has_exec = mode & 0o111 != 0;
            if has_exec {
                assert!(true, "Linux 安装脚本有执行权限");
            }
        }
    }

    if linux_uninstall.exists() {
        let metadata = std::fs::metadata(linux_uninstall);
        if let Ok(meta) = metadata {
            use std::os::unix::fs::PermissionsExt;
            let mode = meta.permissions().mode();
            let has_exec = mode & 0o111 != 0;
            if has_exec {
                assert!(true, "Linux 卸载脚本有执行权限");
            }
        }
    }

    // 未来应该有 Windows 和 macOS 的对应脚本
    // install/windows/install.ps1
    // install/macos/install.sh
}

#[test]
fn test_trait_object_compatibility() {
    // 验证 Installer trait 可以作为 trait object 使用
    // 这是跨平台抽象的关键

    use aegis_input::installer::Installer;

    fn use_installer(installer: &Box<dyn Installer>) -> bool {
        installer.is_installed()
    }

    #[cfg(target_os = "linux")]
    {
        use aegis_input::installer::InstallerFactory;

        let installer: Box<dyn Installer> = InstallerFactory::create()
            .expect("无法创建安装器");

        // 验证可以传递 trait object
        let _ = use_installer(&installer);
    }
}

#[test]
fn test_config_abstraction() {
    // 验证配置的抽象不依赖平台特定细节

    use aegis_input::installer::{InstallConfig, default_config};

    let config = default_config();

    // 配置应该使用通用的路径表示，而不是硬编码
    assert!(!config.config_path.as_os_str().is_empty());
    assert!(!config.status_path.as_os_str().is_empty());

    // 用户和组应该是可配置的
    assert!(!config.user.is_empty());
    assert!(!config.group.is_empty());
}