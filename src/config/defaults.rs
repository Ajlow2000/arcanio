use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LoggingConfig {
    pub console: ConsoleLoggingConfig,
    pub file: FileLoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleLoggingConfig {
    pub level: String,
    pub format: String,
    pub show_file: bool,
    pub show_line_numbers: bool,
    pub show_thread_ids: bool,
    pub show_target: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileLoggingConfig {
    pub enabled: bool,
    pub level: String,
    pub path: String,
    pub rotation: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            logging: LoggingConfig::default(),
        }
    }
}

impl Default for LoggingConfig {
    fn default() -> Self {
        Self {
            console: ConsoleLoggingConfig::default(),
            file: FileLoggingConfig::default(),
        }
    }
}

impl Default for ConsoleLoggingConfig {
    fn default() -> Self {
        Self {
            level: "off".to_string(),
            format: "compact".to_string(),
            show_file: true,
            show_line_numbers: true,
            show_thread_ids: true,
            show_target: true,
        }
    }
}

impl Default for FileLoggingConfig {
    fn default() -> Self {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join(env!("CARGO_PKG_NAME"));
        
        Self {
            enabled: false,
            level: "debug".to_string(),
            path: data_dir.to_string_lossy().to_string(),
            rotation: "daily".to_string(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::config::AppConfig;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.logging.console.level, "off");
        assert_eq!(config.logging.file.enabled, false);
    }
}
