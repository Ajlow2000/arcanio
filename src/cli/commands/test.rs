use arcanio_lib::music::normalize_filename;

use crate::Result;

pub async fn handle_test() -> Result<()> {
    let _ = normalize_filename().await;
    Ok(())
}
