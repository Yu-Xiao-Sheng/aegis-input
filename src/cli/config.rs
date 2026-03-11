//! Config 子命令
//!
//! 配置管理

use anyhow::Result;
use std::io::Write;
use std::path::PathBuf;

/// 运行 config 命令
pub async fn run(args: ConfigArgs) -> Result<i32> {
    if args.reset {
        return reset_config();
    }

    // TODO: 实现其他 config 操作
    println!("配置命令");
    Ok(0)
}

/// Config 子命令参数
#[derive(Debug, Clone)]
pub struct ConfigArgs {
    pub reset: bool,
}

/// 重置配置
fn reset_config() -> Result<i32> {
    let config_path = crate::config::config_path();

    if config_path.exists() {
        println!("这将删除当前配置。");
        print!("确定要继续吗? (y/N): ");
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().to_lowercase().starts_with('y') {
            println!("已取消");
            return Ok(0);
        }
    }

    // 删除配置文件
    if config_path.exists() {
        std::fs::remove_file(&config_path)?;
        println!("✓ 配置已删除");
    }

    // 删除状态文件
    let status_path = crate::config::status_path();
    if status_path.exists() {
        std::fs::remove_file(&status_path)?;
        println!("✓ 状态已删除");
    }

    println!();
    println!("配置已重置为默认状态。");
    println!("要重新配置，请运行: aegis-input detect");

    Ok(0)
}
