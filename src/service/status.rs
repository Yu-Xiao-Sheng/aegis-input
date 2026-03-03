use crate::core::state::SystemState;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StatusSnapshot {
    pub enabled: bool,
    pub keyboard_external_count: usize,
    pub pointing_external_count: usize,
    pub keyboard_disabled: bool,
    pub pointing_disabled: bool,
}

impl StatusSnapshot {
    pub fn from_state(state: &SystemState) -> Self {
        Self {
            enabled: state.feature.enabled,
            keyboard_external_count: state.policy.keyboard_external_count,
            pointing_external_count: state.policy.pointing_external_count,
            keyboard_disabled: state.policy.keyboard_disabled,
            pointing_disabled: state.policy.pointing_disabled,
        }
    }

    pub fn load(path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}
