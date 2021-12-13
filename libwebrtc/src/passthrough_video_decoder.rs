use libwebrtc_sys::video_decoder::{VideoDecoderImpl, DecodedImageCallback};

use crate::{video_frame::{EmptyVideoFrame, AsCxxVideoFrame}, now::now};

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
        // call the decoded image callback here
        let mut frame = match EmptyVideoFrame::create(now().unwrap()) {
            Ok(f) => f,
            Err(_) => return 0,
        };
        let _ = frame.as_cxx_video_frame_ref_mut()
            .and_then(|f| Ok(callback.decoded(f)));
        0
    }

    fn release(&mut self) -> i32 {
        0
    }

    fn get_num_frames_received(&self) -> i32 {
        self.num_frames_received
    }
}
