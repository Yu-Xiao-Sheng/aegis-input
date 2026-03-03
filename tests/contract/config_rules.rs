use aegis_input::config::config::{Config, DeviceRules};
use tempfile::tempdir;

#[test]
fn default_rules_when_config_missing() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("missing.toml");

    let config = Config::load_or_default(&path).expect("load default");
    assert!(!config.enabled);
    assert!(config.device_rules.external_buses.contains(&"usb".to_string()));
    assert!(config.device_rules.external_buses.contains(&"bluetooth".to_string()));
    assert!(config.device_rules.internal_buses.contains(&"i8042".to_string()));
}

#[test]
fn parse_rules_from_config_file() {
    let dir = tempdir().expect("temp dir");
    let path = dir.path().join("config.toml");

    let content = r#"
        enabled = true

        [device_rules]
        external_buses = ["usb"]
        internal_buses = ["i2c"]
    "#;

    std::fs::write(&path, content).expect("write config");

    let config = Config::load(&path).expect("load config");
    assert!(config.enabled);
    assert_eq!(
        config.device_rules,
        DeviceRules {
            external_buses: vec!["usb".to_string()],
            internal_buses: vec!["i2c".to_string()],
        }
    );
}
