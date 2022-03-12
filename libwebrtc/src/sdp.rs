use crate::error::{Result, WebRTCError};
use cxx::UniquePtr;
use libwebrtc_sys::ffi::{create_arcas_session_description, ArcasSDPType, ArcasSessionDescription};
use libwebrtc_sys::pc::jsep_api::ffi::SdpType;

#[derive(Debug, Clone, Copy)]
pub enum SDPType {
    Offer,
    PrAnswer,
    Answer,
    Rollback,
}

impl From<ArcasSDPType> for SDPType {
    fn from(value: ArcasSDPType) -> Self {
        match value {
            ArcasSDPType::kOffer => SDPType::Offer,
            ArcasSDPType::kPrAnswer => SDPType::PrAnswer,
            ArcasSDPType::kAnswer => SDPType::Answer,
            ArcasSDPType::kRollback => SDPType::Rollback,
            #[allow(unreachable_patterns)]
            _ => panic!("Unknown SDP type"),
        }
    }
}

impl From<SDPType> for ArcasSDPType {
    fn from(value: SDPType) -> ArcasSDPType {
        match value {
            SDPType::Offer => ArcasSDPType::kOffer,
            SDPType::PrAnswer => ArcasSDPType::kPrAnswer,
            SDPType::Answer => ArcasSDPType::kAnswer,
            SDPType::Rollback => ArcasSDPType::kRollback,
            #[allow(unreachable_patterns)]
            _ => panic!("Unknown SDP type"),
        }
    }
}

impl From<SDPType> for SdpType {
    fn from(value: SDPType) -> SdpType {
        match value {
            SDPType::Offer => SdpType::kOffer,
            SDPType::PrAnswer => SdpType::kPrAnswer,
            SDPType::Answer => SdpType::kAnswer,
            SDPType::Rollback => SdpType::kRollback,
            #[allow(unreachable_patterns)]
            _ => panic!("Unknown SDP type"),
        }
    }
}

pub struct SessionDescription {
    pub(crate) cxx_sdp: UniquePtr<ArcasSessionDescription>,
}

impl std::fmt::Debug for SessionDescription {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SessionDescription")
            .field("type", &self.cxx_sdp.get_type())
            .field("sdp", &self.cxx_sdp.cxx_to_string())
            .finish()
    }
}

impl SessionDescription {
    pub(crate) fn new_from_cxx(cxx_sdp: UniquePtr<ArcasSessionDescription>) -> Self {
        SessionDescription { cxx_sdp }
    }

    pub fn new(kind: SDPType, sdp: String) -> Result<Self> {
        let cxx_sdp_result = create_arcas_session_description(kind.into(), sdp);

        // Note: If this is unhandled we may get a nullptr back
        if !cxx_sdp_result.ok {
            return Err(WebRTCError::SdpParseError(
                cxx_sdp_result.error.description,
                cxx_sdp_result.error.line,
            ));
        }

        Ok(Self {
            cxx_sdp: cxx_sdp_result.session,
        })
    }

    pub fn get_type(&self) -> SDPType {
        self.cxx_sdp.get_type().into()
    }

    pub(crate) fn clone_cxx(self) -> UniquePtr<ArcasSessionDescription> {
        self.cxx_sdp.clone_cxx()
    }

    /// Copies the current SDP and forces it to be a remote copy
    /// vs a pointer to a local copy of an sdp.
    pub fn copy_to_remote(&self) -> Result<Self> {
        Self::new(self.cxx_sdp.get_type().into(), self.cxx_sdp.cxx_to_string())
    }

    pub fn take_cxx(self) -> UniquePtr<ArcasSessionDescription> {
        self.cxx_sdp
    }
}

impl ToString for SessionDescription {
    fn to_string(&self) -> String {
        self.cxx_sdp.cxx_to_string()
    }
}
impl Clone for SessionDescription {
    fn clone(&self) -> Self {
        Self {
            cxx_sdp: self.cxx_sdp.clone_cxx(),
        }
    }
}
