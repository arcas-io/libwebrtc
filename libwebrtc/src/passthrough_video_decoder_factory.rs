use libwebrtc_sys::{
    ffi::{create_sdp_video_format_list, ArcasSdpVideoFormatInit, ArcasSdpVideoFormatVecInit},
    video_decoder_factory::VideoDecoderFactoryImpl,
    VideoDecoderProxy,
};

use crate::passthrough_video_decoder::PassthroughVideoDecoder;

#[derive(Default)]
pub struct PassthroughVideoDecoderFactory {}

impl VideoDecoderFactoryImpl for PassthroughVideoDecoderFactory {
    fn get_supported_formats(
        &self,
    ) -> cxx::UniquePtr<cxx::CxxVector<libwebrtc_sys::ffi::ArcasCxxSdpVideoFormat>> {
        let list = ArcasSdpVideoFormatVecInit {
            list: vec![
                ArcasSdpVideoFormatInit {
                    name: "VP9".to_owned(),
                    parameters: vec![],
                },
                ArcasSdpVideoFormatInit {
                    name: "VP8".to_owned(),
                    parameters: vec![],
                },
                ArcasSdpVideoFormatInit {
                    name: "H264".to_owned(),
                    parameters: vec![],
                },
            ],
        };
        create_sdp_video_format_list(list)
    }

    fn query_codec_support(
        &self,
        _format: &libwebrtc_sys::ffi::ArcasCxxSdpVideoFormat,
        _reference_scaling: bool,
    ) -> libwebrtc_sys::ffi::ArcasVideoDecoderFactoryCodecSupport {
        libwebrtc_sys::ffi::ArcasVideoDecoderFactoryCodecSupport {
            is_supported: true,
            is_power_efficient: true,
        }
    }

    fn create_video_decoder(
        &mut self,
        _format: &libwebrtc_sys::ffi::ArcasCxxSdpVideoFormat,
    ) -> Box<libwebrtc_sys::VideoDecoderProxy> {
        Box::from(VideoDecoderProxy::new(Box::from(
            PassthroughVideoDecoder::default(),
        )))
    }
}

impl PassthroughVideoDecoderFactory {
    pub fn new() -> Self {
        Self::default()
    }
}
