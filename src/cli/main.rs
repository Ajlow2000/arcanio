use arcanio_lib::music::normalize_filename;

use crate::{cli::setup_logging, Result};

pub async fn main() -> Result<()> {
    setup_logging()?;

    println!("In cli main");
    normalize_filename().await;
    Ok(())
}
