use cxx::UniquePtr;
use libwebrtc_sys::ffi::{
    ArcasCxxRtpTransceiverDirection, ArcasMediaType, ArcasRTPAudioTransceiver,
    ArcasRTPTransceiverDirection, ArcasRTPVideoTransceiver, ArcasTransceiverInit,
};

use crate::{error::WebRTCError, media_type::MediaType};

#[derive(Debug, Clone)]
pub enum TransceiverDirection {
    SendRecv,
    SendOnly,
    RecvOnly,
    Inactive,
}

impl From<TransceiverDirection> for ArcasCxxRtpTransceiverDirection {
    fn from(direction: TransceiverDirection) -> Self {
        match direction {
            TransceiverDirection::SendRecv => ArcasCxxRtpTransceiverDirection::kSendRecv,
            TransceiverDirection::SendOnly => ArcasCxxRtpTransceiverDirection::kSendOnly,
            TransceiverDirection::RecvOnly => ArcasCxxRtpTransceiverDirection::kRecvOnly,
            TransceiverDirection::Inactive => ArcasCxxRtpTransceiverDirection::kInactive,
        }
    }
}

impl From<ArcasRTPTransceiverDirection> for TransceiverDirection {
    fn from(direction: ArcasRTPTransceiverDirection) -> Self {
        match direction {
            ArcasRTPTransceiverDirection::kSendRecv => TransceiverDirection::SendRecv,
            ArcasRTPTransceiverDirection::kSendOnly => TransceiverDirection::SendOnly,
            ArcasRTPTransceiverDirection::kRecvOnly => TransceiverDirection::RecvOnly,
            ArcasRTPTransceiverDirection::kInactive => TransceiverDirection::Inactive,
            _ => TransceiverDirection::Inactive,
        }
    }
}

impl From<TransceiverDirection> for ArcasRTPTransceiverDirection {
    fn from(direction: TransceiverDirection) -> Self {
        match direction {
            TransceiverDirection::SendRecv => ArcasRTPTransceiverDirection::kSendRecv,
            TransceiverDirection::SendOnly => ArcasRTPTransceiverDirection::kSendOnly,
            TransceiverDirection::RecvOnly => ArcasRTPTransceiverDirection::kRecvOnly,
            TransceiverDirection::Inactive => ArcasRTPTransceiverDirection::kInactive,
        }
    }
}

impl ToString for TransceiverDirection {
    fn to_string(&self) -> String {
        match self {
            TransceiverDirection::SendRecv => "sendrecv".to_string(),
            TransceiverDirection::SendOnly => "sendonly".to_string(),
            TransceiverDirection::RecvOnly => "recvonly".to_string(),
            TransceiverDirection::Inactive => "inactive".to_string(),
        }
    }
}

pub struct TransceiverInit {
    cxx: ArcasTransceiverInit,
}

impl TransceiverInit {
    pub fn new(stream_ids: Vec<String>, direction: TransceiverDirection) -> Self {
        TransceiverInit {
            cxx: ArcasTransceiverInit {
                stream_ids,
                direction: direction.into(),
            },
        }
    }

    pub(crate) fn take_cxx(self) -> ArcasTransceiverInit {
        self.cxx
    }
}

impl Default for TransceiverInit {
    fn default() -> Self {
        TransceiverInit::new(vec!["0".into()], TransceiverDirection::SendRecv)
    }
}

pub struct VideoTransceiver {
    cxx_transceiver: UniquePtr<ArcasRTPVideoTransceiver>,
}

impl Clone for VideoTransceiver {
    fn clone(&self) -> Self {
        Self {
            cxx_transceiver: self.cxx_transceiver.clone(),
        }
    }
}

impl VideoTransceiver {
    pub(crate) fn new(cxx_transceiver: UniquePtr<ArcasRTPVideoTransceiver>) -> Self {
        Self { cxx_transceiver }
    }

    pub fn mid(&self) -> String {
        self.cxx_transceiver.mid()
    }

    pub fn direction(&self) -> TransceiverDirection {
        TransceiverDirection::from(self.cxx_transceiver.direction())
    }

    pub fn media_type(&self) -> MediaType {
        self.cxx_transceiver.media_type().into()
    }

    pub fn set_direction(&mut self, direction: TransceiverDirection) -> Result<(), WebRTCError> {
        if self
            .cxx_transceiver
            .set_direction(direction.into())
            .is_null()
        {
            Ok(())
        } else {
            Err(WebRTCError::FailedToSetDirection)
        }
    }
}

pub struct AudioTransceiver {
    cxx_transceiver: UniquePtr<ArcasRTPAudioTransceiver>,
}

impl Clone for AudioTransceiver {
    fn clone(&self) -> Self {
        Self {
            cxx_transceiver: self.cxx_transceiver.clone(),
        }
    }
}

impl AudioTransceiver {
    pub fn new(cxx_transceiver: UniquePtr<ArcasRTPAudioTransceiver>) -> Self {
        Self { cxx_transceiver }
    }

    pub fn mid(&self) -> String {
        self.cxx_transceiver.mid()
    }

    pub fn direction(&self) -> TransceiverDirection {
        TransceiverDirection::from(self.cxx_transceiver.direction())
    }

    pub fn media_type(&self) -> MediaType {
        self.cxx_transceiver.media_type().into()
    }

    pub fn set_direction(&mut self, direction: TransceiverDirection) -> Result<(), WebRTCError> {
        if self
            .cxx_transceiver
            .set_direction(direction.into())
            .is_null()
        {
            Ok(())
        } else {
            Err(WebRTCError::FailedToSetDirection)
        }
    }
}
