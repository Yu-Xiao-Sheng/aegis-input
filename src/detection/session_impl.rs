//! Linux 平台的检测会话实现
//!
//! 实现交互式设备检测会话

use anyhow::Result;
use crate::detection::{
    DetectionSession, SessionStartInfo, SessionResult, CompletionReason,
    SessionState, InputDevice,
};
use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Duration;
use tokio::signal;

/// Linux 检测会话
pub struct LinuxDetectionSession {
    session_id: String,
    start_time: chrono::DateTime<chrono::Utc>,
    all_devices: Vec<InputDevice>,
    active_devices: HashSet<PathBuf>,
    state: SessionState,
    timeout_secs: u64,
}

impl LinuxDetectionSession {
    /// 创建新的检测会话
    pub fn new(devices: Vec<InputDevice>, timeout_secs: u64) -> Self {
        let session_id = format!("detect-{}", chrono::Utc::now().format("%Y%m%d-%H%M%S"));

        Self {
            session_id,
            start_time: chrono::Utc::now(),
            all_devices: devices,
            active_devices: HashSet::new(),
            state: SessionState::Initializing,
            timeout_secs,
        }
    }
}

#[async_trait::async_trait]
impl DetectionSession for LinuxDetectionSession {
    async fn start(&mut self) -> Result<SessionStartInfo> {
        self.state = SessionState::Running;

        Ok(SessionStartInfo {
            session_id: self.session_id.clone(),
            all_devices: self.all_devices.clone(),
            start_time: self.start_time,
        })
    }

    async fn wait(&mut self) -> Result<SessionResult> {
        // 等待 Ctrl+C 或超时
        let timeout = Duration::from_secs(self.timeout_secs);

        tokio::select! {
            // 等待用户中断（Ctrl+C）
            _ = signal::ctrl_c() => {
                self.state = SessionState::Completed;
                Ok(SessionResult {
                    session_id: self.session_id.clone(),
                    end_time: chrono::Utc::now(),
                    active_devices: self.active_devices.clone(),
                    duration: {
                        let delta = chrono::Utc::now() - self.start_time;
                        delta.to_std().unwrap_or(Duration::from_secs(0))
                    },
                    completion_reason: CompletionReason::UserInterrupt,
                })
            }

            // 等待超时
            _ = tokio::time::sleep(timeout) => {
                self.state = SessionState::Completed;
                Ok(SessionResult {
                    session_id: self.session_id.clone(),
                    end_time: chrono::Utc::now(),
                    active_devices: self.active_devices.clone(),
                    duration: {
                        let delta = chrono::Utc::now() - self.start_time;
                        delta.to_std().unwrap_or(Duration::from_secs(0))
                    },
                    completion_reason: CompletionReason::Timeout,
                })
            }
        }
    }

    fn active_devices(&self) -> HashSet<PathBuf> {
        self.active_devices.clone()
    }

    async fn cancel(&mut self) -> Result<()> {
        self.state = SessionState::Failed;
        Ok(())
    }

    async fn complete(&mut self, selection: HashSet<PathBuf>) -> Result<crate::config::DeviceConfiguration> {
        // 创建配置
        let disabled_devices = selection
            .into_iter()
            .map(|path| crate::config::DeviceRef {
                path: Some(path),
                name: None,
                verified_at: Some(chrono::Utc::now()),
            })
            .collect();

        Ok(crate::config::DeviceConfiguration {
            version: "1.0".to_string(),
            disabled_devices,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }
}
