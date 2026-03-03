use crate::config::config::{config_path, status_path, Config};
use crate::platform::linux::udev::LinuxBackend;
use crate::service::logging::init_logging;
use crate::service::runner::Runner;

pub fn run_service() -> anyhow::Result<()> {
    init_logging();
    let path = config_path();
    let config = Config::load_or_default(&path)?;

    let backend = LinuxBackend::new(config.device_rules.clone())?;
    let status_path = status_path();
    let mut runner = Runner::new(backend, config.enabled)?.with_status_path(status_path);
    runner.run_loop()
}
