use clap::Parser as _;

use crate::{cli::{config::handle_config, normalize::handle_normalize, setup_logging, temp::handle_temp, Cli, Command}, config, Result};

pub async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = config::load_config_with_cli_override(&cli)?;

    setup_logging(&config.logging)?;

    match cli.command {
        Command::Normalize { paths } => { handle_normalize(paths).await? },
        Command::Config { command } => { handle_config(command, &config).await? },
        Command::Temp => { handle_temp().await? },
    }
    
    Ok(())
}
