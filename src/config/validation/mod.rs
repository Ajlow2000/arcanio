mod errors;
mod validators;
mod rules;

pub use errors::*;
pub use validators::*;

#[cfg(test)]
use std::fmt::Display;


/// Core validation trait that all config structs should implement
pub trait Validate {
    /// Validate the configuration, returning all validation errors found
    fn validate(&self) -> ValidationResult<()>;
}

/// Trait for validating individual fields
pub trait ValidateField<T> {
    /// Validate a single field value
    fn validate_field(value: &T, field_name: &str) -> ValidationResult<()>;
}

/// Result type for validation operations
pub type ValidationResult<T> = Result<T, ValidationErrors>;

/// Context information for validation
#[cfg(test)]
#[derive(Debug, Clone)]
pub struct ValidationContext {
    /// The field path (e.g., "logging.level")
    pub field_path: String,
    /// The current value being validated
    pub current_value: String,
    /// Additional context information
    pub context: Vec<String>,
}

#[cfg(test)]
impl ValidationContext {
    pub fn new(field_path: impl Into<String>, current_value: impl Display) -> Self {
        Self {
            field_path: field_path.into(),
            current_value: current_value.to_string(),
            context: Vec::new(),
        }
    }

    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context.push(context.into());
        self
    }
}


/// Macro to collect multiple validation results
macro_rules! collect_validation_errors {
    ($($validation:expr),* $(,)?) => {
        {
            let mut errors = Vec::new();
            $(
                if let Err(mut validation_errors) = $validation {
                    errors.append(&mut validation_errors.errors);
                }
            )*
            if errors.is_empty() {
                Ok(()) as ValidationResult<()>
            } else {
                Err(ValidationErrors::new(errors))
            }
        }
    };
}

pub(crate) use collect_validation_errors;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_context() {
        let context = ValidationContext::new("logging.level", "invalid")
            .with_context("Log level must be one of: off, error, warn, info, debug, trace");

        assert_eq!(context.field_path, "logging.level");
        assert_eq!(context.current_value, "invalid");
        assert_eq!(context.context.len(), 1);
    }

    #[test]
    fn test_collect_validation_errors_success() {
        let ok_result: ValidationResult<()> = Ok(());
        let ok_result2: ValidationResult<()> = Ok(());
        let ok_result3: ValidationResult<()> = Ok(());
        let result: ValidationResult<()> = collect_validation_errors!(
            ok_result,
            ok_result2,
            ok_result3
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_collect_validation_errors_failure() {
        let error1 = ValidationError::new("field1", "value1", "error1");
        let error2 = ValidationError::new("field2", "value2", "error2");
        
        let ok_result: ValidationResult<()> = Ok(());
        let err_result1: ValidationResult<()> = Err(ValidationErrors::single(error1));
        let err_result2: ValidationResult<()> = Err(ValidationErrors::single(error2));
        
        let result: ValidationResult<()> = collect_validation_errors!(
            err_result1,
            ok_result,
            err_result2
        );
        
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert_eq!(errors.errors.len(), 2);
    }
}