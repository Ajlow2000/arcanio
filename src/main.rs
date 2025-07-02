mod error;
mod cli;

pub use error::Result;
pub use error::Error;
use tokio::signal;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();

    let cli_handle = tokio::spawn(async move {
        cli::main().await
    });

    tokio::select! {
        result = cli_handle => {
            match result {
                Ok(cli_result) => {
                    if let Err(e) = cli_result {
                        eprintln!("CLI error: {:?}", e);
                        std::process::exit(e.exit_code());
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
}
