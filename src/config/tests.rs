#![cfg(test)]

use crate::config::{AppConfig, ConfigBuilder};
use std::fs;
use tempfile::tempdir;

#[test]
fn test_default_config() {
    let config = AppConfig::default();
    assert_eq!(config.logging.level, "off");
    assert_eq!(config.logging.format, "compact");
    assert!(config.logging.show_file);
    assert!(config.logging.show_line_numbers);
    assert!(config.logging.show_thread_ids);
    assert!(config.logging.show_target);
}

#[test]
fn test_config_builder_defaults() {
    let config = ConfigBuilder::new()
        .load_defaults()
        .build()
        .unwrap();

    assert_eq!(config.logging.level, "off");
}

#[test]
fn test_config_builder_from_file() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("test_config.toml");

    let config_content = r#"
[logging]
level = "debug"
format = "json"
show_file = false
"#;

    fs::write(&config_path, config_content).unwrap();

    let config = ConfigBuilder::new()
        .load_defaults()
        .load_from_file(&config_path)
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(config.logging.level, "debug");
    assert_eq!(config.logging.format, "json");
    assert!(!config.logging.show_file);
}

#[test]
fn test_config_builder_from_env() {
    std::env::set_var(env!("CARGO_PKG_NAME").to_string() + "_LOGGING_LEVEL", "info");

    let config = ConfigBuilder::new()
        .load_defaults()
        .load_from_env()
        .build()
        .unwrap();

    assert_eq!(config.logging.level, "info");

    std::env::remove_var(env!("CARGO_PKG_NAME").to_string() + "_LOGGING_LEVEL");
}

#[test]
fn test_config_builder_cli_args() {
    let config = ConfigBuilder::new()
        .load_defaults()
        .load_from_cli_args(2)
        .build()
        .unwrap();

    assert_eq!(config.logging.level, "info");
}

#[test]
fn test_config_merge() {
    let mut base_config = AppConfig::default();
    let mut override_config = AppConfig::default();
    override_config.logging.level = "debug".to_string();

    base_config.merge_with(override_config);

    assert_eq!(base_config.logging.level, "debug");
    // Other fields should remain default
    assert_eq!(base_config.logging.format, "compact");
}

#[test]
fn test_config_file_not_found() {
    let result = ConfigBuilder::new()
        .load_defaults()
        .load_from_file("/nonexistent/path/config.toml");

    assert!(result.is_err());
}

#[test]
fn test_invalid_config_file() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("invalid_config.toml");

    let invalid_content = r#"
[logging
level = "debug"
"#;

    fs::write(&config_path, invalid_content).unwrap();

    let result = ConfigBuilder::new()
        .load_defaults()
        .load_from_file(&config_path);

    assert!(result.is_err());
}
