use crate::core::policy::PolicyAction;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceType {
    Keyboard,
    Pointing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceOrigin {
    Internal,
    External,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeviceState {
    Connected,
    Disconnected,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceEvent {
    pub id: String,
    pub name: String,
    pub device_type: DeviceType,
    pub origin: DeviceOrigin,
    pub state: DeviceState,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureState {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisablePolicy {
    pub keyboard_external_count: usize,
    pub pointing_external_count: usize,
    pub keyboard_disabled: bool,
    pub pointing_disabled: bool,
}

#[derive(Debug, Clone, Default)]
struct DeviceRegistry {
    external_keyboards: HashSet<String>,
    external_pointing: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct SystemState {
    pub feature: FeatureState,
    pub policy: DisablePolicy,
    registry: DeviceRegistry,
}

impl SystemState {
    pub fn new(enabled: bool) -> Self {
        Self {
            feature: FeatureState { enabled },
            policy: DisablePolicy {
                keyboard_external_count: 0,
                pointing_external_count: 0,
                keyboard_disabled: false,
                pointing_disabled: false,
            },
            registry: DeviceRegistry::default(),
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) -> Vec<PolicyAction> {
        self.feature.enabled = enabled;
        let target_keyboard = enabled && self.policy.keyboard_external_count > 0;
        let target_pointing = enabled && self.policy.pointing_external_count > 0;
        self.compute_actions(target_keyboard, target_pointing)
    }

    pub fn apply_event(&mut self, event: &DeviceEvent) -> Vec<PolicyAction> {
        if event.origin == DeviceOrigin::External {
            match (event.device_type, event.state) {
                (DeviceType::Keyboard, DeviceState::Connected) => {
                    self.registry.external_keyboards.insert(event.id.clone());
                }
                (DeviceType::Keyboard, DeviceState::Disconnected) => {
                    self.registry.external_keyboards.remove(&event.id);
                }
                (DeviceType::Pointing, DeviceState::Connected) => {
                    self.registry.external_pointing.insert(event.id.clone());
                }
                (DeviceType::Pointing, DeviceState::Disconnected) => {
                    self.registry.external_pointing.remove(&event.id);
                }
            }

            self.policy.keyboard_external_count = self.registry.external_keyboards.len();
            self.policy.pointing_external_count = self.registry.external_pointing.len();
        }

        let target_keyboard = self.feature.enabled && self.policy.keyboard_external_count > 0;
        let target_pointing = self.feature.enabled && self.policy.pointing_external_count > 0;
        self.compute_actions(target_keyboard, target_pointing)
    }

    fn compute_actions(
        &mut self,
        target_keyboard: bool,
        target_pointing: bool,
    ) -> Vec<PolicyAction> {
        let mut actions = Vec::new();

        if target_keyboard != self.policy.keyboard_disabled {
            self.policy.keyboard_disabled = target_keyboard;
            actions.push(if target_keyboard {
                PolicyAction::Disable(DeviceType::Keyboard)
            } else {
                PolicyAction::Enable(DeviceType::Keyboard)
            });
        }

        if target_pointing != self.policy.pointing_disabled {
            self.policy.pointing_disabled = target_pointing;
            actions.push(if target_pointing {
                PolicyAction::Disable(DeviceType::Pointing)
            } else {
                PolicyAction::Enable(DeviceType::Pointing)
            });
        }

        actions
    }
}
