use std::str::FromStr;

use crate::{config::LoggingConfig, Error, Result};
use tracing::Level;
use tracing_subscriber::{filter::LevelFilter, prelude::*, Layer};
use tracing_appender::rolling::{RollingFileAppender, Rotation};

pub fn setup_logging(logging_config: &LoggingConfig) -> Result<()> {
    let console_level = match logging_config.console.level.as_str() {
        "off" => LevelFilter::OFF,
        level_str => LevelFilter::from_level(
            Level::from_str(level_str).map_err(|_| {
                Error::LoggingSetupError
            })?
        )
    };

    // Create console layer
    let console_layer = tracing_subscriber::fmt::layer()
        .compact()                                                  // Use a more compact, abbreviated log format
        .with_file(logging_config.console.show_file)                // Display source code file paths
        .with_line_number(logging_config.console.show_line_numbers) // Display source code line numbers
        .with_thread_ids(logging_config.console.show_thread_ids)    // Display the thread ID an event was recorded on
        .with_target(logging_config.console.show_target)            // Display the event's target (module path)
        .with_filter(console_level);

    let registry = tracing_subscriber::registry().with(console_layer);

    // Add file layer if enabled
    if logging_config.file.enabled {
        let file_level = match logging_config.file.level.as_str() {
            "off" => LevelFilter::OFF,
            level_str => LevelFilter::from_level(
                Level::from_str(level_str).map_err(|_| {
                    Error::LoggingSetupError
                })?
            )
        };

        let rotation = match logging_config.file.rotation.as_str() {
            "hourly" => Rotation::HOURLY,
            "daily" => Rotation::DAILY,
            "never" => Rotation::NEVER,
            _ => Rotation::DAILY,
        };

        let file_path = std::path::Path::new(&logging_config.file.path);
        
        // Ensure the directory exists
        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| Error::LoggingSetupError)?;
        }

        let file_appender = RollingFileAppender::new(rotation, file_path, env!("CARGO_PKG_NAME").to_string() + ".log");
        let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
        
        // Keep the guard alive by leaking it (this is intentional for logging)
        std::mem::forget(guard);
        
        let file_layer = tracing_subscriber::fmt::layer()
            .with_writer(non_blocking)
            .with_ansi(false)  // Disable ANSI colors for file output
            .with_file(logging_config.console.show_file)
            .with_line_number(logging_config.console.show_line_numbers)
            .with_thread_ids(logging_config.console.show_thread_ids)
            .with_target(logging_config.console.show_target)
            .with_filter(file_level);

        registry.with(file_layer).init();
    } else {
        registry.init();
    }

    Ok(())
}
