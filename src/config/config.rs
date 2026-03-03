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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub enabled: bool,
    pub device_rules: DeviceRules,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: false,
            device_rules: DeviceRules::default(),
        }
    }
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
