use cxx::UniquePtr;
use libwebrtc_sys::ffi::ArcasICECandidate;

pub struct ICECandidate {
    cxx_ptr: UniquePtr<ArcasICECandidate>,
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
