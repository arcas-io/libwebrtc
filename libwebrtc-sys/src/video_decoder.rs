pub trait VideoDecoderImpl {
    fn decode(
        &mut self,
        image: &crate::ffi::ArcasCxxEncodedImage,
        missing_frames: bool,
        render_times_ms: i64,
    ) -> i32;

    fn release(&mut self) -> i32;

    fn get_num_frames_received(&self) -> i32;
}

pub struct VideoDecoderProxy {
    decoder: Box<dyn VideoDecoderImpl>,
}

impl VideoDecoderProxy {
    pub fn new(decoder: Box<dyn VideoDecoderImpl>) -> Self {
        Self { decoder }
    }

    pub fn decode(
        &mut self,
        image: &crate::ffi::ArcasCxxEncodedImage,
        missing_frames: bool,
        render_times_ms: i64,
    ) -> i32 {
        self.decoder.decode(image, missing_frames, render_times_ms)
    }

    pub fn release(&mut self) -> i32 {
        self.decoder.release()
    }

    pub fn get_num_frames_received(&self) -> i32 {
        self.decoder.get_num_frames_received()
    }
}
