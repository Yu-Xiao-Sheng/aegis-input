use crate::config::config::Config;
use crate::core::policy::PolicyAction;
use crate::core::state::SystemState;
use crate::platform::PlatformBackend;
use crate::service::status::StatusSnapshot;
use std::path::PathBuf;
use tracing::warn;

pub struct Runner<B: PlatformBackend> {
    backend: B,
    state: SystemState,
    status_path: Option<PathBuf>,
}

impl<B: PlatformBackend> Runner<B> {
    pub fn new(mut backend: B, enabled: bool) -> anyhow::Result<Self> {
        let mut state = SystemState::new(enabled);
        let devices = backend.scan_devices()?;
        for event in devices {
            let actions = state.apply_event(&event);
            apply_actions(&mut backend, actions)?;
        }
        Ok(Self {
            backend,
            state,
            status_path: None,
        })
    }

    pub fn with_status_path(mut self, path: PathBuf) -> Self {
        self.status_path = Some(path);
        if let Err(err) = self.persist_status() {
            warn!("状态写入失败，将禁用状态持久化: {}", err);
            self.status_path = None;
        }
        self
    }

    pub fn set_enabled(&mut self, enabled: bool) -> anyhow::Result<()> {
        let actions = self.state.set_enabled(enabled);
        apply_actions(&mut self.backend, actions)?;
        if let Err(err) = self.persist_status() {
            warn!("状态写入失败: {}", err);
        }
        Ok(())
    }

    pub fn run_loop(&mut self) -> anyhow::Result<()> {
        loop {
            let event = self.backend.next_event()?;
            let actions = self.state.apply_event(&event);
            apply_actions(&mut self.backend, actions)?;
            if let Err(err) = self.persist_status() {
                warn!("状态写入失败: {}", err);
            }
        }
    }

    pub fn process_next_event(&mut self) -> anyhow::Result<()> {
        let event = self.backend.next_event()?;
        let actions = self.state.apply_event(&event);
        apply_actions(&mut self.backend, actions)?;
        if let Err(err) = self.persist_status() {
            warn!("状态写入失败: {}", err);
        }
        Ok(())
    }

    pub fn apply_config(&mut self, config: &Config) -> anyhow::Result<()> {
        if self.state.feature.enabled != config.enabled {
            self.set_enabled(config.enabled)?;
        }
        Ok(())
    }

    fn persist_status(&self) -> anyhow::Result<()> {
        if let Some(path) = &self.status_path {
            let snapshot = StatusSnapshot::from_state(&self.state);
            snapshot.save(path)?;
        }
        Ok(())
    }

    pub fn backend(&self) -> &B {
        &self.backend
    }

    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }
}

fn apply_actions<B: PlatformBackend>(backend: &mut B, actions: Vec<PolicyAction>) -> anyhow::Result<()> {
    for action in actions {
        backend.apply_action(action)?;
    }
    Ok(())
}
