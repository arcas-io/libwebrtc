use cxx::UniquePtr;
use libwebrtc_sys::ffi::ArcasVideoTrack;

use crate::error::{Result, WebRTCError};

pub struct VideoTrack {
    pub(crate) cxx_track: Option<UniquePtr<ArcasVideoTrack>>,
}

impl VideoTrack {
    pub(crate) fn new(cxx_track: UniquePtr<ArcasVideoTrack>) -> Self {
        Self {
            cxx_track: Some(cxx_track),
        }
    }

    pub(crate) fn take_cxx(&mut self) -> Result<UniquePtr<ArcasVideoTrack>> {
        match self.cxx_track.take() {
            Some(cxx) => Ok(cxx),
            None => Err(WebRTCError::UnexpectedError(
                "Take cxx video track twice".into(),
            )),
        }
    }
}
