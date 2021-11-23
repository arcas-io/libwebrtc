use cxx::{SharedPtr, UniquePtr};
use libwebrtc_sys::ffi::{ArcasCxxVideoCodecType, ArcasSpatialLayer, ArcasVideoCodec};

use crate::{
    encoded_video_frame_producer::{DEFAULT_FPS, DEFAULT_HEIGHT, DEFAULT_WIDTH},
    error::{Result, WebRTCError},
};

const MAX_FRAMERATE: u32 = 60;
const DEFAULT_MAX_BITRATE_KBS: u32 = 20000;
const DEFAULT_TARGET_BITRATE_KBS: u32 = 1000;
const DEFAULT_MIN_BITRATE_KBS: u32 = 500;

// QP max taken from some test in libwebrtc...
const DEFAULT_QP_MAX: u32 = 56;

/**
 * NOTE: All magic values in this file come from video_encoder.cc in libwebrtc.
 */

/**
 * Common configuration to video codec settings.
 *
 * These are used both for primary settings as well as  simulcast layers and temporal layers.
 */

#[derive(Debug, Clone)]
pub struct VideoCodecConfig {
    pub width: i32,
    pub height: i32,
    pub max_bitrate_kbs: u32,
    pub target_bitrate_kbs: u32,
    pub min_bitrate_kbs: u32,
    pub max_frame_rate: u32,
    pub qp_max: u32,
}

impl VideoCodecConfig {
    fn to_spatial_layer(&self) -> SharedPtr<ArcasSpatialLayer> {
        let spatial_layer = libwebrtc_sys::ffi::create_arcas_spatial_layer();
        spatial_layer.set_width(self.width);
        spatial_layer.set_height(self.height);
        spatial_layer.set_max_bitrate(self.max_bitrate_kbs);
        spatial_layer.set_target_bitrate(self.target_bitrate_kbs);
        spatial_layer.set_min_bitrate(self.min_bitrate_kbs);
        spatial_layer.set_max_framerate(self.max_frame_rate as f32);
        spatial_layer.set_qp_max(self.qp_max);
        spatial_layer.set_active(true);
        spatial_layer
    }

    fn set_on_video_codec(&self, video_codec: &ArcasVideoCodec) {
        video_codec.set_width(self.width as u16);
        video_codec.set_height(self.height as u16);
        video_codec.set_max_bitrate(self.max_bitrate_kbs);
        video_codec.set_min_bitrate(self.min_bitrate_kbs);
        video_codec.set_max_framerate(self.max_frame_rate);
        video_codec.set_qp_max(self.qp_max);
        video_codec.set_active(true);
    }
}

impl Default for VideoCodecConfig {
    fn default() -> Self {
        VideoCodecConfig {
            width: 640,
            height: 480,
            qp_max: DEFAULT_QP_MAX,
            max_bitrate_kbs: DEFAULT_MAX_BITRATE_KBS,
            target_bitrate_kbs: DEFAULT_TARGET_BITRATE_KBS,
            min_bitrate_kbs: DEFAULT_MIN_BITRATE_KBS,
            max_frame_rate: MAX_FRAMERATE,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VideoCodecDescriptionSpatialLayer {
    width: i32,
    height: i32,
    max_frame_rate: u32,
    number_of_temporal_layers: u8,
    max_bitrate: u32,
    target_bitrate: u32,
    min_bitrate: u32,
    qp_max: u32,
    active: bool,
}

impl From<&ArcasSpatialLayer> for VideoCodecDescriptionSpatialLayer {
    fn from(layer: &ArcasSpatialLayer) -> Self {
        Self {
            width: layer.get_width(),
            height: layer.get_height(),
            max_frame_rate: layer.get_max_framerate() as u32,
            number_of_temporal_layers: layer.get_number_of_temporal_layers(),
            max_bitrate: layer.get_max_bitrate(),
            target_bitrate: layer.get_target_bitrate(),
            min_bitrate: layer.get_min_bitrate(),
            qp_max: layer.get_qp_max(),
            active: layer.get_active(),
        }
    }
}

/// Use this for when a codec description needs to be hashable.  Built initially
/// for passthrough/reactive encoding where we need to create one encoder per
/// configuration type.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VideoCodecDescription {
    scalability_mode: String,
    width: i32,
    height: i32,
    max_bitrate: u32,
    min_bitrate: u32,
    start_bitrate: u32,
    max_framerate: u32,
    active: bool,
    qp_max: u32,
    spatial_layers: Vec<VideoCodecDescriptionSpatialLayer>,
    simulcast_streams: Vec<VideoCodecDescriptionSpatialLayer>,
}

impl VideoCodecDescription {
    pub fn create_from_codec(codec: &UniquePtr<ArcasVideoCodec>) -> Self {
        Self {
            scalability_mode: codec.get_scalability_mode(),
            width: codec.get_width(),
            height: codec.get_height(),
            max_bitrate: codec.get_max_bitrate(),
            min_bitrate: codec.get_min_bitrate(),
            start_bitrate: codec.get_start_bitrate(),
            max_framerate: codec.get_max_framerate(),
            active: codec.get_active(),
            qp_max: codec.get_qp_max(),
            spatial_layers: codec
                .spatial_layers()
                .into_iter()
                .map(|layer| layer.into())
                .collect(),
            simulcast_streams: codec
                .simulcast_streams()
                .into_iter()
                .map(|layer| layer.into())
                .collect(),
        }
    }
}

pub struct VideoCodec {
    pub codec_type: libwebrtc_sys::ffi::ArcasCxxVideoCodecType,
    pub primary: VideoCodecConfig,
    pub spatial_layers: Vec<VideoCodecConfig>,
    pub simulcast_streams: Vec<VideoCodecConfig>,
}

impl VideoCodec {
    pub fn vp9_default() -> Self {
        Self::vp9(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_FPS)
    }
    pub fn vp8_default() -> Self {
        Self::vp8(DEFAULT_WIDTH, DEFAULT_HEIGHT, DEFAULT_FPS)
    }
    pub fn vp9(width: i32, height: i32, fps: u32) -> Self {
        VideoCodec {
            codec_type: libwebrtc_sys::ffi::ArcasCxxVideoCodecType::kVideoCodecVP9,
            primary: VideoCodecConfig {
                width,
                height,
                max_frame_rate: fps,
                ..VideoCodecConfig::default()
            },
            spatial_layers: vec![VideoCodecConfig {
                width,
                height,
                max_frame_rate: fps,
                ..VideoCodecConfig::default()
            }],
            simulcast_streams: vec![VideoCodecConfig {
                width,
                height,
                max_frame_rate: fps,
                ..VideoCodecConfig::default()
            }],
        }
    }

    pub fn vp8(width: i32, height: i32, fps: u32) -> Self {
        VideoCodec {
            codec_type: libwebrtc_sys::ffi::ArcasCxxVideoCodecType::kVideoCodecVP8,
            primary: VideoCodecConfig {
                width,
                height,
                max_frame_rate: fps,
                ..VideoCodecConfig::default()
            },
            spatial_layers: vec![VideoCodecConfig {
                width,
                height,
                max_frame_rate: fps,
                ..VideoCodecConfig::default()
            }],
            simulcast_streams: vec![VideoCodecConfig {
                width,
                height,
                max_frame_rate: fps,
                ..VideoCodecConfig::default()
            }],
        }
    }

    pub fn new(
        codec_type: libwebrtc_sys::ffi::ArcasCxxVideoCodecType,
        primary: VideoCodecConfig,
        spatial_layers: Vec<VideoCodecConfig>,
        simulcast_streams: Vec<VideoCodecConfig>,
    ) -> Self {
        Self {
            codec_type,
            primary,
            spatial_layers,
            simulcast_streams,
        }
    }

    pub fn codec_name(&self) -> String {
        match self.codec_type {
            ArcasCxxVideoCodecType::kVideoCodecVP8 => "VP8".to_string(),
            ArcasCxxVideoCodecType::kVideoCodecVP9 => "VP9".to_string(),
            ArcasCxxVideoCodecType::kVideoCodecAV1 => "AV1".to_string(),
            ArcasCxxVideoCodecType::kVideoCodecH264 => "H264".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    pub(crate) fn to_arcas_video_codec(&self) -> Result<SharedPtr<ArcasVideoCodec>> {
        let codec = libwebrtc_sys::ffi::create_arcas_video_codec();
        let codec_ref = codec
            .as_ref()
            .ok_or_else(|| WebRTCError::CXXUnwrapError("Arcas Video Codec".into()))?;
        self.primary.set_on_video_codec(codec_ref);

        match self.codec_type {
            ArcasCxxVideoCodecType::kVideoCodecVP8 => {
                set_default_vp8_settings(codec_ref);
            }
            ArcasCxxVideoCodecType::kVideoCodecVP9 => set_default_vp9_settings(codec_ref),
            ArcasCxxVideoCodecType::kVideoCodecH264 => set_default_h264_settings(codec_ref),
            _ => {
                return Err(WebRTCError::VideoCodecUnsupportedType(format!(
                    "{:?}",
                    self.codec_type
                )));
            }
        }

        let primary_spatial_layer = self.primary.to_spatial_layer();
        let primary_spatial_layer_ref = primary_spatial_layer
            .as_ref()
            .ok_or_else(|| WebRTCError::CXXUnwrapError("Arcas Spatial Layer".into()))?;

        codec.set_spatial_layer_at(0, primary_spatial_layer_ref);

        let mut spatial_index = 1;
        for layer in self.spatial_layers.iter() {
            let spatial_layer = layer.to_spatial_layer();
            let spatial_layer_ref = spatial_layer
                .as_ref()
                .ok_or_else(|| WebRTCError::CXXUnwrapError("Arcas Spatial Layer".into()))?;
            codec.set_spatial_layer_at(spatial_index, spatial_layer_ref);
            spatial_index += 1;
        }

        let mut simulcast_idx = 0u8;
        #[allow(clippy::explicit_counter_loop)]
        for (_, simulcast_stream) in self.simulcast_streams.iter().enumerate() {
            let simulcast_stream = simulcast_stream.to_spatial_layer();
            let simulcast_stream_ref = simulcast_stream
                .as_ref()
                .ok_or_else(|| WebRTCError::CXXUnwrapError("Arcas Spatial Layer".into()))?;
            codec.set_simulcast_stream_at(simulcast_idx, simulcast_stream_ref);
            simulcast_idx += 1;
        }

        Ok(codec)
    }
}

impl Default for VideoCodec {
    fn default() -> Self {
        VideoCodec {
            primary: VideoCodecConfig::default(),
            spatial_layers: vec![VideoCodecConfig::default()],
            simulcast_streams: vec![],
            codec_type: ArcasCxxVideoCodecType::kVideoCodecVP8,
        }
    }
}

fn set_default_vp8_settings(codec: &ArcasVideoCodec) {
    codec.set_codec_type(libwebrtc_sys::ffi::ArcasCxxVideoCodecType::kVideoCodecVP8);
    codec.vp8_set_number_of_temporal_layers(1);
    codec.vp8_set_denoising_on(true);
    codec.vp8_set_automatic_resize_on(true);
    codec.vp8_set_frame_dropping_on(true);
    codec.vp8_set_key_frame_interval(3000);
}

fn set_default_vp9_settings(codec: &ArcasVideoCodec) {
    codec.set_codec_type(libwebrtc_sys::ffi::ArcasCxxVideoCodecType::kVideoCodecVP9);
    codec.vp9_set_number_of_temporal_layers(1);
    codec.vp9_set_denoising_on(true);
    codec.vp9_set_frame_dropping_on(true);
    codec.vp9_set_key_frame_interval(3000);
    codec.vp9_set_adaptive_qp_on(true);
    codec.vp9_set_automatic_resize_on(true);
    codec.vp9_set_number_of_spatial_layers(1);
    codec.vp9_set_flexible_mode(true);
    codec.vp9_set_inter_layer_pred(libwebrtc_sys::ffi::ArcasCxxInterLayerPredMode::kOn);
}

fn set_default_h264_settings(codec: &ArcasVideoCodec) {
    codec.set_codec_type(libwebrtc_sys::ffi::ArcasCxxVideoCodecType::kVideoCodecH264);
    codec.h264_set_frame_dropping_on(true);
    codec.h264_set_key_frame_interval(3000);
    codec.h264_set_number_of_temporal_layers(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vp9_default_creation() {
        let vp9 = VideoCodec::vp9(720, 480, 60u32);
        let _codec = vp9.to_arcas_video_codec().unwrap();
    }
}
