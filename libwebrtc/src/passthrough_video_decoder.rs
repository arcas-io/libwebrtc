use libwebrtc_sys::video_decoder::VideoDecoderImpl;

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
    ) -> i32 {
        self.num_frames_received += 1;
        0
    }

    fn release(&mut self) -> i32 {
        0
    }

    fn get_num_frames_received(&self) -> i32 {
        self.num_frames_received
    }
}
