use crate::error::Result;
use cxx::SharedPtr;
use libwebrtc_sys::audio_track_source::ffi::{create_audio_track_source, ArcasAudioTrackSource};

use crate::error::WebRTCError;

#[derive(Clone)]
pub struct AudioTrackSource {
    cxx: SharedPtr<ArcasAudioTrackSource>,
    num_channels: usize,
    sample_rate_hz: i32,
}

unsafe impl Send for AudioTrackSource {}

impl AudioTrackSource {
    pub fn new(num_channels: usize, sample_rate_hz: i32) -> Self {
        Self {
            cxx: create_audio_track_source(),
            num_channels,
            sample_rate_hz,
        }
    }

    pub fn cxx_ref(&self) -> Result<&ArcasAudioTrackSource> {
        self.cxx
            .as_ref()
            .ok_or_else(|| WebRTCError::UnexpectedError("source ref missing".into()))
    }

    pub fn push_10ms_zeroed_data(&self) {
        self.cxx
            .push_zeroed_data(self.sample_rate_hz, self.num_channels);
    }
}
