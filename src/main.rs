mod error;
mod cli;
mod config;

pub use error::Result;
pub use error::Error;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().map_err(|_| {Error::PanicHandlerSetupError})?;

    let cli_handle = tokio::spawn(async move {
        cli::main().await
    });

    tokio::select! {
        result = cli_handle => {
            match result {
                Ok(cli_result) => {
                    if let Err(e) = cli_result {
                        let exit_code = e.exit_code();
                        let wrapped = color_eyre::eyre::eyre!(e);
                        eprintln!("\nError callstack: {:?}", wrapped);
                        std::process::exit(exit_code);
                    }
                }
                Err(e) => eprintln!("Task join error: {:?}", e),
            }
        },
        _ = signal::ctrl_c() => { 
            println!("\nExiting..." );
            std::process::exit(Error::ControlC.exit_code());
        },
    }
    
    Ok(())
}
