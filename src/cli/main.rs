use arcanio_lib::music::normalize_filename;
use clap::Parser as _;

use crate::{cli::{setup_logging, Cli}, Result};

pub async fn main() -> Result<()> {
    println!("In cli main");

    let cli = Cli::parse();

    setup_logging(cli.verbose)?;

    match cli.command {
        crate::cli::Command::Test => {normalize_filename().await},
    }
    
    Ok(())
}
