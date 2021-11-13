use cxx::UniquePtr;
use libwebrtc_sys::ffi::ArcasRTCError;

pub struct RTCError {
    cxx_rtc_error: UniquePtr<ArcasRTCError>,
}

impl RTCError {
    pub(crate) fn new(cxx_rtc_error: UniquePtr<ArcasRTCError>) -> Self {
        Self { cxx_rtc_error }
    }

    fn message(&self) -> String {
        self.cxx_rtc_error.message()
    }
}

impl ToString for RTCError {
    fn to_string(&self) -> String {
        self.message()
    }
}

impl std::fmt::Debug for RTCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RTCError")
            .field("kind", &self.cxx_rtc_error.kind())
            .field("message", &self.cxx_rtc_error.message())
            .finish()
    }
}
