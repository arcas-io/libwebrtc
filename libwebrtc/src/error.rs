use crossbeam_channel::{RecvError, SendError};
use cxx::UniquePtr;
use libwebrtc_sys::ffi::ArcasRTCError;
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

    #[error("Async send error from tokio: {0}")]
    AsyncSendError(String),

    #[error("Value has already been taken: {0}")]
    TakeError(String),

    #[error("Failed to parse SDP: message: {0} @ line: {1}")]
    SdpParseError(String, String),

    #[error("Failed to seet sdp")]
    FailedToGenerateSDP(String),

    #[error("Encountered error during recv: {0}")]
    RecvError(String),

    #[error("Received RTC error: {0}")]
    RTCError(String),

    #[error("Failed to set SDP: {0}")]
    FailedToSetSDP(String),

    #[error("Failed to set transceiver direction")]
    FailedToSetDirection,
}

impl<T> From<SendError<T>> for WebRTCError {
    fn from(err: SendError<T>) -> Self {
        WebRTCError::SendError(format!("{}", err))
    }
}

impl<T> From<tokio::sync::mpsc::error::SendError<T>> for WebRTCError {
    fn from(err: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Self::AsyncSendError(format!("{}", err))
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

impl From<UniquePtr<ArcasRTCError>> for WebRTCError {
    fn from(value: UniquePtr<ArcasRTCError>) -> Self {
        WebRTCError::RTCError(format!(
            "kind={:?} ok={:?} message={}",
            value.kind(),
            value.ok(),
            value.message()
        ))
    }
}

pub(crate) fn aracs_rtc_error_to_err(err: UniquePtr<ArcasRTCError>) -> WebRTCError {
    WebRTCError::RTCError(format!(
        "kind={:?} ok={:?} message={}",
        err.kind(),
        err.ok(),
        err.message()
    ))
}
