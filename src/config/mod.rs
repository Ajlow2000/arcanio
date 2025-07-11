mod builder;
mod defaults;
mod merge;
mod paths;

pub use builder::ConfigBuilder;
pub use defaults::*;
pub use merge::*;

use crate::Result;
use crate::cli::Cli;


pub fn load_config_with_cli_override(cli: &Cli) -> Result<AppConfig> {
    let mut config = ConfigBuilder::new()
        .load_defaults()
        .load_from_files()?
        .load_from_env()
        .build()?;

    // Handle custom config file path if specified
    if let Some(config_path) = &cli.config {
        config = ConfigBuilder::new()
            .load_defaults()
            .load_from_file(config_path)?
            .load_from_env()
            .build()?;
    }
    
    // Merge CLI arguments using the new merge system
    let cli_config = AppConfig::from_cli(cli);
    let defaults = AppConfig::default();
    config.merge_with(cli_config, &defaults);
    
    Ok(config)
}
