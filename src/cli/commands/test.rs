use arcanio_lib::music::normalize_filename;

use crate::{Error, Result};

pub async fn handle_test() -> Result<()> {
    normalize_filename().await?;
    Ok(())
}
