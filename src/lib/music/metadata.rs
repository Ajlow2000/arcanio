use tracing::{debug, info, trace};

pub async fn normalize_filename() {
    for _i in 0..10 {
        if _i == 3 {
            debug!("bar")
        }
        if _i == 5 {
            trace!("baz")
        }
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        info!("foo")
    }
}
