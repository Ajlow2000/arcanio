use std::fmt::{Display, Formatter};
#[cfg(test)]
use crate::config::validation::ValidationContext;

/// A single validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    /// The field that failed validation
    pub field_path: String,
    /// The value that failed validation
    pub current_value: String,
    /// Human-readable error message
    pub message: String,
    /// Additional context
    pub context: Vec<String>,
    /// Suggested fixes
    pub suggestions: Vec<String>,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(field_path: impl Into<String>, current_value: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field_path: field_path.into(),
            current_value: current_value.into(),
            message: message.into(),
            context: Vec::new(),
            suggestions: Vec::new(),
        }
    }

    /// Add context information to the error
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(context.into());
        self
    }

    /// Add a suggestion for fixing the error
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestions.push(suggestion.into());
        self
    }

    /// Create error from validation context
    #[cfg(test)]
    pub fn from_context(context: ValidationContext, message: impl Into<String>) -> Self {
        Self {
            field_path: context.field_path,
            current_value: context.current_value,
            message: message.into(),
            context: context.context,
            suggestions: Vec::new(),
        }
    }
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Validation error in field '{}': {}", self.field_path, self.message)?;
        
        if !self.current_value.is_empty() {
            write!(f, " (current value: '{}')", self.current_value)?;
        }
        
        for context in &self.context {
            write!(f, "\n  Context: {}", context)?;
        }
        
        if !self.suggestions.is_empty() {
            write!(f, "\n  Suggestions:")?;
            for suggestion in &self.suggestions {
                write!(f, "\n    - {}", suggestion)?;
            }
        }
        
        Ok(())
    }
}

/// Collection of validation errors
#[derive(Debug, Clone)]
pub struct ValidationErrors {
    pub errors: Vec<ValidationError>,
}

impl ValidationErrors {
    /// Create a new collection of validation errors
    pub fn new(errors: Vec<ValidationError>) -> Self {
        Self { errors }
    }

    /// Create a ValidationErrors with a single error
    pub fn single(error: ValidationError) -> Self {
        Self {
            errors: vec![error],
        }
    }

    /// Check if there are any errors
    #[cfg(test)]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the number of errors
    #[cfg(test)]
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Add another error to the collection
    #[cfg(test)]
    pub fn push(&mut self, error: ValidationError) {
        self.errors.push(error);
    }

    /// Merge another ValidationErrors into this one
    #[cfg(test)]
    pub fn merge(&mut self, other: ValidationErrors) {
        self.errors.extend(other.errors);
    }

    /// Get errors for a specific field path
    #[cfg(test)]
    pub fn errors_for_field(&self, field_path: &str) -> Vec<&ValidationError> {
        self.errors.iter().filter(|e| e.field_path == field_path).collect()
    }

    /// Get a summary of all error messages
    #[cfg(test)]
    pub fn summary(&self) -> String {
        if self.errors.is_empty() {
            return "No validation errors".to_string();
        }

        let mut summary = format!("Found {} validation error(s):", self.errors.len());
        for error in &self.errors {
            summary.push_str(&format!("\n  - {}: {}", error.field_path, error.message));
        }
        summary
    }
}

impl Display for ValidationErrors {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.errors.is_empty() {
            return write!(f, "No validation errors");
        }

        writeln!(f, "Configuration validation failed with {} error(s):", self.errors.len())?;
        for (i, error) in self.errors.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{}", error)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationError {}
impl std::error::Error for ValidationErrors {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_creation() {
        let error = ValidationError::new("logging.level", "invalid", "Invalid log level")
            .with_context("Must be one of: off, error, warn, info, debug, trace")
            .with_suggestion("Try using 'info' instead");

        assert_eq!(error.field_path, "logging.level");
        assert_eq!(error.current_value, "invalid");
        assert_eq!(error.message, "Invalid log level");
        assert_eq!(error.context.len(), 1);
        assert_eq!(error.suggestions.len(), 1);
    }

    #[test]
    fn test_validation_errors_collection() {
        let error1 = ValidationError::new("field1", "value1", "error1");
        let error2 = ValidationError::new("field2", "value2", "error2");
        
        let mut errors = ValidationErrors::new(vec![error1]);
        errors.push(error2);
        
        assert_eq!(errors.len(), 2);
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_validation_errors_display() {
        let error = ValidationError::new("logging.level", "invalid", "Invalid log level")
            .with_suggestion("Try using 'info' instead");
        
        let errors = ValidationErrors::single(error);
        let display = format!("{}", errors);
        
        assert!(display.contains("Configuration validation failed"));
        assert!(display.contains("logging.level"));
        assert!(display.contains("Invalid log level"));
        assert!(display.contains("Try using 'info' instead"));
    }

    #[test]
    fn test_errors_for_field() {
        let error1 = ValidationError::new("logging.level", "invalid", "error1");
        let error2 = ValidationError::new("logging.format", "invalid", "error2");
        let error3 = ValidationError::new("logging.level", "another", "error3");
        
        let errors = ValidationErrors::new(vec![error1, error2, error3]);
        let level_errors = errors.errors_for_field("logging.level");
        
        assert_eq!(level_errors.len(), 2);
        assert_eq!(level_errors[0].message, "error1");
        assert_eq!(level_errors[1].message, "error3");
    }
}