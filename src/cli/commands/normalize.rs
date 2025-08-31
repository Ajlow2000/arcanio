use crate::Result;
use arcanio_lib::files::glob_expand;

pub async fn handle_normalize(paths: Vec<String>) -> Result<()> {
    let files = glob_expand(paths)?;
    
    for file in files {
        println!("{}", file.display());
    }
    
    Ok(())
}
