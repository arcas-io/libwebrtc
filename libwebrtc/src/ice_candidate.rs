use cxx::UniquePtr;
use libwebrtc_sys::ffi::ArcasICECandidate;
use libwebrtc_sys::p2p::ice_transport_internal::ffi::{
    create_arcas_p2p_ice_config, ArcasP2PIceConfig,
};

pub struct ICECandidate {
    cxx_ptr: UniquePtr<ArcasICECandidate>,
}
pub struct P2pIceConfig {
    pub(crate) cxx_ptr: UniquePtr<ArcasP2PIceConfig>,
}

impl ICECandidate {
    pub fn new(cxx_ptr: UniquePtr<ArcasICECandidate>) -> Self {
        Self { cxx_ptr }
    }

    pub fn take_cxx(self) -> UniquePtr<ArcasICECandidate> {
        self.cxx_ptr
    }

    pub fn sdp(&self) -> String {
        self.cxx_ptr.to_string()
    }

    pub fn sdp_mid(&self) -> String {
        self.cxx_ptr.sdp_mid()
    }

    pub fn sdp_mline_index(&self) -> u32 {
        self.cxx_ptr.sdp_mline_index()
    }
}

impl ToString for ICECandidate {
    fn to_string(&self) -> String {
        self.cxx_ptr.to_string()
    }
}

impl P2pIceConfig {
    pub fn set_presume_writable_when_fully_relayed(&mut self, val: bool) {
        self.cxx_ptr
            .pin_mut()
            .set_presume_writable_when_fully_relayed(val);
    }
}

impl Default for P2pIceConfig {
    fn default() -> Self {
        Self {
            cxx_ptr: create_arcas_p2p_ice_config(),
        }
    }
}
