use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Error type used within the library
#[derive(Error, Debug)]
pub enum Error {
    #[error("file not found")]
    FileNotFound,
}
