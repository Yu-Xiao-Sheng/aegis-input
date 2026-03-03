pub mod linux;

use crate::core::policy::PolicyAction;
use crate::core::state::DeviceEvent;

pub trait PlatformBackend {
    fn scan_devices(&mut self) -> anyhow::Result<Vec<DeviceEvent>>;
    fn next_event(&mut self) -> anyhow::Result<DeviceEvent>;
    fn apply_action(&mut self, action: PolicyAction) -> anyhow::Result<()>;
}
