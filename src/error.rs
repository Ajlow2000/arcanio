use thiserror::Error;

pub type Result<T> = color_eyre::eyre::Result<T, Error>;

/// Error type used within the application
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Eyre(#[from] color_eyre::eyre::ErrReport),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error("control c")]
    ControlC,

    #[error("invalid config")]
    InvalidConfig,

    #[error("unable to setup panic handler")]
    PanicHandlerSetupError,

    #[error("unable to setup logging")]
    LoggingSetupError,

    #[error(transparent)]
    LibError(#[from] arcanio_lib::Error),

}

impl Error {
    pub fn exit_code(&self) -> i32 {
        match self {
            Error::PanicHandlerSetupError => 205,
            Error::ControlC => 130,
            Error::LoggingSetupError => 4,
            Error::InvalidConfig => 3,
            Error::LibError(_) => 5,
            Error::IoError(_) => 2,
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

