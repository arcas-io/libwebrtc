use cxx::{CxxVector, UniquePtr};

use crate::{
    ffi::{
        ArcasCxxDataRate, ArcasCxxSdpVideoFormat, ArcasVideoEncoderFactoryCodecInfo,
        ArcasVideoEncoderFactoryCodecSupport,
    },
    VideoEncoderProxy,
};

pub trait VideoEncoderFactoryImpl {
    /// Returns a list of supported video formats in order of preference, to use
    /// for signaling etc.
    fn get_supported_formats(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

    /// Returns a list of supported video formats in order of preference, that can
    /// also be tagged with additional information to allow the VideoEncoderFactory
    /// to separate between different implementations when CreateVideoEncoder is
    /// called.
    fn get_implementations(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

    /// Returns information about how this format will be encoded. The specified
    /// format must be one of the supported formats by this factory.
    fn query_video_encoder(
        &self,
        format: &ArcasCxxSdpVideoFormat,
    ) -> ArcasVideoEncoderFactoryCodecInfo;

    /// Query whether the specifed format is supported or not and if it will be
    /// power efficient, which is currently interpreted as if there is support for
    /// hardware acceleration.
    /// See https://w3c.github.io/webrtc-svc/#scalabilitymodes* for a specification
    /// of valid values for `scalability_mode`.
    fn query_codec_support(
        &self,
        format: &ArcasCxxSdpVideoFormat,
        scalability_mode: Vec<String>,
    ) -> ArcasVideoEncoderFactoryCodecSupport;

    /// Create video encoder returning a Box'ed trait object for the VideoEncoderImpl.
    fn create_video_encoder(&self, format: &ArcasCxxSdpVideoFormat) -> Box<VideoEncoderProxy>;

    /// Return an optional encoder selector (see `VideoEncoderSelectorImpl`) empty Vec is none.
    fn get_encoder_selector(&self) -> Option<VideoEncoderSelectorProxy>;
}
pub struct VideoEncoderFactoryProxy {
    api: Box<dyn VideoEncoderFactoryImpl>,
}

impl VideoEncoderFactoryProxy {
    pub fn new(api: Box<dyn VideoEncoderFactoryImpl>) -> Self {
        Self { api }
    }

    pub fn get_supported_formats(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.get_supported_formats()
    }
    pub fn get_implementations(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.get_implementations()
    }

    pub fn query_video_encoder(
        &self,
        format: &ArcasCxxSdpVideoFormat,
    ) -> ArcasVideoEncoderFactoryCodecInfo {
        self.api.query_video_encoder(format)
    }

    pub fn query_codec_support(
        &self,
        format: &ArcasCxxSdpVideoFormat,
        scalability_mode: Vec<String>,
    ) -> ArcasVideoEncoderFactoryCodecSupport {
        self.api.query_codec_support(format, scalability_mode)
    }

    pub fn create_video_encoder(&self, format: &ArcasCxxSdpVideoFormat) -> Box<VideoEncoderProxy> {
        self.api.create_video_encoder(format)
    }

    pub fn get_encoder_selector(&self) -> Vec<VideoEncoderSelectorProxy> {
        match self.api.get_encoder_selector() {
            Some(value) => {
                vec![value]
            }
            None => {
                vec![]
            }
        }
    }
}

pub trait VideoEncoderSelectorImpl {
    /// Informs the encoder selector about which encoder that is currently being
    /// used.
    fn on_current_encoder(&self, format: &ArcasCxxSdpVideoFormat);

    /// Called every time the available bitrate is updated. Should return a
    /// non-empty if an encoder switch should be performed.
    fn on_available_bitrate(
        &self,
        data_rate: &ArcasCxxDataRate,
    ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

    /// Called if the currently used encoder reports itself as broken. Should
    /// return a non-empty if an encoder switch should be performed.
    fn on_encoder_broken(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;
}

pub struct VideoEncoderSelectorProxy {
    api: Box<dyn VideoEncoderSelectorImpl>,
}

impl VideoEncoderSelectorProxy {
    pub fn new(api: Box<dyn VideoEncoderSelectorImpl>) -> Self {
        Self { api }
    }

    pub fn on_current_encoder(&self, format: &ArcasCxxSdpVideoFormat) {
        self.api.on_current_encoder(format);
    }

    pub fn on_available_bitrate(
        &self,
        data_rate: &ArcasCxxDataRate,
    ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.on_available_bitrate(data_rate)
    }

    pub fn on_encoder_broken(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.on_encoder_broken()
    }
}
