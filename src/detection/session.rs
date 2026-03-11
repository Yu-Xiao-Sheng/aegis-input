//! 检测会话管理
//!
//! 定义检测会话的核心接口和实现

use anyhow::Result;
use std::collections::HashSet;
use std::path::PathBuf;

/// 检测会话管理接口
#[async_trait::async_trait]
pub trait DetectionSession: Send + Sync {
    /// 启动检测会话
    async fn start(&mut self) -> Result<SessionStartInfo>;

    /// 等待会话完成（用户结束或超时）
    async fn wait(&mut self) -> Result<SessionResult>;

    /// 获取当前活跃的设备列表
    fn active_devices(&self) -> HashSet<PathBuf>;

    /// 取消会话
    async fn cancel(&mut self) -> Result<()>;

    /// 完成会话并生成配置
    async fn complete(&mut self, selection: HashSet<PathBuf>) -> Result<crate::config::DeviceConfiguration>;
}

/// 会话启动信息
#[derive(Debug, Clone)]
pub struct SessionStartInfo {
    pub session_id: String,
    pub all_devices: Vec<super::InputDevice>,
    pub start_time: chrono::DateTime<chrono::Utc>,
}

/// 会话结果
#[derive(Debug, Clone)]
pub struct SessionResult {
    pub session_id: String,
    pub end_time: chrono::DateTime<chrono::Utc>,
    pub active_devices: HashSet<PathBuf>,
    pub duration: std::time::Duration,
    pub completion_reason: CompletionReason,
}

/// 完成原因
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionReason {
    /// 用户中断（Ctrl+C）
    UserInterrupt,
    /// 超时
    Timeout,
    /// 错误
    Error,
}

/// 会话状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    /// 初始化中
    Initializing,
    /// 运行中
    Running,
    /// 用户选择中
    UserSelection,
    /// 已完成
    Completed,
    /// 失败
    Failed,
}
