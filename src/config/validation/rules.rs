use crate::config::defaults::{AppConfig, LoggingConfig, ConsoleLoggingConfig, FileLoggingConfig};
use crate::config::validation::{
    Validate, ValidateField, ValidationResult, ValidationError, ValidationErrors,
    LogLevelValidator, LogFormatValidator, FileRotationValidator, FilePathValidator,
    collect_validation_errors,
};

impl Validate for AppConfig {
    fn validate(&self) -> ValidationResult<()> {
        // Validate nested config structs
        collect_validation_errors!(
            self.logging.validate()
        )
    }
}

impl Validate for LoggingConfig {
    fn validate(&self) -> ValidationResult<()> {
        // Validate nested config structs
        collect_validation_errors!(
            self.console.validate(),
            self.file.validate(),
            validate_logging_consistency(self)
        )
    }
}

impl Validate for ConsoleLoggingConfig {
    fn validate(&self) -> ValidationResult<()> {
        // Validate console logging fields
        collect_validation_errors!(
            LogLevelValidator::validate_field(&self.level, "logging.console.level"),
            LogFormatValidator::validate_field(&self.format, "logging.console.format")
        )
    }
}

impl Validate for FileLoggingConfig {
    fn validate(&self) -> ValidationResult<()> {
        // Only validate file logging fields if enabled
        if self.enabled {
            collect_validation_errors!(
                LogLevelValidator::validate_field(&self.level, "logging.file.level"),
                FilePathValidator::validate_field(&self.path, "logging.file.path"),
                FileRotationValidator::validate_field(&self.rotation, "logging.file.rotation")
            )
        } else {
            Ok(())
        }
    }
}

/// Validate cross-field consistency rules for logging config
fn validate_logging_consistency(config: &LoggingConfig) -> ValidationResult<()> {
    let mut errors = Vec::new();

    // Rule: If file logging is enabled, file_path should not be empty
    if config.file.enabled && config.file.path.is_empty() {
        errors.push(
            ValidationError::new("logging.file.path", &config.file.path, "File path cannot be empty when file logging is enabled")
                .with_suggestion("Provide a valid file path or disable file logging")
        );
    }

    // Note: The rule about console vs file logging consistency was removed
    // It's perfectly valid to have console logging disabled while file logging 
    // is enabled with a separate log level.

    if errors.is_empty() {
        Ok(())
    } else {
        Err(ValidationErrors::new(errors))
    }
}


/// Get the priority level for log levels (0 = off, higher = more verbose)
#[cfg(test)]
fn log_level_priority(level: &str) -> u8 {
    match level {
        "off" => 0,
        "error" => 1,
        "warn" => 2,
        "info" => 3,
        "debug" => 4,
        "trace" => 5,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_valid_app_config() {
        let config = AppConfig::default();
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_log_level() {
        let mut config = AppConfig::default();
        config.logging.console.level = "invalid".to_string();
        config.logging.file.enabled = false; // Disable file logging to avoid consistency errors
        
        let result = config.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors.errors[0].field_path.contains("logging.console.level"));
    }

    #[test]
    fn test_invalid_log_format() {
        let mut config = AppConfig::default();
        config.logging.console.format = "xml".to_string();
        config.logging.file.enabled = false; // Disable file logging to avoid consistency errors
        
        let result = config.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors.errors[0].field_path.contains("logging.console.format"));
    }

    #[test]
    fn test_multiple_validation_errors() {
        let mut config = AppConfig::default();
        config.logging.console.level = "invalid".to_string();
        config.logging.console.format = "xml".to_string();
        config.logging.file.rotation = "never-ever".to_string();
        config.logging.file.enabled = true; // Enable file logging to trigger rotation validation
        
        let result = config.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn test_file_logging_consistency() {
        let mut config = AppConfig::default();
        config.logging.file.enabled = true;
        config.logging.file.path = "".to_string(); // Empty path should cause error
        
        let result = config.validate();
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        let file_path_errors = errors.errors_for_field("logging.file.path");
        assert!(!file_path_errors.is_empty());
    }

    #[test]
    fn test_valid_file_logging_config() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.log");
        
        let mut config = AppConfig::default();
        config.logging.console.level = "info".to_string(); // Enable console logging
        config.logging.file.enabled = true;
        config.logging.file.path = file_path.to_string_lossy().to_string();
        config.logging.file.level = "info".to_string();
        
        let result = config.validate();
        assert!(result.is_ok());
    }

    #[test]
    fn test_log_level_priority() {
        assert_eq!(log_level_priority("off"), 0);
        assert_eq!(log_level_priority("error"), 1);
        assert_eq!(log_level_priority("warn"), 2);
        assert_eq!(log_level_priority("info"), 3);
        assert_eq!(log_level_priority("debug"), 4);
        assert_eq!(log_level_priority("trace"), 5);
        assert_eq!(log_level_priority("invalid"), 0);
    }

    #[test]
    fn test_console_vs_file_logging_is_allowed() {
        let mut config = AppConfig::default();
        config.logging.console.level = "off".to_string();
        config.logging.file.enabled = true;
        config.logging.file.level = "info".to_string();
        
        let result = validate_logging_consistency(&config.logging);
        // This configuration should now be valid - console off, file logging enabled
        assert!(result.is_ok());
    }

}