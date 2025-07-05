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

    // TODO some cli flags wont exist in config.  And some config values wont exist in cli.  So I gotta figure something out about that

    setup_logging(&config.logging)?;

    match cli.command {
        Command::Config { command } => { handle_config(command, &config).await? },
        Command::Temp => { handle_temp().await? },
    }
    
    Ok(())
}
