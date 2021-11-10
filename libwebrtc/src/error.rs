use crossbeam_channel::{RecvError, SendError};
use media_pipeline::error::MediaPipelineError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, WebRTCError>;

#[derive(Error, Debug)]
pub enum WebRTCError {
    #[error("Unexpected: {0}")]
    UnexpectedError(String),

    #[error("Unexpected media error: {0}")]
    UnexpectedMediaError(String),

    #[error("Failed to unwrap from underlying cxx library: {0}")]
    CXXUnwrapError(String),

    #[error("Send error over thread boundary: {0}")]
    SendError(String),

    #[error("Unknown video codec error")]
    VideoCodecError,

    #[error("Memory error in video encode")]
    VideoCodecMemory,

    #[error("Wrong video encodeing parameters")]
    VideoCodecErrParameter,
    #[error("Unitialized encoder")]
    VideoCodecUninitialized,

    #[error("Video encoder is falling back to software")]
    VideoCodecFallbackSoftware,

    #[error("Target bitrate overshoot")]
    VideoCodecTargetBitrateOvershoot,

    #[error("Simulacast parameters not supported")]
    VideoCodecErrSimulcastParamsNotSupported,

    #[error("Generic video encoder error")]
    VideoCodecEncoderFailure,

    #[error("Unsupported video codec: {0}")]
    VideoCodecUnsupportedType(String),

    #[error("Video encoder failed to initialize")]
    VideoEncoderFailedInit,

    #[error("Took video encoder rx multiple times")]
    FailedToTakeVideoEncoderRx,

    #[error("Failed to get system time")]
    FailedToGetSystemTime,

    #[error("Cancelled operation: {0}")]
    Cancel(String),

    #[error("Unknown receive error: {0}")]
    ReceiveError(String),
}

impl<T> From<SendError<T>> for WebRTCError {
    fn from(err: SendError<T>) -> Self {
        WebRTCError::SendError(format!("{}", err))
    }
}

impl From<RecvError> for WebRTCError {
    fn from(value: RecvError) -> Self {
        WebRTCError::ReceiveError(value.to_string())
    }
}

impl From<MediaPipelineError> for WebRTCError {
    fn from(value: MediaPipelineError) -> Self {
        WebRTCError::UnexpectedMediaError(value.to_string())
    }
}
