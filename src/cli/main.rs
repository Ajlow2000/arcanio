use clap::Parser as _;

use crate::{cli::{handle_config, handle_temp, setup_logging, Cli, Command}, config, Result};

pub async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Load configuration
    let config = if let Some(config_file) = &cli.config {
        config::load_config_with_override(Some(config_file), cli.verbose)?
    } else {
        config::load_config(cli.verbose)?
    };

    setup_logging(&config.logging)?;

    match cli.command {
        Command::Config { command } => { handle_config(command, &config).await? },
        Command::Temp => { handle_temp().await? },
    }
    
    Ok(())
}
