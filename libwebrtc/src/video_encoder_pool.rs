use std::{sync::Arc, time::SystemTime};

use crossbeam_channel::{select, Sender};
use cxx::UniquePtr;
use dashmap::DashMap;
use libwebrtc_sys::ffi::{
    ArcasEncodedImageCallback, ArcasVideoCodec, ArcasVideoEncoderRateControlParameters,
};
use log::debug;

use crate::{
    encoded_video_frame_producer::DEFAULT_FPS,
    error::Result,
    raw_video_frame_producer::{GStreamerRawFrameProducer, RawFrameProducer},
    reactive_video_encoder::DEFAULT_ENCODING,
    video_codec::VideoCodec,
    video_encoder::{FrameTypes, VideoEncoderFactory},
};

pub enum VideoEncoderPoolRequest {
    Create {
        controller_id: String,
        id: String,
        codec: UniquePtr<ArcasVideoCodec>,
        number_of_cores: i32,
        max_payload_size: usize,
        rate: UniquePtr<ArcasVideoEncoderRateControlParameters>,
        callback: UniquePtr<ArcasEncodedImageCallback>,
    },
    Release {
        controller_id: String,
        id: String,
    },
}

pub struct VideoEncoderPoolController {
    callbacks: Arc<DashMap<String, UniquePtr<ArcasEncodedImageCallback>>>,
}

impl VideoEncoderPoolController {
    pub fn new() -> Self {
        Self {
            callbacks: Arc::new(DashMap::new()),
        }
    }

    pub fn remove_callback(&self, id: String) {
        self.callbacks.remove(&id);
    }

    pub(crate) fn add_callback(&self, id: String, callback: UniquePtr<ArcasEncodedImageCallback>) {
        self.callbacks.insert(id, callback);
    }

    pub(crate) fn start(
        &self,
        id: String,
        number_of_cores: i32,
        max_payload_size: usize,
        cxx_callback: UniquePtr<ArcasEncodedImageCallback>,
        cxx_codec: UniquePtr<ArcasVideoCodec>,
        cxx_rate: UniquePtr<ArcasVideoEncoderRateControlParameters>,
    ) {
        self.callbacks.insert(id, cxx_callback);
        let callbacks = self.callbacks.clone();
        // spawn the actual encoder thread to deal with the encoding...
        std::thread::spawn(move || {
            // XXX: Hack should we really spawn one factory per encoder we need?
            let video_encoder_factory = Arc::new(VideoEncoderFactory::new());
            // XXX: hack get vp9 sdp format
            let formats = video_encoder_factory.get_supported_formats();
            let format = formats
                .iter()
                .find(|value| value.get_name() == DEFAULT_ENCODING)
                .unwrap();
            let mut encoder = video_encoder_factory
                .create_encoder_without_init(format)
                .unwrap();
            let settings = libwebrtc_sys::ffi::create_arcas_video_encoder_settings(
                true,
                number_of_cores,
                max_payload_size,
            );
            encoder
                .encoder
                .init_encode(cxx_codec.as_ref().unwrap(), settings.as_ref().unwrap());
            encoder.encoder.set_rates(cxx_rate.as_ref().unwrap());
            let codec_raw = VideoCodec::vp8_default();
            let mut pipeline = GStreamerRawFrameProducer::default_pipeline(&codec_raw).unwrap();
            let rx = pipeline.start().unwrap();

            let encode_rx = encoder.take_encoded_image_rx().unwrap();
            std::thread::spawn(move || {
                let mut encode = SystemTime::now();
                while let Ok(result) = encode_rx.recv() {
                    let now = SystemTime::now();
                    debug!("encode the thing! {:?}", encode.elapsed().unwrap());
                    let (encoded_frame, codec_specific_info) = result;
                    debug!("Distributing frame to encoders");
                    {
                        callbacks.iter().for_each(|callback_ref| unsafe {
                            callback_ref.value().on_encoded_image(
                                encoded_frame.as_ref().unwrap(),
                                codec_specific_info.as_ref().unwrap(),
                            );
                        });
                        debug!(
                            "distribute time: {:?}",
                            SystemTime::now().duration_since(now)
                        );
                    }
                    encode = SystemTime::now();
                }
            });

            let mut increment = 0;
            let mut last_keyframe = SystemTime::now();
            while let Ok(frame) = rx.recv() {
                let frames = if increment % DEFAULT_FPS == 0 {
                    last_keyframe = SystemTime::now();
                    vec![FrameTypes::KeyFrame, FrameTypes::DeltaFrame]
                } else {
                    vec![FrameTypes::DeltaFrame]
                };
                debug!(
                    "frame: inc={:?} {:?} (last={:?})",
                    increment,
                    &frames,
                    SystemTime::now().duration_since(last_keyframe)
                );
                encoder.encode(frame, frames).unwrap();
                increment += 1;
            }
        });
    }
}

impl Default for VideoEncoderPoolController {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct VideoEncoderPool {}

impl VideoEncoderPool {
    pub fn create() -> Result<(Self, Sender<VideoEncoderPoolRequest>)> {
        let encoder_map: Arc<DashMap<String, Arc<VideoEncoderPoolController>>> =
            Arc::new(DashMap::new());
        let (request_tx, request_rx) = crossbeam_channel::unbounded::<VideoEncoderPoolRequest>();

        // This thread is in charge of spawning individual encoder threads in repsonse to requests.
        std::thread::spawn(move || loop {
            select! {
                recv(request_rx) -> req => {
                    match req {
                        Ok(request) => {
                            match request {
                                VideoEncoderPoolRequest::Create { id, controller_id, codec, number_of_cores, max_payload_size, rate, callback } => {
                                    match encoder_map.get(&controller_id) {
                                        Some(value) => {
                                            value.add_callback(id, callback);
                                            continue;
                                        },
                                        None => {
                                            let controller = Arc::new(VideoEncoderPoolController::new());
                                            encoder_map.insert(controller_id, controller.clone());
                                            controller.start(
                                                id,
                                                number_of_cores,
                                                max_payload_size,
                                                callback,
                                                codec,
                                                rate,
                                            );
                                        }
                                    }
                                },
                                VideoEncoderPoolRequest::Release { controller_id, id } => {
                                    match encoder_map.get(&controller_id) {
                                        Some(value) => {
                                            value.remove_callback(id);
                                            continue;
                                        },
                                        None => {
                                            continue;
                                        }
                                    }
                                },
                            }
                        },
                        Err(err) => {
                            debug!("Failed to process encoder request: {:?}", err);
                            continue;
                        }
                    }
                }
            }
        });

        Ok((Self {}, request_tx))
    }
}
