mod builder;
mod defaults;
mod paths;
mod tests;

pub use builder::ConfigBuilder;
pub use defaults::*;

use crate::Result;

pub fn load_config(verbose: u8) -> Result<AppConfig> {
    ConfigBuilder::new()
        .load_defaults()
        .load_from_files()?
        .load_from_env()
        .load_from_cli_args(verbose)
        .build()
}

pub fn load_config_with_override(config_file: Option<&str>, verbose: u8) -> Result<AppConfig> {
    let mut builder = ConfigBuilder::new()
        .load_defaults()
        .load_from_files()?
        .load_from_env();

    if let Some(config_path) = config_file {
        builder = builder.load_from_file(config_path)?;
    }

    builder.load_from_cli_args(verbose).build()
}
