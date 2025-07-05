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

impl AppConfig {
    pub fn merge_with(&mut self, other: AppConfig) {
        self.logging.merge_with(other.logging);
    }
}

impl LoggingConfig {
    pub fn merge_with(&mut self, other: LoggingConfig) {
        if other.level != LoggingConfig::default().level {
            self.level = other.level;
        }
        if other.format != LoggingConfig::default().format {
            self.format = other.format;
        }
        if other.show_file != LoggingConfig::default().show_file {
            self.show_file = other.show_file;
        }
        if other.show_line_numbers != LoggingConfig::default().show_line_numbers {
            self.show_line_numbers = other.show_line_numbers;
        }
        if other.show_thread_ids != LoggingConfig::default().show_thread_ids {
            self.show_thread_ids = other.show_thread_ids;
        }
        if other.show_target != LoggingConfig::default().show_target {
            self.show_target = other.show_target;
        }
        if other.file_enabled != LoggingConfig::default().file_enabled {
            self.file_enabled = other.file_enabled;
        }
        if other.file_path != LoggingConfig::default().file_path {
            self.file_path = other.file_path;
        }
        if other.file_rotation != LoggingConfig::default().file_rotation {
            self.file_rotation = other.file_rotation;
        }
        if other.file_level != LoggingConfig::default().file_level {
            self.file_level = other.file_level;
        }
    }
}

