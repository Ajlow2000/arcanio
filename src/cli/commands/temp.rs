use arcanio_lib::music::normalize_filename;

use crate::Result;

#[tracing::instrument]
pub async fn handle_temp() -> Result<()> {
    normalize_filename().await?;
    Ok(())
}
