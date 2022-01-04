use cxx::SharedPtr;
use libwebrtc_sys::audio_track_source::ffi::{create_audio_track_source, ArcasAudioTrackSource};

pub struct AudioTrackSource {
    cxx: SharedPtr<ArcasAudioTrackSource>,
    num_channels: usize,
    sample_rate_hz: i32,
}

impl AudioTrackSource {
    pub fn new(num_channels: usize, sample_rate_hz: i32) -> Self {
        Self {
            cxx: create_audio_track_source(),
            num_channels,
            sample_rate_hz,
        }
    }

    pub fn push_zeroed_data(&self) {
        self.cxx
            .push_zeroed_data(self.sample_rate_hz, self.num_channels);
    }
}
