//! CLI 设备选择器实现
//!
//! 实现交互式设备选择

use crate::detection::{DeviceSelector, InputDevice};
use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Write};

pub struct CliDeviceSelector;

impl DeviceSelector for CliDeviceSelector {
    fn prompt_selection(
        &self,
        all_devices: &[InputDevice],
        active_devices: &HashSet<std::path::PathBuf>,
    ) -> Result<HashSet<std::path::PathBuf>> {
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("  设备选择");
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        // 显示活跃设备
        if !active_devices.is_empty() {
            println!("检测期间活跃的设备:\n");
            for (i, device) in all_devices.iter().enumerate() {
                if active_devices.contains(&device.path) {
                    println!("  ✓ [{}] {}", i + 1, device.name);
                }
            }
            println!();
        }

        // 显示未活跃设备
        let inactive_devices: Vec<_> = all_devices
            .iter()
            .filter(|d| !active_devices.contains(&d.path))
            .collect();

        if !inactive_devices.is_empty() {
            println!("未活跃的设备:\n");
            for device in inactive_devices {
                let idx = all_devices
                    .iter()
                    .position(|d| d.path == device.path)
                    .unwrap();
                println!("    [{}] {}", idx + 1, device.name);
            }
            println!();
        }

        // 提示用户输入
        println!("要禁用哪些设备？（可多选）");
        println!("  输入编号，如: 1,3");
        println!("  输入 'all' 禁用所有活跃设备");
        println!("  输入 'none' 跳过\n");

        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let input = input.trim();

        // 解析输入
        if input == "none" {
            return Ok(HashSet::new());
        }

        if input == "all" {
            return Ok(active_devices.clone());
        }

        // 解析编号列表
        let selected_indices = parse_indices(input)?;
        let mut selected_devices = HashSet::new();

        for idx in selected_indices {
            if let Some(device) = all_devices.get(idx) {
                selected_devices.insert(device.path.clone());
            }
        }

        Ok(selected_devices)
    }

    fn validate_selection(
        &self,
        selection: &HashSet<std::path::PathBuf>,
        available: &[InputDevice],
    ) -> Result<()> {
        let available_paths: HashSet<_> = available.iter().map(|d| &d.path).collect();

        for path in selection {
            if !available_paths.contains(path) {
                return Err(anyhow::anyhow!("无效的设备选择: {:?}", path));
            }
        }

        Ok(())
    }
}

/// 解析输入的编号列表
fn parse_indices(input: &str) -> Result<Vec<usize>> {
    let mut indices = Vec::new();

    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let idx: usize = part.parse()?;
        if idx == 0 {
            return Err(anyhow::anyhow!("设备编号从 1 开始"));
        }
        indices.push(idx - 1); // 转换为 0-based
    }

    Ok(indices)
}
