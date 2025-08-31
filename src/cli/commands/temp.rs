use crate::Result;

#[tracing::instrument]
pub async fn handle_temp() -> Result<()> {
    println!("Nothing to do");
    Ok(())
}
