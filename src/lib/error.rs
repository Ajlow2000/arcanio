use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Error type used within the library
#[derive(Error, Debug)]
pub enum Error {
    #[error("file not found")]
    FileNotFound,

    #[error("invalid file path")]
    InvalidFilePath,
    
    #[error("glob pattern error: {0}")]
    GlobPattern(#[from] glob::PatternError),
    
    #[error("glob error: {0}")]
    Glob(#[from] glob::GlobError),
}
