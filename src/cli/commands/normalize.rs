use crate::Result;
use arcanio_lib::{files::{glob_expand, SupportedFiletype}};

pub async fn handle_normalize(paths: Vec<String>) -> Result<()> {
    // let files = glob_expand(paths)?;
    // 
    // for file in files {
    //     match file.extension().and_then(|e| e.to_str()) {
    //         Some("flac") => {
    //             let mut cmd = SupportedFiletype::Flac.validation_command(&file)?;
    //             match cmd.output().await {
    //                 Ok(output) => {
    //                     if output.status.success() {
    //                         let codec = String::from_utf8_lossy(&output.stdout).trim().to_string();
    //                         if codec == "flac" {
    //                             println!("{} (valid flac)", file.display());
    //                         } else {
    //                             println!("{} (invalid flac - actually {})", file.display(), codec);
    //                         }
    //                     } else {
    //                         println!("{} (invalid flac)", file.display());
    //                     }
    //                 }
    //                 Err(e) => {
    //                     println!("{} (validation failed: {})", file.display(), e);
    //                 }
    //             }
    //         }
    //         Some("m4a") => {
    //             let mut cmd = SupportedFiletype::M4a.validation_command(&file)?;
    //             match cmd.output().await {
    //                 Ok(output) => {
    //                     if output.status.success() {
    //                         let codec = String::from_utf8_lossy(&output.stdout).trim().to_string();
    //                         if codec == "aac" {
    //                             println!("{} (valid m4a)", file.display());
    //                         } else {
    //                             println!("{} (invalid m4a - actually {})", file.display(), codec);
    //                         }
    //                     } else {
    //                         println!("{} (invalid m4a)", file.display());
    //                     }
    //                 }
    //                 Err(e) => {
    //                     println!("{} (validation failed: {})", file.display(), e);
    //                 }
    //             }
    //         }
    //         _ => {
    //             println!("{}", file.display());
    //         }
    //     }
    // }
    // 
    Ok(())
}
