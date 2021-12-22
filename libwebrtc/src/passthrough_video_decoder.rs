use libwebrtc_sys::video_decoding::{DecodedImageCallback, VideoDecoderImpl};

use crate::{
    now::now,
    video_frame::{AsCxxVideoFrame, EmptyVideoFrame},
};

#[derive(Default)]
pub struct PassthroughVideoDecoder {
    num_frames_received: i32,
}

impl VideoDecoderImpl for PassthroughVideoDecoder {
    fn decode(
        &mut self,
        _image: &libwebrtc_sys::ffi::ArcasCxxEncodedImage,
        _missing_frames: bool,
        _render_times_ms: i64,
        callback: DecodedImageCallback<'_>,
    ) -> i32 {
        self.num_frames_received += 1;
        let ts = match now() {
            Ok(x) => x,
            Err(_) => return 0,
        };
        // call the decoded image callback here
        let mut frame = match EmptyVideoFrame::create(ts) {
            Ok(f) => f,
            Err(_) => return 0,
        };
        let _ = frame
            .as_cxx_video_frame_ref_mut()
            .map(|f| callback.decoded(f));
        0
    }

    fn release(&mut self) -> i32 {
        0
    }

    fn get_num_frames_received(&self) -> i32 {
        self.num_frames_received
    }
}
