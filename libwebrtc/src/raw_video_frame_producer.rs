use crossbeam_channel::{select, Receiver, Sender};
use log::debug;

use crate::{
    error::{Result, WebRTCError},
    now, ok_or_return,
    video_codec::VideoCodec,
    video_frame::RawVideoFrame,
};

/// Helper trait for interfaces which can produce raw video frames.
pub trait RawFrameProducer {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn fps(&self) -> u32;
    fn start(&mut self) -> Result<Receiver<RawVideoFrame>>;
    fn cancel(&self);
}

/// Abstraction over the media_pipeline crate specific to producing values from
/// a gstreamer pipeline and converting them into raw I420 frames.
pub struct GStreamerRawFrameProducer {
    pub pipeline: String,
    pub width: i32,
    pub height: i32,
    pub fps: u32,
    cancel_rx: Option<Receiver<()>>,
    cancel_tx: Sender<()>,
}

impl GStreamerRawFrameProducer {
    pub fn default_pipeline(codec: &VideoCodec) -> Result<Self> {
        Self::new("videotestsrc pattern=snow".into(), codec)
    }

    pub fn new(partial_pipeline: String, codec: &VideoCodec) -> Result<Self> {
        let (cancel_tx, cancel_rx) = crossbeam_channel::unbounded::<()>();
        let width = codec.primary.width;
        let height = codec.primary.height;
        let fps = codec.primary.max_frame_rate;
        let pipeline = format!(
            "{} ! video/x-raw,format=(string)I420,width={},height={},framerate={}/1",
            partial_pipeline, width, height, fps
        );

        Ok(Self {
            cancel_rx: Some(cancel_rx),
            cancel_tx,
            width,
            pipeline,
            height,
            fps,
        })
    }
}

impl RawFrameProducer for GStreamerRawFrameProducer {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn fps(&self) -> u32 {
        self.fps
    }

    fn cancel(&self) {
        ok_or_return!(self.cancel_tx.send(()));
    }

    fn start(&mut self) -> Result<Receiver<RawVideoFrame>> {
        let rx: crossbeam_channel::Receiver<bytes::BytesMut> =
            media_pipeline::create_and_start_appsink_pipeline(self.pipeline.as_str())?;

        let (result_tx, result_rx) = crossbeam_channel::bounded::<RawVideoFrame>(100);

        let cancel_rx = self
            .cancel_rx
            .take()
            .ok_or_else(|| WebRTCError::UnexpectedError("cannot call start twice".into()))?;

        let width = self.width;
        let height = self.height;

        std::thread::spawn(move || {
            loop {
                select! {
                    recv(rx) -> buf_result => {
                        let buf = ok_or_return!(buf_result);
                        let now_ms = ok_or_return!(now::now());

                        // XXX: Creating a raw frame should never fail...
                        let raw_frame = ok_or_return!(RawVideoFrame::create(
                            width,
                            height,
                            now_ms,
                            buf.into()));

                        ok_or_return!(result_tx.send(raw_frame));
                    },
                    recv(cancel_rx) -> _ => {
                        debug!("terminating gstreamer raw video frame producer");
                        break;
                    }
                }
            }
        });

        Ok(result_rx)
    }
}

/// Clean up after gstreamer on drop.
impl Drop for GStreamerRawFrameProducer {
    fn drop(&mut self) {
        ok_or_return!(self.cancel_tx.send(()));
    }
}

// Tests are primarily written in the encoded_video_frame_producer

#[test]
fn test_drop() {
    let codec = VideoCodec::vp9(
        crate::encoded_video_frame_producer::DEFAULT_WIDTH,
        crate::encoded_video_frame_producer::DEFAULT_HEIGHT,
        crate::encoded_video_frame_producer::DEFAULT_FPS,
    );
    {
        let mut producer = GStreamerRawFrameProducer::default_pipeline(&codec).unwrap();
        let _ = producer.start().unwrap();
    }
}

#[test]
fn test_get_a_frame() {
    let codec = VideoCodec::vp9(
        crate::encoded_video_frame_producer::DEFAULT_WIDTH,
        crate::encoded_video_frame_producer::DEFAULT_HEIGHT,
        crate::encoded_video_frame_producer::DEFAULT_FPS,
    );
    let mut producer = GStreamerRawFrameProducer::default_pipeline(&codec).unwrap();
    let rx = producer.start().unwrap();
    let _frame = rx.recv().unwrap();
}
