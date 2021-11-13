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
        return self.cxx_ptr;
    }
}
