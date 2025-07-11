use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub format: String,
    pub show_file: bool,
    pub show_line_numbers: bool,
    pub show_thread_ids: bool,
    pub show_target: bool,
    pub file_enabled: bool,
    pub file_path: String,
    pub file_rotation: String,
    pub file_level: String,
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
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join(env!("CARGO_PKG_NAME"));
        
        Self {
            level: "off".to_string(),
            format: "compact".to_string(),
            show_file: true,
            show_line_numbers: true,
            show_thread_ids: true,
            show_target: true,
            file_enabled: true,
            file_path: data_dir.to_string_lossy().to_string(),
            file_rotation: "daily".to_string(),
            file_level: "debug".to_string(),
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::config::AppConfig;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.logging.level, "off");
    }
}
