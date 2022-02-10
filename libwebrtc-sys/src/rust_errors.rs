use thiserror::Error;

pub type Result<T> = std::result::Result<T, LibWebRTCError>;

#[derive(Error, Debug)]
pub enum LibWebRTCError {
    #[error("Conversion error: {0}")]
    ConversionError(String),
}
