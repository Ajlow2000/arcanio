use crate::Result;

pub async fn main() -> Result<()> {
    println!("In cli main");
    Ok(())
    // Err(crate::Error::InvalidConfig)
    //     .suggestion("skill issue")?
}
