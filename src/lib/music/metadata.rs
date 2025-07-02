use tracing::info;

pub async fn normalize_filename() {
    for _i in 0..10 {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        info!("foo")
    }
}
