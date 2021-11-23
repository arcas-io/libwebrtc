use std::{thread::sleep, time::Duration};

use crossbeam_channel::{unbounded, Receiver};

use crate::{error::Result, now, ok_or_return, video_frame::EmptyVideoFrame};

// Helper utility for generating empty frame data for the passthrough/reactive
// encoders.  This is useful as it tricks libwebrtc into thinking there is real
// image data coming through the pipeline.
pub struct EmptyFrameProducer {
    pub fps: u32,
}

impl EmptyFrameProducer {
    pub fn new(fps: u32) -> Result<Self> {
        Ok(Self { fps })
    }

    pub fn start(&mut self) -> Result<Receiver<EmptyVideoFrame>> {
        let (result_tx, result_rx) = unbounded::<EmptyVideoFrame>();
        let fps = self.fps;

        std::thread::spawn(move || loop {
            let sleep_time = 1_000 / fps as u64;
            let now_ms = ok_or_return!(now::now());

            // XXX: Creating a raw frame should never fail...
            let raw_frame = ok_or_return!(EmptyVideoFrame::create(now_ms));
            ok_or_return!(result_tx.send(raw_frame));

            sleep(Duration::from_millis(sleep_time));
        });

        Ok(result_rx)
    }

    pub fn cancel(&self) {
        // todo: kill the thread...
    }
}
