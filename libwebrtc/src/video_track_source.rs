use std::sync::Arc;

use cxx::UniquePtr;
use libwebrtc_sys::ffi::{create_arcas_video_track_source, ArcasVideoTrackSource};
use parking_lot::Mutex;

use crate::{
    error::{Result, WebRTCError},
    video_frame::{AsCxxVideoFrame, EncodedVideoFrame, RawVideoFrame},
};

/// This represents the writable handle of the video track source. It's *extremely* unsafe to
/// write to multiple track source references directly so we attempt to make it easy to clone the
/// read handle and easy to use the write.
pub struct VideoTrackSourceWriter {
    pub(crate) cxx_track_source: Arc<Mutex<UniquePtr<ArcasVideoTrackSource>>>,
}

impl VideoTrackSourceWriter {
    pub fn push_raw_frame(&self, frame: RawVideoFrame) -> Result<()> {
        self.cxx_track_source
            .lock()
            .push_frame(frame.as_cxx_video_frame_ref()?);
        Ok(())
    }

    pub fn push_encoded_frame(&self, frame: EncodedVideoFrame) -> Result<()> {
        self.cxx_track_source
            .lock()
            .push_frame(frame.as_cxx_video_frame_ref()?);
        Ok(())
    }
}

/// Readable handle for the underlying video track source.
pub struct VideoTrackSource {
    pub(crate) cxx_track_source: Arc<UniquePtr<ArcasVideoTrackSource>>,
}

impl VideoTrackSource {
    pub fn create() -> (Self, VideoTrackSourceWriter) {
        let source = create_arcas_video_track_source();
        let source_writer = source.cxx_clone();

        (
            Self {
                cxx_track_source: Arc::new(source),
            },
            VideoTrackSourceWriter {
                cxx_track_source: Arc::new(Mutex::new(source_writer)),
            },
        )
    }

    pub(crate) fn cxx_ref(&self) -> Result<&ArcasVideoTrackSource> {
        self.cxx_track_source
            .as_ref()
            .as_ref()
            .ok_or_else(|| WebRTCError::UnexpectedError("missing track source".into()))
    }
}

impl Clone for VideoTrackSource {
    /// Creates a clone of the reference (increasing the refcount) of the
    /// underlying VideoTrackSource this will still point to the same underlying
    /// source and is intended to be used so that sources can be passed across a
    /// thread boundary for media production.
    fn clone(&self) -> Self {
        Self {
            cxx_track_source: Arc::new(self.cxx_track_source.cxx_clone()),
        }
    }
}
