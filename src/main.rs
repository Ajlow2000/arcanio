mod error;
mod cli;

pub use error::Result;
pub use error::Error;
use tokio::signal;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    color_eyre::install().unwrap();

    let (shutdown_sender, mut shutdown_receiver) = mpsc::unbounded_channel::<()>();

    let result = cli::main().await;
    
    if let Err(e) = result {
        eprintln!("{:?}", e);
        std::process::exit(e.exit_code());
    }
    
    let _ = shutdown_sender.send(());

    tokio::select! {
        _ = signal::ctrl_c() => { println!("\nExiting..." )},
        _ = shutdown_receiver.recv() => {},
    }
}
