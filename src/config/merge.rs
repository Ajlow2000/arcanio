//! Generic configuration merging system
//!
//! This module provides a flexible system for merging configuration values from different sources
//! (CLI arguments, config files, environment variables, etc.) with proper precedence handling.
//!
//! ## Key Components
//!
//! - `Merge` trait: Core trait for merging config structs
//! - `FromCli` trait: Converts CLI arguments to config structs
//! - `impl_merge!` macro: Generates merge implementations automatically
//!
//! ## Usage
//!
//! To add a new CLI field that maps to a config field:
//!
//! 1. Add the field to the CLI struct in `cli/structure.rs`
//! 2. Add the corresponding field to the config struct in `config/defaults.rs`
//! 3. Update the `FromCli` implementation to handle the new field
//! 4. Update the `impl_merge!` macro call to include the new field
//!
//! ## Example
//!
//! ```rust
//! // 1. Add to CLI
//! #[derive(Parser)]
//! struct Cli {
//!     #[arg(long)]
//!     pub output_format: Option<String>,
//! }
//!
//! // 2. Add to config
//! struct MyConfig {
//!     output_format: String,
//! }
//!
//! // 3. Update FromCli implementation
//! impl FromCli<Cli> for MyConfig {
//!     fn from_cli(cli: &Cli) -> Self {
//!         let mut config = MyConfig::default();
//!         if let Some(format) = &cli.output_format {
//!             config.output_format = format.clone();
//!         }
//!         config
//!     }
//! }
//!
//! // 4. Update macro call
//! impl_merge!(MyConfig, output_format);
//! ```

use crate::cli::Cli;
use crate::config::defaults::{AppConfig, LoggingConfig, ConsoleLoggingConfig, FileLoggingConfig};

/// A trait for merging configuration values from different sources
pub trait Merge<T> {
    /// Merge values from another source, using the provided defaults to determine
    /// which values should be considered "set" vs "default"
    fn merge_with(&mut self, other: T, defaults: &Self);
}

/// A trait for converting CLI arguments to configuration values
pub trait FromCli<T> {
    /// Convert CLI arguments to a configuration struct
    fn from_cli(cli: &T) -> Self;
}


/// A macro to generate merge implementations for structs
/// 
/// This macro automatically generates the boilerplate code for merging struct fields,
/// only overriding fields that differ from their default values.
/// 
/// # Usage
/// ```
/// impl_merge!(StructName, field1, field2, field3);
/// ```
/// 
/// # Example
/// ```
/// struct MyConfig {
///     name: String,
///     enabled: bool,
///     count: u32,
/// }
/// 
/// impl_merge!(MyConfig, name, enabled, count);
/// ```
/// 
/// This will generate an implementation that compares each field against the defaults
/// and only updates fields that have changed from their default values.
macro_rules! impl_merge {
    ($struct_name:ident, $($field:ident),*) => {
        impl Merge<$struct_name> for $struct_name {
            fn merge_with(&mut self, other: $struct_name, defaults: &Self) {
                $(
                    if other.$field != defaults.$field {
                        self.$field = other.$field;
                    }
                )*
            }
        }
    };
}

pub(crate) use impl_merge;

impl Merge<AppConfig> for AppConfig {
    fn merge_with(&mut self, other: AppConfig, defaults: &Self) {
        self.logging.merge_with(other.logging, &defaults.logging);
    }
}

impl Merge<LoggingConfig> for LoggingConfig {
    fn merge_with(&mut self, other: LoggingConfig, defaults: &Self) {
        self.console.merge_with(other.console, &defaults.console);
        self.file.merge_with(other.file, &defaults.file);
    }
}

impl_merge!(ConsoleLoggingConfig, 
    level, format, show_file, show_line_numbers, show_thread_ids, show_target
);

impl_merge!(FileLoggingConfig, 
    enabled, level, path, rotation
);

/// Convert CLI verbosity levels to log level strings
fn verbosity_to_log_level(verbose: u8) -> String {
    match verbose {
        0 => "off".to_string(),
        1 => "error".to_string(),
        2 => "info".to_string(),
        3 => "debug".to_string(),
        _ => "trace".to_string(),
    }
}

impl FromCli<Cli> for AppConfig {
    fn from_cli(cli: &Cli) -> Self {
        let mut config = AppConfig::default();
        
        // Only override console logging level if verbosity is specified
        if cli.verbose > 0 {
            config.logging.console.level = verbosity_to_log_level(cli.verbose);
        }
        
        config
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::{Cli, Command};

    #[test]
    fn test_merge_with_defaults() {
        let mut base = AppConfig::default();
        let mut override_config = AppConfig::default();
        override_config.logging.console.level = "debug".to_string();
        
        let defaults = AppConfig::default();
        base.merge_with(override_config, &defaults);
        
        assert_eq!(base.logging.console.level, "debug");
        assert_eq!(base.logging.console.format, "compact"); // unchanged
    }

    #[test]
    fn test_from_cli_no_verbosity() {
        let cli = Cli {
            command: Command::Temp,
            verbose: 0,
            config: None,
        };
        
        let config = AppConfig::from_cli(&cli);
        assert_eq!(config.logging.console.level, "off");
    }

    #[test]
    fn test_from_cli_with_verbosity() {
        let cli = Cli {
            command: Command::Temp,
            verbose: 2,
            config: None,
        };
        
        let config = AppConfig::from_cli(&cli);
        assert_eq!(config.logging.console.level, "info");
    }

    #[test]
    fn test_verbosity_levels() {
        assert_eq!(verbosity_to_log_level(0), "off");
        assert_eq!(verbosity_to_log_level(1), "error");
        assert_eq!(verbosity_to_log_level(2), "info");
        assert_eq!(verbosity_to_log_level(3), "debug");
        assert_eq!(verbosity_to_log_level(4), "trace");
        assert_eq!(verbosity_to_log_level(5), "trace");
    }
}