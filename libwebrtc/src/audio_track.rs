use cxx::UniquePtr;
use libwebrtc_sys::audio_track::ffi::ArcasAudioTrack;

use crate::error::{Result, WebRTCError};

pub struct AudioTrack {
    pub(crate) cxx: Option<UniquePtr<ArcasAudioTrack>>,
}

impl AudioTrack {
    pub(crate) fn new(cxx: UniquePtr<ArcasAudioTrack>) -> Self {
        Self { cxx: Some(cxx) }
    }

    pub(crate) fn take_cxx(&mut self) -> Result<UniquePtr<ArcasAudioTrack>> {
        match self.cxx.take() {
            Some(x) => Ok(x),
            None => Err(WebRTCError::CXXUnwrapError(
                "could not unwrap AudioTrack".to_owned(),
            )),
        }
    }
}
