use clap::Parser as _;

use crate::{cli::{handle_test, setup_logging, Cli, Command}, Result};

pub async fn main() -> Result<()> {
    let cli = Cli::parse();

    setup_logging(cli.verbose)?;

    match cli.command {
        Command::Test => { handle_test().await? },
    }
    
    Ok(())
}
