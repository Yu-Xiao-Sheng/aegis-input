use crate::core::state::DeviceType;
use std::collections::{HashMap, HashSet};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use tracing::warn;

pub struct EvdevDisabler {
    keyboard_paths: HashSet<PathBuf>,
    pointing_paths: HashSet<PathBuf>,
    keyboard_handles: HashMap<PathBuf, evdev::Device>,
    pointing_handles: HashMap<PathBuf, evdev::Device>,
}

impl Default for EvdevDisabler {
    fn default() -> Self {
        Self::new()
    }
}

impl EvdevDisabler {
    pub fn new() -> Self {
        Self {
            keyboard_paths: HashSet::new(),
            pointing_paths: HashSet::new(),
            keyboard_handles: HashMap::new(),
            pointing_handles: HashMap::new(),
        }
    }

    pub fn register_internal(&mut self, device_type: DeviceType, path: &Path) {
        match device_type {
            DeviceType::Keyboard => {
                self.keyboard_paths.insert(path.to_path_buf());
            }
            DeviceType::Pointing => {
                self.pointing_paths.insert(path.to_path_buf());
            }
        }
    }

    pub fn disable(&mut self, device_type: DeviceType) -> anyhow::Result<()> {
        match device_type {
            DeviceType::Keyboard => self.grab_keyboard(),
            DeviceType::Pointing => self.grab_pointing(),
        }
    }

    pub fn enable(&mut self, device_type: DeviceType) -> anyhow::Result<()> {
        match device_type {
            DeviceType::Keyboard => self.ungrab_keyboard(),
            DeviceType::Pointing => self.ungrab_pointing(),
        }
    }

    fn grab_keyboard(&mut self) -> anyhow::Result<()> {
        let paths: Vec<PathBuf> = self.keyboard_paths.iter().cloned().collect();
        for path in paths {
            if self.keyboard_handles.contains_key(&path) {
                continue;
            }
            let mut device = match evdev::Device::open(&path) {
                Ok(device) => device,
                Err(err) => {
                    warn!("打开设备失败: {:?} ({})", path, err);
                    continue;
                }
            };
            if let Err(err) = device.grab() {
                if should_skip_grab(&err) {
                    warn!("设备不支持抓取或权限不足，跳过: {:?} ({})", path, err);
                    continue;
                }
                return Err(err.into());
            }
            self.keyboard_handles.insert(path, device);
        }
        Ok(())
    }

    fn grab_pointing(&mut self) -> anyhow::Result<()> {
        let paths: Vec<PathBuf> = self.pointing_paths.iter().cloned().collect();
        for path in paths {
            if self.pointing_handles.contains_key(&path) {
                continue;
            }
            let mut device = match evdev::Device::open(&path) {
                Ok(device) => device,
                Err(err) => {
                    warn!("打开设备失败: {:?} ({})", path, err);
                    continue;
                }
            };
            if let Err(err) = device.grab() {
                if should_skip_grab(&err) {
                    warn!("设备不支持抓取或权限不足，跳过: {:?} ({})", path, err);
                    continue;
                }
                return Err(err.into());
            }
            self.pointing_handles.insert(path, device);
        }
        Ok(())
    }

    fn ungrab_keyboard(&mut self) -> anyhow::Result<()> {
        ungrab_all(&mut self.keyboard_handles)
    }

    fn ungrab_pointing(&mut self) -> anyhow::Result<()> {
        ungrab_all(&mut self.pointing_handles)
    }
}

fn ungrab_all(handles: &mut HashMap<PathBuf, evdev::Device>) -> anyhow::Result<()> {
    let mut failures = Vec::new();
    for (path, mut device) in handles.drain() {
        if let Err(err) = device.ungrab() {
            if should_skip_grab(&err) {
                warn!("设备不支持释放或权限不足，跳过: {:?} ({})", path, err);
                continue;
            }
            failures.push((path, err));
        }
    }
    if let Some((path, err)) = failures.into_iter().next() {
        return Err(anyhow::anyhow!("failed to ungrab {:?}: {}", path, err));
    }
    Ok(())
}

fn should_skip_grab(err: &std::io::Error) -> bool {
    matches!(err.kind(), ErrorKind::PermissionDenied) || matches!(err.raw_os_error(), Some(25))
}
