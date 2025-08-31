mod error;
pub use error::Error;
pub use error::Result;

pub mod music;
pub mod files;


#[derive(strum::EnumIter)]
pub enum SupportedFiletypes {
    Flac,
    M4a,
}

impl SupportedFiletypes {
    pub fn get_extension(self) -> String {
        match self {
            SupportedFiletypes::Flac => String::from(".flac"),
            SupportedFiletypes::M4a => String::from(".m4a"),
        }
    }
}
