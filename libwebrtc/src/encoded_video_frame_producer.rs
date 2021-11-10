use crossbeam_channel::{unbounded, Receiver, Sender};
use libwebrtc_sys::video_encoder::VideoEncoderImpl;
use log::{debug, warn};

use crate::{
    error::{Result, WebRTCError},
    now::now,
    ok_or_return,
    video_codec::VideoCodec,
    video_encoder::{
        EncodeResult, EncodedImageOutput, FrameTypes, VideoEncoder, VideoEncoderFactory,
        VideoEncoderSettings,
    },
    video_frame::{EncodedVideoFrame, RawVideoFrame},
};

pub const DEFAULT_WIDTH: i32 = 720;
pub const DEFAULT_HEIGHT: i32 = 480;
pub const DEFAULT_FPS: u32 = 30;

fn create_video_encoder(codec: VideoCodec, settings: VideoEncoderSettings) -> Result<VideoEncoder> {
    let factory = VideoEncoderFactory::new();
    let codec_name = codec.codec_name();
    let supported_formats = factory.get_supported_formats();
    let format = supported_formats
        .iter()
        .find(|f| f.get_name() == codec_name)
        .ok_or_else(|| WebRTCError::VideoCodecUnsupportedType(codec_name.clone()))?;

    factory.create_encoder(format, codec, settings)
}

/// Represents an active GStreamer pipeline that produces frames.  Frames run
/// through an encoder to produce output suitable for use in the passthrough
/// encoder.
pub struct EncodedFrameProducerProducer {
    /// The gstreamer pipeline that was used to create the producer.
    pub width: i32,
    pub height: i32,
    pub encoded_rx: Receiver<EncodedVideoFrame>,
    pub encoder_err_rx: Receiver<WebRTCError>,
    pub raw_frame_tx: Sender<RawVideoFrame>,
}

impl EncodedFrameProducerProducer {
    pub fn default_config() -> Result<EncodedFrameProducerProducer> {
        let settings = VideoEncoderSettings::default();
        Self::new(
            VideoCodec::vp9(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_FPS),
            settings,
        )
    }

    pub fn new(
        codec: VideoCodec,
        settings: VideoEncoderSettings,
    ) -> Result<EncodedFrameProducerProducer> {
        let (encoded_tx, encoded_rx) = unbounded::<EncodedVideoFrame>();
        let (encoder_err_tx, encoder_err_rx) = unbounded::<WebRTCError>();
        let (encoder_output_rx_tx, encoder_output_rx_rx) =
            unbounded::<Receiver<EncodedImageOutput>>();
        let (raw_frame_tx, raw_frame_rx) = unbounded::<RawVideoFrame>();
        let (init_tx, init_rx) = unbounded::<Result<()>>();
        let width = codec.primary.width;
        let height = codec.primary.height;

        let encoder_thread_err_tx = encoder_err_tx.clone();
        std::thread::spawn(move || {
            let init_result = create_video_encoder(codec, settings);
            let mut encoder = match init_result {
                Ok(encoder) => {
                    ok_or_return!(init_tx.send(Ok(())));
                    encoder
                }
                Err(err) => {
                    ok_or_return!(init_tx.send(Err(err)));
                    return;
                }
            };

            match encoder.take_encoded_image_rx() {
                Some(rx) => {
                    ok_or_return!(encoder_output_rx_tx.send(rx));
                }
                None => {
                    ok_or_return!(init_tx.send(Err(WebRTCError::UnexpectedError(
                        "missing encoded image rx".into(),
                    ))));
                    return;
                }
            };

            while let Ok(frame) = raw_frame_rx.recv() {
                let encode_info =
                    encoder.encode(frame, vec![FrameTypes::KeyFrame, FrameTypes::DeltaFrame]);

                match encode_info {
                    Ok(encode_result) => match encode_result {
                        EncodeResult::RequestKeyFrame => {
                            debug!("request keyframe result from encoder");
                        }
                        EncodeResult::NoOutput => {
                            warn!("Encoded frame with no output");
                        }
                        _ => {}
                    },
                    Err(encode_err) => {
                        ok_or_return!(encoder_thread_err_tx.send(encode_err));
                    }
                }
            }
        });

        let init_result = init_rx.recv()?;
        if init_result.is_err() {
            return Err(init_result.err().unwrap());
        }

        let encoder_output_rx = encoder_output_rx_rx.recv()?;

        let receiver_thread_err_tx = encoder_err_tx;
        std::thread::spawn(move || {
            while let Ok(encoder_result) = encoder_output_rx.recv() {
                let (encoded_image, codec_specific_info) = encoder_result;
                let now_ms = ok_or_return!(now());
                match EncodedVideoFrame::create(encoded_image, codec_specific_info, now_ms) {
                    Ok(encoded_frame) => {
                        ok_or_return!(encoded_tx.send(encoded_frame));
                    }
                    Err(err) => {
                        ok_or_return!(receiver_thread_err_tx.send(err));
                    }
                }
            }
        });

        Ok(Self {
            width,
            height,
            encoded_rx,
            raw_frame_tx,
            encoder_err_rx,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        raw_video_frame_producer::{GStreamerRawFrameProducer, RawFrameProducer},
    };

    use super::*;

    #[test]
    fn test_gstreamer_frame_producer_init() {
        let codec = VideoCodec::vp9(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_FPS);
        let mut raw_frames = GStreamerRawFrameProducer::default_pipeline(&codec).unwrap();
        let encoder =
            EncodedFrameProducerProducer::new(codec, VideoEncoderSettings::default()).unwrap();
        let raw_frames_rx = raw_frames.start().unwrap();

        for _ in 0..10 {
            let frame = raw_frames_rx.recv().unwrap();
            encoder.raw_frame_tx.send(frame).unwrap();
            let _ = encoder.encoded_rx.recv().unwrap();
        }
    }
}
