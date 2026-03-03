use crate::core::state::DeviceType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyAction {
    Disable(DeviceType),
    Enable(DeviceType),
}
