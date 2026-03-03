use aegis_input::core::policy::PolicyAction;
use aegis_input::core::state::DeviceEvent;
use aegis_input::platform::PlatformBackend;
use std::collections::VecDeque;

pub struct FakePlatform {
    scan_events: Vec<DeviceEvent>,
    events: VecDeque<DeviceEvent>,
    pub actions: Vec<PolicyAction>,
}

impl FakePlatform {
    pub fn new(scan_events: Vec<DeviceEvent>, events: Vec<DeviceEvent>) -> Self {
        Self {
            scan_events,
            events: VecDeque::from(events),
            actions: Vec::new(),
        }
    }
}

impl PlatformBackend for FakePlatform {
    fn scan_devices(&mut self) -> anyhow::Result<Vec<DeviceEvent>> {
        Ok(self.scan_events.clone())
    }

    fn next_event(&mut self) -> anyhow::Result<DeviceEvent> {
        self.events
            .pop_front()
            .ok_or_else(|| anyhow::anyhow!("no more events"))
    }

    fn apply_action(&mut self, action: PolicyAction) -> anyhow::Result<()> {
        self.actions.push(action);
        Ok(())
    }
}
