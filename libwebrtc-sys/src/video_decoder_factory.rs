use cxx::{CxxVector, UniquePtr};

use crate::VideoDecoderProxy;

pub trait VideoDecoderFactoryImpl {
    /// Returns a list of supported video formats in order of preference, to use
    /// for signaling etc.
    fn get_supported_formats(&self) -> UniquePtr<CxxVector<crate::ffi::ArcasCxxSdpVideoFormat>>;

    /// Query whether the specifed format is supported or not and if it will be
    /// power efficient, which is currently interpreted as if there is support for
    /// hardware acceleration.
    /// The parameter `reference_scaling` is used to query support for prediction
    /// across spatial layers. An example where support for reference scaling is
    /// needed is if the video stream is produced with a scalability mode that has
    /// a dependency between the spatial layers. See
    /// https://w3c.github.io/webrtc-svc/#scalabilitymodes* for a specification of
    /// different scalabilty modes. NOTE: QueryCodecSupport is currently an
    /// experimental feature that is subject to change without notice.
    fn query_codec_support(
        &self,
        format: &crate::ffi::ArcasCxxSdpVideoFormat,
        reference_scaling: bool,
    ) -> crate::ffi::ArcasVideoDecoderFactoryCodecSupport;

    /// Creates a VideoDecoder for the specified format.
    fn create_video_decoder(
        &mut self,
        format: &crate::ffi::ArcasCxxSdpVideoFormat,
    ) -> Box<VideoDecoderProxy>;
}

pub struct VideoDecoderFactoryProxy {
    api: Box<dyn VideoDecoderFactoryImpl>,
}

impl VideoDecoderFactoryProxy {
    pub fn new(api: Box<dyn VideoDecoderFactoryImpl>) -> Self {
        Self { api }
    }

    pub fn get_supported_formats(
        &self,
    ) -> UniquePtr<CxxVector<crate::ffi::ArcasCxxSdpVideoFormat>> {
        self.api.get_supported_formats()
    }

    pub fn query_codec_support(
        &self,
        format: &crate::ffi::ArcasCxxSdpVideoFormat,
        reference_scaling: bool,
    ) -> crate::ffi::ArcasVideoDecoderFactoryCodecSupport {
        self.api.query_codec_support(format, reference_scaling)
    }

    pub fn create_video_decoder(
        &mut self,
        format: &crate::ffi::ArcasCxxSdpVideoFormat,
    ) -> Box<VideoDecoderProxy> {
        self.api.create_video_decoder(format)
    }
}
