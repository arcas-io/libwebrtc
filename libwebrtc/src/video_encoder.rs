use crossbeam_channel::{unbounded, Receiver};

use cxx::{SharedPtr, UniquePtr};
use libwebrtc_sys::{
    ffi::{
        create_arcas_video_encoder_factory_from_builtin, create_arcas_video_encoder_settings,
        create_arcas_video_frame_types_collection, ArcasCodecSpecificInfo, ArcasCxxEncodedImage,
        ArcasCxxVideoFrameType, ArcasVideoCodec, ArcasVideoEncoderDropReason,
        ArcasVideoEncoderFactoryWrapper, ArcasVideoEncoderInfo,
        ArcasVideoEncoderRateControlParameters, ArcasVideoEncoderSettings,
        ArcasVideoEncoderWrapper,
    },
    VIDEO_CODEC_ENCODER_FAILURE, VIDEO_CODEC_ERROR, VIDEO_CODEC_ERR_PARAMETER,
    VIDEO_CODEC_ERR_SIMULCAST_PARAMETERS_NOT_SUPPORTED, VIDEO_CODEC_FALLBACK_SOFTWARE,
    VIDEO_CODEC_MEMORY, VIDEO_CODEC_NO_OUTPUT, VIDEO_CODEC_OK, VIDEO_CODEC_OK_REQUEST_KEYFRAME,
    VIDEO_CODEC_TARGET_BITRATE_OVERSHOOT, VIDEO_CODEC_UNINITIALIZED,
};
use log::error;

use crate::{
    error::{self, Result, WebRTCError},
    ok_or_return,
    sdp_video_format::SDPVideoFormat,
    video_codec::VideoCodec,
    video_frame::AsCxxVideoFrame,
};

pub type EncodedImageOutput = (
    UniquePtr<ArcasCxxEncodedImage>,
    UniquePtr<ArcasCodecSpecificInfo>,
);

#[derive(Debug)]
pub enum EncodeResult {
    Ok,
    RequestKeyFrame,
    NoOutput,
}

pub enum FrameTypes {
    KeyFrame,
    EmptyFrame,
    DeltaFrame,
}

impl From<&FrameTypes> for ArcasCxxVideoFrameType {
    fn from(frame_type: &FrameTypes) -> Self {
        match frame_type {
            FrameTypes::KeyFrame => ArcasCxxVideoFrameType::kVideoFrameKey,
            FrameTypes::EmptyFrame => ArcasCxxVideoFrameType::kEmptyFrame,
            FrameTypes::DeltaFrame => ArcasCxxVideoFrameType::kVideoFrameDelta,
        }
    }
}

pub struct VideoEncoderSettings {
    pub loss_notification: bool,
    pub number_of_cores: i32,
    pub max_payload_size: usize,
}

impl VideoEncoderSettings {
    pub(crate) fn to_arcas_video_encoder_settings(&self) -> SharedPtr<ArcasVideoEncoderSettings> {
        create_arcas_video_encoder_settings(
            self.loss_notification,
            self.number_of_cores,
            self.max_payload_size,
        )
    }
}

impl Default for VideoEncoderSettings {
    fn default() -> Self {
        VideoEncoderSettings {
            loss_notification: true,
            number_of_cores: 4,
            max_payload_size: 1460,
        }
    }
}

pub struct VideoEncoderFactory {
    factory: UniquePtr<ArcasVideoEncoderFactoryWrapper>,
}

impl VideoEncoderFactory {
    pub fn new() -> Self {
        let factory: UniquePtr<ArcasVideoEncoderFactoryWrapper> =
            create_arcas_video_encoder_factory_from_builtin();
        Self { factory }
    }

    pub fn get_supported_formats(&self) -> Vec<SDPVideoFormat> {
        let formats = self.factory.get_supported_formats();
        let mut result = Vec::new();
        for format in formats.iter() {
            result.push(SDPVideoFormat::new_from_cxx(format.clone()));
        }

        result
    }
    pub fn create_encoder(
        &self,
        format: &SDPVideoFormat,
        codec: VideoCodec,
        settings: VideoEncoderSettings,
    ) -> Result<VideoEncoder> {
        let (drop_tx, drop_rx) = unbounded();
        let (image_tx, image_rx) = unbounded();
        let wrapper = Box::new(libwebrtc_sys::EncodedImageCallbackHandler::new(
            Box::new(move |drop_reason| {
                ok_or_return!(drop_tx.send(drop_reason));
            }),
            Box::new(move |encoded_image, codec_info| {
                ok_or_return!(image_tx.send((encoded_image, codec_info)));
            }),
        ));
        let cxx_encoder = self.factory.create_encoder(format.as_ref()?, wrapper);
        let encoder = VideoEncoder::new(codec, settings, cxx_encoder, drop_rx, image_rx)?;
        Ok(encoder)
    }
}

impl Default for VideoEncoderFactory {
    fn default() -> Self {
        Self::new()
    }
}

pub struct VideoEncoder {
    pub video_codec: VideoCodec,
    encoder: UniquePtr<ArcasVideoEncoderWrapper>,
    drop_rx: Option<Receiver<ArcasVideoEncoderDropReason>>,

    // Hold reference to C++ object
    #[allow(dead_code)]
    arcas_video_codec: SharedPtr<ArcasVideoCodec>,
    // Hold reference to C++ object
    #[allow(dead_code)]
    arcas_video_encoder_settings: SharedPtr<ArcasVideoEncoderSettings>,
    // Hold reference to C++ object
    #[allow(dead_code)]
    arcas_rate_control_params: SharedPtr<ArcasVideoEncoderRateControlParameters>,

    encoded_image_rx: Option<Receiver<EncodedImageOutput>>,
}

impl VideoEncoder {
    pub fn new(
        video_codec: VideoCodec,
        settings: VideoEncoderSettings,
        encoder: UniquePtr<ArcasVideoEncoderWrapper>,
        drop_rx: Receiver<ArcasVideoEncoderDropReason>,
        encoded_image_rx: Receiver<EncodedImageOutput>,
    ) -> Result<Self> {
        let arcas_video_codec = video_codec.to_arcas_video_codec()?;
        let arcas_video_codec_ref = arcas_video_codec.as_ref().ok_or_else(|| {
            error::WebRTCError::CXXUnwrapError("failed to get ArcasVideoCodec".into())
        })?;
        let arcas_video_encoder_settings = settings.to_arcas_video_encoder_settings();
        let arcas_video_encoder_settings_ref =
            arcas_video_encoder_settings.as_ref().ok_or_else(|| {
                error::WebRTCError::CXXUnwrapError("failed to get ArcasVideoEncoderSettings".into())
            })?;

        let value = encoder.init_encode(arcas_video_codec_ref, arcas_video_encoder_settings_ref);

        if value != *VIDEO_CODEC_OK {
            return Err(error::WebRTCError::VideoEncoderFailedInit);
        }

        let mut bitrate_allocation = libwebrtc_sys::ffi::create_video_bitrate_allocation();
        let bitrate_allocation_write_ref = bitrate_allocation.as_mut().ok_or_else(|| {
            error::WebRTCError::CXXUnwrapError("failed to get ArcasVideoBitrateAllocation".into())
        })?;

        // XXX: Set the bitrate allocation based on the parameters in the codec for now.
        bitrate_allocation_write_ref.set_bitrate(
            0,
            0,
            video_codec.primary.target_bitrate_kbs * 1000,
        );

        let bitrate_allocation_read_ref = bitrate_allocation.as_ref().ok_or_else(|| {
            error::WebRTCError::CXXUnwrapError("failed to get ArcasVideoBitrateAllocation".into())
        })?;

        let arcas_rate_params =
            libwebrtc_sys::ffi::create_arcas_video_encoder_rate_control_parameters(
                bitrate_allocation_read_ref,
                video_codec.primary.max_frame_rate as f64,
            );
        let arcas_rate_params_ref = arcas_rate_params.as_ref().ok_or_else(|| {
            error::WebRTCError::CXXUnwrapError(
                "failed to get ArcasVideoEncoderRateControlParameters".into(),
            )
        })?;
        encoder.set_rates(arcas_rate_params_ref);

        Ok(Self {
            video_codec,
            encoder,
            arcas_video_codec,
            drop_rx: Some(drop_rx),
            encoded_image_rx: Some(encoded_image_rx),
            arcas_video_encoder_settings,
            arcas_rate_control_params: arcas_rate_params,
        })
    }

    pub fn take_drop_rx(&mut self) -> Option<Receiver<ArcasVideoEncoderDropReason>> {
        self.drop_rx.take()
    }

    /// These are special methods in that they return std::shared_ptr where our
    /// goal is to explicitly hide those in this crate. This is needed mostly to
    /// lower the overhead of encoding.
    pub fn take_encoded_image_rx(&mut self) -> Option<Receiver<EncodedImageOutput>> {
        self.encoded_image_rx.take()
    }

    pub fn get_encoder_info(&self) -> ArcasVideoEncoderInfo {
        self.encoder.get_encoder_info()
    }

    pub fn encode<T: AsCxxVideoFrame>(
        &self,
        video_frame: T,
        frame_types: Vec<FrameTypes>,
    ) -> Result<EncodeResult> {
        let frame_type_list = frame_types
            .iter()
            .map(|frame_type| frame_type.into())
            .collect();
        let frame_types = create_arcas_video_frame_types_collection(frame_type_list);
        let frame_types_ref = frame_types
            .as_ref()
            .ok_or_else(|| WebRTCError::CXXUnwrapError("Frame types collection unwrap".into()))?;

        let output = self
            .encoder
            .encode(video_frame.as_cxx_video_frame_ref()?, frame_types_ref);

        if output == *VIDEO_CODEC_OK_REQUEST_KEYFRAME
            || output == *VIDEO_CODEC_NO_OUTPUT
            || output == *VIDEO_CODEC_OK
        {
            if output == *VIDEO_CODEC_OK_REQUEST_KEYFRAME {
                return Ok(EncodeResult::RequestKeyFrame);
            }

            if output == *VIDEO_CODEC_NO_OUTPUT {
                return Ok(EncodeResult::NoOutput);
            }

            Ok(EncodeResult::Ok)
        } else {
            if output == *VIDEO_CODEC_ERROR {
                return Err(WebRTCError::VideoCodecError);
            }

            if output == *VIDEO_CODEC_MEMORY {
                return Err(WebRTCError::VideoCodecMemory);
            }

            if output == *VIDEO_CODEC_ERR_PARAMETER {
                return Err(WebRTCError::VideoCodecErrParameter);
            }

            if output == *VIDEO_CODEC_UNINITIALIZED {
                return Err(WebRTCError::VideoCodecUninitialized);
            }

            if output == *VIDEO_CODEC_FALLBACK_SOFTWARE {
                return Err(WebRTCError::VideoCodecFallbackSoftware);
            }

            if output == *VIDEO_CODEC_TARGET_BITRATE_OVERSHOOT {
                return Err(WebRTCError::VideoCodecTargetBitrateOvershoot);
            }

            if output == *VIDEO_CODEC_ERR_SIMULCAST_PARAMETERS_NOT_SUPPORTED {
                return Err(WebRTCError::VideoCodecErrSimulcastParamsNotSupported);
            }

            if output == *VIDEO_CODEC_ENCODER_FAILURE {
                return Err(WebRTCError::VideoCodecEncoderFailure);
            }

            Err(WebRTCError::VideoCodecError)
        }
    }
}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        self.encoder.release();
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, time::SystemTime};

    use bytes::{Bytes, BytesMut};

    use super::*;

    #[test]
    fn test_gstreamer_to_video_encoder() {
        // libwebrtc_sys::ffi::set_arcas_log_level(libwebrtc_sys::ffi::LoggingSeverity::LS_VERBOSE);
        // libwebrtc_sys::ffi::set_arcas_log_to_stderr(true);
        let width = 720;
        let height = 480;
        let fps = 60u32;
        let launch = format!("videotestsrc pattern=snow ! videoconvert ! videoscale ! video/x-raw,format=I420,width={},height={}",  width, height);
        let rx: crossbeam_channel::Receiver<bytes::BytesMut> =
            media_pipeline::create_and_start_appsink_pipeline(launch.as_str()).unwrap();

        let (video_tx, video_rx) = mpsc::channel::<BytesMut>();
        let (video_done_tx, video_done_rx) = mpsc::channel::<u8>();
        let complete_tx = video_done_tx;

        // encoder thread...
        std::thread::spawn(move || {
            let encoder_factory = VideoEncoderFactory::new();
            let formats = encoder_factory.get_supported_formats();
            let vp9 = formats.iter().find(|f| f.get_name() == "VP9").unwrap();
            let vp9_codec = VideoCodec::vp9(width, height, fps);
            let settings = VideoEncoderSettings::default();
            let mut vp9_encoder = encoder_factory
                .create_encoder(vp9, vp9_codec, settings)
                .unwrap();

            let info = vp9_encoder.get_encoder_info();
            println!("encoder info:: {:?}", info);

            let encode_rx = vp9_encoder.take_encoded_image_rx().unwrap();
            std::thread::spawn(move || {
                let mut frames = 0;
                for (encoded_image, codec_specific_info) in encode_rx {
                    let len = encoded_image.size();
                    println!(
                        "encoded image size: {:?} {:?}",
                        len,
                        codec_specific_info.get_codec_type()
                    );
                    frames += 1;
                    if frames == 10 {
                        complete_tx.send(1).unwrap();
                        break;
                    }
                }
            });

            while let Ok(buf) = video_rx.recv() {
                let time_ms = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();
                let in_ms = time_ms.as_secs() * 1000 + time_ms.subsec_nanos() as u64 / 1_000_000;
                let buf_bytes: Bytes = buf.into();
                let video_frame =
                    crate::video_frame::RawVideoFrame::create(width, height, in_ms, buf_bytes)
                        .unwrap();

                let _ = vp9_encoder
                    .encode(
                        video_frame,
                        vec![FrameTypes::KeyFrame, FrameTypes::DeltaFrame],
                    )
                    .unwrap();
            }
        });

        std::thread::spawn(move || {
            while let Ok(buf) = rx.recv() {
                video_tx.send(buf).expect("Should send gstreamer buffers");
            }
        });
        video_done_rx.recv().unwrap();
    }
}
