use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceRules {
    pub external_buses: Vec<String>,
    pub internal_buses: Vec<String>,
}

impl Default for DeviceRules {
    fn default() -> Self {
        Self {
            external_buses: vec!["usb".into(), "bluetooth".into()],
            internal_buses: vec!["i8042".into(), "serio".into(), "platform".into(), "i2c".into()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct Config {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub device_rules: DeviceRules,
    /// 版本信息（新增，用于向后兼容）
    #[serde(default = "default_version")]
    pub version: String,
    /// 设备级别配置（新增）
    #[serde(default)]
    pub devices_specific: DevicesSpecific,
}

fn default_version() -> String {
    "0.1".to_string()
}

/// 设备级别配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DevicesSpecific {
    /// 要禁用的特定设备列表
    #[serde(default)]
    pub disabled: Vec<DeviceRef>,
}

impl Config {
    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn load_or_default(path: &Path) -> anyhow::Result<Self> {
        match Self::load(path) {
            Ok(config) => Ok(config),
            Err(err) if is_not_found(&err) => Ok(Self::default()),
            Err(err) => Err(err),
        }
    }

    pub fn load_with_fallback(path: &Path, current: &Config) -> Config {
        match Self::load(path) {
            Ok(config) => config,
            Err(_) => current.clone(),
        }
    }

    /// 升级旧配置到新格式
    pub fn upgrade(&mut self) {
        // 如果版本小于 1.0，升级到 1.0
        if self.version == "0.1" {
            self.version = "1.0".to_string();
            // 保留旧配置，devices_specific 使用默认值（空列表）
        }
    }
}

pub fn config_path() -> PathBuf {
    if let Ok(path) = env::var("AEGIS_INPUT_CONFIG") {
        return PathBuf::from(path);
    }
    let base = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("aegis-input").join("config.toml")
}

pub fn status_path() -> PathBuf {
    if let Ok(path) = env::var("AEGIS_INPUT_STATUS") {
        return PathBuf::from(path);
    }
    let base = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from("."));
    base.join("aegis-input").join("status.toml")
}

fn is_not_found(err: &anyhow::Error) -> bool {
    err.downcast_ref::<io::Error>()
        .map(|e| e.kind() == io::ErrorKind::NotFound)
        .unwrap_or(false)
}

/// 设备配置
///
/// 表示用户选择要禁用的设备配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceConfiguration {
    /// 配置版本
    pub version: String,
    /// 要禁用的设备引用列表
    pub disabled_devices: Vec<DeviceRef>,
    /// 配置创建时间
    pub created_at: chrono::DateTime<chrono::Utc>,
    /// 配置更新时间
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// 设备引用
///
/// 用于配置文件中引用特定设备
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeviceRef {
    /// 设备路径（优先匹配）
    pub path: Option<PathBuf>,
    /// 设备名称（后备匹配）
    pub name: Option<String>,
    /// 验证时间（用于警告）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_at: Option<chrono::DateTime<chrono::Utc>>,
}

impl DeviceRef {
    /// 创建路径引用
    pub fn from_path(path: PathBuf) -> Self {
        Self {
            path: Some(path),
            name: None,
            verified_at: None,
        }
    }

    /// 创建名称引用
    pub fn from_name(name: String) -> Self {
        Self {
            path: None,
            name: Some(name),
            verified_at: None,
        }
    }

    /// 匹配设备
    pub fn matches(&self, device: &crate::detection::InputDevice) -> bool {
        // 路径匹配（优先）
        if let Some(ref path) = self.path {
            if &device.path == path {
                return true;
            }
        }

        // 名称匹配（后备）
        if let Some(ref name) = self.name {
            if &device.name == name {
                return true;
            }
        }

        false
    }
}

/// 配置验证器
pub struct ConfigValidator {
    max_disabled_devices: usize,
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self {
            max_disabled_devices: 10,
        }
    }
}

impl ConfigValidator {
    pub fn new(max_disabled_devices: usize) -> Self {
        Self {
            max_disabled_devices,
        }
    }

    /// 验证配置
    pub fn validate(&self, config: &DeviceConfiguration) -> anyhow::Result<()> {
        // 检查设备数量
        if config.disabled_devices.len() > self.max_disabled_devices {
            return Err(anyhow::anyhow!(
                "禁用设备数量超过限制（{} > {}）",
                config.disabled_devices.len(),
                self.max_disabled_devices
            ));
        }

        // 检查每个设备引用
        for device_ref in &config.disabled_devices {
            // 至少有路径或名称之一
            if device_ref.path.is_none() && device_ref.name.is_none() {
                return Err(anyhow::anyhow!(
                    "设备引用无效：必须提供 path 或 name"
                ));
            }
        }

        Ok(())
    }

    /// 验证并解析设备引用
    pub fn resolve_device_refs(
        &self,
        config: &DeviceConfiguration,
        available_devices: &[crate::detection::InputDevice],
    ) -> Vec<PathBuf> {
        let mut resolved = Vec::new();

        for device_ref in &config.disabled_devices {
            for device in available_devices {
                if device_ref.matches(device) {
                    if !resolved.contains(&device.path) {
                        resolved.push(device.path.clone());
                    }
                }
            }
        }

        resolved
    }
}
