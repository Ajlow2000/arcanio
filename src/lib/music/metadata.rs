use tracing::{debug, error, info, trace, warn};
use crate::{Error, Result};

pub async fn normalize_filename() -> Result<()> {
    for _i in 0..10 {
        if _i == 3 {
            debug!("bar")
        }
        if _i == 5 {
            trace!("baz")
        }
        if _i == 2 {
            warn!("baz")
        }
        if _i == 2 {
            error!("baz")
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        info!("foo");
    }
    Ok(())
}
