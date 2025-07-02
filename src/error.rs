use thiserror::Error;

pub type Result<T> = color_eyre::eyre::Result<T, Error>;

/// Error type used within the application
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Eyre(#[from] color_eyre::eyre::ErrReport),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("invalid config")]
    InvalidConfig,
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::InvalidConfig => 2,
            Error::IoError(_) => 3,
            Error::Eyre(report) => {
                // Check if the root cause is a custom error
                if let Some(custom_err) = report.root_cause().downcast_ref::<Error>() {
                    custom_err.exit_code()
                } else {
                    1
                }
            }
        }
    }
}

