use cxx::UniquePtr;

use crate::{
    codec_specific_info::ffi::ArcasCodecSpecificInfo,
    shared_bridge::ffi::ArcasVideoEncoderDropReason, video_encoding::ffi::ArcasCxxEncodedImage,
};

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/video_encoding_wrapper.h");
        include!("libwebrtc-sys/src/video_encoding.rs.h");

        type ArcasVideoEncoderFactoryWrapper;
        type ArcasSDPVideoFormatWrapper =
            crate::video_encoding_wrapper::ffi::ArcasSDPVideoFormatWrapper;
        type ArcasCxxSdpVideoFormat = crate::shared_bridge::ffi::ArcasCxxSdpVideoFormat;
        type ArcasVideoEncoderWrapper =
            crate::video_encoding_wrapper::ffi::ArcasVideoEncoderWrapper;
        type ArcasCxxEncodedImage = crate::video_encoding::ffi::ArcasCxxEncodedImage;
        type ArcasCodecSpecificInfo = crate::codec_specific_info::ffi::ArcasCodecSpecificInfo;
        type ArcasVideoEncoderDropReason = crate::shared_bridge::ffi::ArcasVideoEncoderDropReason;

        type ArcasReactiveVideoEncoderWrapper;

        /// Intended to be used by rust calling back into C++ factories.
        fn create_encoder_reactive(
            self: &ArcasVideoEncoderFactoryWrapper,
            format: &ArcasCxxSdpVideoFormat,
        ) -> UniquePtr<ArcasReactiveVideoEncoderWrapper>;

        fn create_arcas_video_encoder_factory_from_builtin(
        ) -> UniquePtr<ArcasVideoEncoderFactoryWrapper>;

        fn get_supported_formats(
            self: &ArcasVideoEncoderFactoryWrapper,
        ) -> UniquePtr<CxxVector<ArcasSDPVideoFormatWrapper>>;

        fn cxx_get_supported_formats(
            self: &ArcasVideoEncoderFactoryWrapper,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

        fn create_encoder(
            self: &ArcasVideoEncoderFactoryWrapper,
            format: &ArcasSDPVideoFormatWrapper,
            callback: Box<EncodedImageCallbackHandler>,
        ) -> UniquePtr<ArcasVideoEncoderWrapper>;
    }

    extern "Rust" {
        #[rust_name = "EncodedImageCallbackHandler"]
        type ArcasRustEncodedImageCallbackHandler;

        // EncodedImageCallbackHandler
        fn trigger_encoded_image(
            self: &EncodedImageCallbackHandler,
            image: UniquePtr<ArcasCxxEncodedImage>,
            codec_info: UniquePtr<ArcasCodecSpecificInfo>,
        );

        fn trigger_dropped(self: &EncodedImageCallbackHandler, reason: ArcasVideoEncoderDropReason);
    }
}

type EncodedImageCallback =
    dyn Fn(UniquePtr<ArcasCxxEncodedImage>, UniquePtr<ArcasCodecSpecificInfo>);

pub struct EncodedImageCallbackHandler {
    on_dropped: Box<dyn Fn(ArcasVideoEncoderDropReason)>,
    on_encoded_image: Box<EncodedImageCallback>,
}

impl EncodedImageCallbackHandler {
    pub fn new(
        on_dropped: Box<dyn Fn(ArcasVideoEncoderDropReason)>,
        on_encoded_image: Box<EncodedImageCallback>,
    ) -> Self {
        Self {
            on_dropped,
            on_encoded_image,
        }
    }

    pub fn trigger_encoded_image(
        &self,
        image: UniquePtr<ArcasCxxEncodedImage>,
        codec_info: UniquePtr<ArcasCodecSpecificInfo>,
    ) {
        (self.on_encoded_image)(image, codec_info);
    }

    pub fn trigger_dropped(&self, reason: ArcasVideoEncoderDropReason) {
        (self.on_dropped)(reason);
    }
}
