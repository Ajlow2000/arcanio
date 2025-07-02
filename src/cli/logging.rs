use crate::Result;
use tracing_subscriber::filter::LevelFilter;

pub fn setup_logging(verbosity: u8) -> Result<()> {
    let level = match verbosity {
        0 => LevelFilter::WARN,
        1 => LevelFilter::INFO,
        2 => LevelFilter::DEBUG,
        _ => LevelFilter::TRACE,
    };

    let subscriber = tracing_subscriber::fmt()
        .compact()              // Use a more compact, abbreviated log format
        .with_file(true)        // Display source code file paths
        .with_line_number(true) // Display source code line numbers
        .with_thread_ids(true)  // Display the thread ID an event was recorded on
        .with_target(true)      // Don't display the event's target (module path)
        .with_max_level(level)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| crate::Error::LoggingSetupError)?;

    Ok(())
}
