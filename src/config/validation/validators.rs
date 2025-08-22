use std::path::Path;
use std::fs;
use crate::config::validation::{ValidationError, ValidationErrors, ValidationResult, ValidateField};

/// Validator for log levels
pub struct LogLevelValidator;

impl ValidateField<String> for LogLevelValidator {
    fn validate_field(value: &String, field_name: &str) -> ValidationResult<()> {
        const VALID_LEVELS: &[&str] = &["off", "error", "warn", "info", "debug", "trace"];
        
        if VALID_LEVELS.contains(&value.as_str()) {
            Ok(())
        } else {
            let error = ValidationError::new(field_name, value, "Invalid log level")
                .with_context(format!("Valid log levels are: {}", VALID_LEVELS.join(", ")))
                .with_suggestion(format!("Try using '{}' instead", suggest_closest_match(value, VALID_LEVELS)));
            
            Err(ValidationErrors::single(error))
        }
    }
}

/// Validator for log formats
pub struct LogFormatValidator;

impl ValidateField<String> for LogFormatValidator {
    fn validate_field(value: &String, field_name: &str) -> ValidationResult<()> {
        const VALID_FORMATS: &[&str] = &["compact", "pretty", "json"];
        
        if VALID_FORMATS.contains(&value.as_str()) {
            Ok(())
        } else {
            let error = ValidationError::new(field_name, value, "Invalid log format")
                .with_context(format!("Valid log formats are: {}", VALID_FORMATS.join(", ")))
                .with_suggestion(format!("Try using '{}' instead", suggest_closest_match(value, VALID_FORMATS)));
            
            Err(ValidationErrors::single(error))
        }
    }
}

/// Validator for file rotation settings
pub struct FileRotationValidator;

impl ValidateField<String> for FileRotationValidator {
    fn validate_field(value: &String, field_name: &str) -> ValidationResult<()> {
        const VALID_ROTATIONS: &[&str] = &["daily", "hourly", "size-based", "never"];
        
        if VALID_ROTATIONS.contains(&value.as_str()) {
            Ok(())
        } else {
            let error = ValidationError::new(field_name, value, "Invalid file rotation setting")
                .with_context(format!("Valid rotation settings are: {}", VALID_ROTATIONS.join(", ")))
                .with_suggestion(format!("Try using '{}' instead", suggest_closest_match(value, VALID_ROTATIONS)));
            
            Err(ValidationErrors::single(error))
        }
    }
}

/// Validator for file paths
pub struct FilePathValidator;

impl ValidateField<String> for FilePathValidator {
    fn validate_field(value: &String, field_name: &str) -> ValidationResult<()> {
        let path = Path::new(value);
        
        // Check if it's an absolute path or relative path
        if value.is_empty() {
            let error = ValidationError::new(field_name, value, "File path cannot be empty")
                .with_suggestion("Provide a valid file path");
            return Err(ValidationErrors::single(error));
        }
        
        // Check if parent directory exists (for file paths)
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                let error = ValidationError::new(field_name, value, "Parent directory does not exist")
                    .with_context(format!("Parent directory: {}", parent.display()))
                    .with_suggestion("Create the parent directory first or use a different path");
                return Err(ValidationErrors::single(error));
            }
            
            // Check if parent directory is writable
            if let Err(e) = fs::metadata(parent).and_then(|m| {
                if m.permissions().readonly() {
                    Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Directory is readonly"))
                } else {
                    Ok(())
                }
            }) {
                let error = ValidationError::new(field_name, value, "Cannot write to parent directory")
                    .with_context(format!("Error: {}", e))
                    .with_suggestion("Choose a writable directory or fix permissions");
                return Err(ValidationErrors::single(error));
            }
        }
        
        Ok(())
    }
}

/// Validator for directory paths
pub struct DirectoryPathValidator;

impl ValidateField<String> for DirectoryPathValidator {
    fn validate_field(value: &String, field_name: &str) -> ValidationResult<()> {
        if value.is_empty() {
            let error = ValidationError::new(field_name, value, "Directory path cannot be empty")
                .with_suggestion("Provide a valid directory path");
            return Err(ValidationErrors::single(error));
        }
        
        let path = Path::new(value);
        
        // Check if path exists
        if !path.exists() {
            let error = ValidationError::new(field_name, value, "Directory does not exist")
                .with_suggestion("Create the directory first or use a different path");
            return Err(ValidationErrors::single(error));
        }
        
        // Check if it's actually a directory
        if !path.is_dir() {
            let error = ValidationError::new(field_name, value, "Path is not a directory")
                .with_suggestion("Provide a path to a directory, not a file");
            return Err(ValidationErrors::single(error));
        }
        
        // Check if directory is writable
        if let Err(e) = fs::metadata(path).and_then(|m| {
            if m.permissions().readonly() {
                Err(std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Directory is readonly"))
            } else {
                Ok(())
            }
        }) {
            let error = ValidationError::new(field_name, value, "Directory is not writable")
                .with_context(format!("Error: {}", e))
                .with_suggestion("Fix directory permissions or choose a different directory");
            return Err(ValidationErrors::single(error));
        }
        
        Ok(())
    }
}

/// Helper function to suggest the closest match from a list of valid options
fn suggest_closest_match<'a>(input: &str, valid_options: &'a [&str]) -> &'a str {
    valid_options
        .iter()
        .min_by_key(|option| levenshtein_distance(input, option))
        .unwrap_or(&valid_options[0])
}

/// Simple Levenshtein distance implementation for string similarity
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let s1_len = s1_chars.len();
    let s2_len = s2_chars.len();
    
    if s1_len == 0 { return s2_len; }
    if s2_len == 0 { return s1_len; }
    
    let mut matrix = vec![vec![0; s2_len + 1]; s1_len + 1];
    
    for i in 0..=s1_len {
        matrix[i][0] = i;
    }
    for j in 0..=s2_len {
        matrix[0][j] = j;
    }
    
    for i in 1..=s1_len {
        for j in 1..=s2_len {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] { 0 } else { 1 };
            matrix[i][j] = (matrix[i - 1][j] + 1)
                .min(matrix[i][j - 1] + 1)
                .min(matrix[i - 1][j - 1] + cost);
        }
    }
    
    matrix[s1_len][s2_len]
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_log_level_validator_valid() {
        let result = LogLevelValidator::validate_field(&"info".to_string(), "logging.level");
        assert!(result.is_ok());
    }

    #[test]
    fn test_log_level_validator_invalid() {
        let result = LogLevelValidator::validate_field(&"invalid".to_string(), "logging.level");
        assert!(result.is_err());
        
        let errors = result.unwrap_err();
        assert_eq!(errors.len(), 1);
        assert!(errors.errors[0].message.contains("Invalid log level"));
    }

    #[test]
    fn test_log_format_validator_valid() {
        let result = LogFormatValidator::validate_field(&"json".to_string(), "logging.format");
        assert!(result.is_ok());
    }

    #[test]
    fn test_log_format_validator_invalid() {
        let result = LogFormatValidator::validate_field(&"xml".to_string(), "logging.format");
        assert!(result.is_err());
    }

    #[test]
    fn test_file_rotation_validator_valid() {
        let result = FileRotationValidator::validate_field(&"daily".to_string(), "logging.file_rotation");
        assert!(result.is_ok());
    }

    #[test]
    fn test_file_path_validator_empty() {
        let result = FilePathValidator::validate_field(&"".to_string(), "logging.file_path");
        assert!(result.is_err());
    }

    #[test]
    fn test_file_path_validator_valid() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.log");
        
        let result = FilePathValidator::validate_field(&file_path.to_string_lossy().to_string(), "logging.file_path");
        assert!(result.is_ok());
    }

    #[test]
    fn test_directory_path_validator_valid() {
        let dir = tempdir().unwrap();
        
        let result = DirectoryPathValidator::validate_field(&dir.path().to_string_lossy().to_string(), "logging.file_path");
        assert!(result.is_ok());
    }

    #[test]
    fn test_directory_path_validator_nonexistent() {
        let result = DirectoryPathValidator::validate_field(&"/nonexistent/path".to_string(), "logging.file_path");
        assert!(result.is_err());
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("info", "info"), 0);
        assert_eq!(levenshtein_distance("info", "warn"), 4);
        assert_eq!(levenshtein_distance("debug", "trace"), 5);
    }

    #[test]
    fn test_suggest_closest_match() {
        let options = &["info", "debug", "warn", "error"];
        assert_eq!(suggest_closest_match("inf", options), "info");
        assert_eq!(suggest_closest_match("debg", options), "debug");
        assert_eq!(suggest_closest_match("warning", options), "warn");
    }
}