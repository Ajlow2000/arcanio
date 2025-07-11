use config::{Config, Environment, File, FileFormat};
use std::fmt::Debug;
use std::path::Path;

use crate::config::{defaults::AppConfig, paths::ConfigPaths};
use crate::Result;

#[derive(Debug)]
pub struct ConfigBuilder {
    config: Config,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: Config::builder().build().unwrap_or_default(),
        }
    }

    pub fn load_defaults(mut self) -> Self {
        let defaults = AppConfig::default();
        
        match Config::try_from(&defaults) {
            Ok(defaults_config) => {
                self.config = Config::builder()
                    .add_source(defaults_config)
                    .build()
                    .unwrap_or_default();
            }
            Err(_) => {
                // If Config::try_from fails, manually set the defaults
                let builder = Config::builder()
                    .set_default("logging.level", defaults.logging.level).unwrap_or_else(|_| Config::builder())
                    .set_default("logging.format", defaults.logging.format).unwrap_or_else(|_| Config::builder())
                    .set_default("logging.show_file", defaults.logging.show_file).unwrap_or_else(|_| Config::builder())
                    .set_default("logging.show_line_numbers", defaults.logging.show_line_numbers).unwrap_or_else(|_| Config::builder())
                    .set_default("logging.show_thread_ids", defaults.logging.show_thread_ids).unwrap_or_else(|_| Config::builder())
                    .set_default("logging.show_target", defaults.logging.show_target).unwrap_or_else(|_| Config::builder());
                
                self.config = builder.build().unwrap_or_default();
            }
        }
        
        self
    }

    pub fn load_from_files(mut self) -> Result<Self> {
        let config_paths = ConfigPaths::get_config_file_paths();
        
        let mut builder = Config::builder().add_source(self.config.clone());
        
        for path in config_paths.iter().rev() {
            if path.exists() {
                builder = builder.add_source(File::from(path.as_path()).format(FileFormat::Toml).required(false));
            }
        }
        
        self.config = builder.build()
            .map_err(|e| crate::Error::ConfigLoadError(format!("Failed to load config files: {}", e)))?;
        
        Ok(self)
    }

    pub fn load_from_file<P: AsRef<Path> + Debug>(mut self, path: P) -> Result<Self> {
        let path = path.as_ref();
        
        if !path.exists() {
            return Err(crate::Error::ConfigFileNotFoundError(path.to_string_lossy().to_string()));
        }

        let builder = Config::builder()
            .add_source(self.config.clone())
            .add_source(File::from(path).format(FileFormat::Toml));
        
        self.config = builder.build()
            .map_err(|e| crate::Error::ConfigParseError(format!("Failed to parse config file {}: {}", path.display(), e)))?;
        
        Ok(self)
    }

    pub fn load_from_env(mut self) -> Self {
        let env_source = Environment::with_prefix(env!("CARGO_PKG_NAME"))
            .separator("_")
            .try_parsing(true);
        
        let builder = Config::builder()
            .add_source(self.config.clone())
            .add_source(env_source);
        
        if let Ok(config) = builder.build() {
            self.config = config;
        }
        
        self
    }


    pub fn build(self) -> Result<AppConfig> {
        self.config
            .try_deserialize()
            .map_err(|e| crate::Error::ConfigValidationError(format!("Config validation failed: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::config::ConfigBuilder;

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
}
