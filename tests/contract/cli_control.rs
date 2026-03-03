use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;
use tempfile::tempdir;

#[test]
fn cli_enable_disable_status() {
    let dir = tempdir().expect("temp dir");
    let config_path = dir.path().join("config.toml");

    cargo_bin_cmd!("aegis-input")
        .env("AEGIS_INPUT_CONFIG", &config_path)
        .arg("status")
        .assert()
        .success()
        .stdout(contains("enabled=false"));

    cargo_bin_cmd!("aegis-input")
        .env("AEGIS_INPUT_CONFIG", &config_path)
        .arg("enable")
        .assert()
        .success()
        .stdout(contains("enabled=true"));

    cargo_bin_cmd!("aegis-input")
        .env("AEGIS_INPUT_CONFIG", &config_path)
        .arg("status")
        .assert()
        .success()
        .stdout(contains("enabled=true"));

    cargo_bin_cmd!("aegis-input")
        .env("AEGIS_INPUT_CONFIG", &config_path)
        .arg("disable")
        .assert()
        .success()
        .stdout(contains("enabled=false"));
}
