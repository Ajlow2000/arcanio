use crate::Result;

pub fn setup_logging() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .compact()              // Use a more compact, abbreviated log format
        .with_file(true)        // Display source code file paths
        .with_line_number(true) // Display source code line numbers
        .with_thread_ids(true)  // Display the thread ID an event was recorded on
        .with_target(true)      // Don't display the event's target (module path)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .map_err(|_| crate::Error::LoggingSetupError)?;

    Ok(())
}
