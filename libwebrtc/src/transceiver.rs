use cxx::UniquePtr;
use libwebrtc_sys::ffi::{
    ArcasCxxRtpTransceiverDirection, ArcasRTPVideoTransceiver, ArcasTransceiverInit,
};

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

impl VideoTransceiver {
    pub(crate) fn new(cxx_transceiver: UniquePtr<ArcasRTPVideoTransceiver>) -> Self {
        Self { cxx_transceiver }
    }

    pub fn mid(&self) -> String {
        self.cxx_transceiver.mid()
    }
}
