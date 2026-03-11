use crate::config::DeviceRules;
use crate::core::policy::PolicyAction;
use crate::core::state::{DeviceEvent, DeviceOrigin, DeviceState, DeviceType};
use crate::platform::PlatformBackend;
use std::ffi::OsStr;
use std::path::PathBuf;

use super::evdev::EvdevDisabler;

pub struct LinuxBackend {
    rules: DeviceRules,
    monitor: udev::MonitorSocket,
    disabler: EvdevDisabler,
}

impl LinuxBackend {
    pub fn new(rules: DeviceRules) -> anyhow::Result<Self> {
        let monitor = udev::MonitorBuilder::new()?
            .match_subsystem("input")?
            .listen()?;

        Ok(Self {
            rules,
            monitor,
            disabler: EvdevDisabler::new(),
        })
    }

    fn device_event_from_udev(
        &self,
        device: &udev::Device,
        state: DeviceState,
    ) -> Option<DeviceEvent> {
        let device_type = detect_device_type(device)?;
        let origin = detect_device_origin(device, &self.rules);
        let devnode = device.devnode()?;
        let id = devnode.to_string_lossy().to_string();
        if !id.starts_with("/dev/input/event") {
            return None;
        }
        let name = device
            .property_value("NAME")
            .and_then(|v| v.to_str())
            .unwrap_or("unknown")
            .to_string();

        Some(DeviceEvent {
            id,
            name,
            device_type,
            origin,
            state,
        })
    }

    fn register_internal_device(&mut self, event: &DeviceEvent) {
        if event.origin != DeviceOrigin::Internal || event.state != DeviceState::Connected {
            return;
        }
        let path = PathBuf::from(&event.id);
        let path = path.canonicalize().unwrap_or(path);
        self.disabler.register_internal(event.device_type, &path);
    }
}

impl PlatformBackend for LinuxBackend {
    fn scan_devices(&mut self) -> anyhow::Result<Vec<DeviceEvent>> {
        let mut enumerator = udev::Enumerator::new()?;
        enumerator.match_subsystem("input")?;
        let mut events = Vec::new();

        for device in enumerator.scan_devices()? {
            if let Some(event) = self.device_event_from_udev(&device, DeviceState::Connected) {
                self.register_internal_device(&event);
                events.push(event);
            }
        }

        Ok(events)
    }

    fn next_event(&mut self) -> anyhow::Result<DeviceEvent> {
        loop {
            for event in self.monitor.iter() {
                let state = match event.event_type() {
                    udev::EventType::Add => DeviceState::Connected,
                    udev::EventType::Remove => DeviceState::Disconnected,
                    _ => continue,
                };

                if let Some(device_event) = self.device_event_from_udev(&event.device(), state) {
                    if state == DeviceState::Connected {
                        self.register_internal_device(&device_event);
                    }
                    return Ok(device_event);
                }
            }

            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }

    fn apply_action(&mut self, action: PolicyAction) -> anyhow::Result<()> {
        match action {
            PolicyAction::Disable(device_type) => self.disabler.disable(device_type),
            PolicyAction::Enable(device_type) => self.disabler.enable(device_type),
        }
    }
}

fn detect_device_type(device: &udev::Device) -> Option<DeviceType> {
    if prop_is_true(device, "ID_INPUT_KEYBOARD") {
        return Some(DeviceType::Keyboard);
    }

    if prop_is_true(device, "ID_INPUT_MOUSE")
        || prop_is_true(device, "ID_INPUT_TOUCHPAD")
        || prop_is_true(device, "ID_INPUT_TRACKPOINT")
    {
        return Some(DeviceType::Pointing);
    }

    None
}

fn detect_device_origin(device: &udev::Device, rules: &DeviceRules) -> DeviceOrigin {
    let bus = device.property_value("ID_BUS");
    if matches_bus(bus, &rules.internal_buses) {
        return DeviceOrigin::Internal;
    }
    if matches_bus(bus, &rules.external_buses) {
        return DeviceOrigin::External;
    }
    DeviceOrigin::Internal
}

fn matches_bus(value: Option<&OsStr>, list: &[String]) -> bool {
    let Some(value) = value.and_then(|v| v.to_str()) else {
        return false;
    };
    list.iter().any(|bus| bus.eq_ignore_ascii_case(value))
}

fn prop_is_true(device: &udev::Device, key: &str) -> bool {
    matches!(
        device.property_value(key).and_then(|v| v.to_str()),
        Some("1")
    )
}
