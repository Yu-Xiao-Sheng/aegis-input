use crate::config::{config_path, status_path, Config};
use crate::service::status::StatusSnapshot;
use std::env;

pub fn run() -> anyhow::Result<i32> {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("");
    match cmd {
        "detect" => {
            // 使用 tokio 运行时运行异步命令
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                // 简单参数解析（TODO: 使用 clap）
                let args = crate::cli::detect::DetectArgs {
                    timeout: 300,
                    output: crate::cli::detect::OutputFormat::Auto,
                    config_only: false,
                };
                crate::cli::detect::run(args).await
            })
        }
        "config" => {
            let rt = tokio::runtime::Runtime::new()?;
            rt.block_on(async {
                // 简单参数解析
                let reset = args.get(2).map(|s| s == "reset").unwrap_or(false);
                let args = crate::cli::config::ConfigArgs { reset };
                crate::cli::config::run(args).await
            })
        }
        "enable" => {
            let path = config_path();
            let mut config = Config::load_or_default(&path)?;
            config.enabled = true;
            config.save(&path)?;
            persist_status(true);
            println!("enabled=true");
            Ok(0)
        }
        "disable" => {
            let path = config_path();
            let mut config = Config::load_or_default(&path)?;
            config.enabled = false;
            config.save(&path)?;
            persist_status(false);
            println!("enabled=false");
            Ok(0)
        }
        "status" => {
            let status = load_status_or_default()?;
            println!("enabled={}", status.enabled);
            println!("keyboard_external_count={}", status.keyboard_external_count);
            println!("pointing_external_count={}", status.pointing_external_count);
            println!("keyboard_disabled={}", status.keyboard_disabled);
            println!("pointing_disabled={}", status.pointing_disabled);
            Ok(0)
        }
        "run" => {
            crate::service::main::run_service()?;
            Ok(0)
        }
        _ => {
            print_usage();
            Ok(1)
        }
    }
}

fn print_usage() {
    eprintln!("Usage: aegis-input <enable|disable|status|run|detect|config>");
}

fn load_status_or_default() -> anyhow::Result<StatusSnapshot> {
    let path = status_path();
    match StatusSnapshot::load(&path) {
        Ok(snapshot) => Ok(snapshot),
        Err(_) => {
            let config = Config::load_or_default(&config_path())?;
            Ok(StatusSnapshot {
                enabled: config.enabled,
                keyboard_external_count: 0,
                pointing_external_count: 0,
                keyboard_disabled: false,
                pointing_disabled: false,
            })
        }
    }
}

fn persist_status(enabled: bool) {
    let snapshot = StatusSnapshot {
        enabled,
        keyboard_external_count: 0,
        pointing_external_count: 0,
        keyboard_disabled: false,
        pointing_disabled: false,
    };
    let path = status_path();
    let _ = snapshot.save(&path);
}
