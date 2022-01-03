use crate::video_encoding::VideoEncoderFactoryProxy;
use crate::audio_encoding::AudioEncoderFactoryProxy;
use crate::video_decoding::VideoDecoderFactoryProxy;

#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("include/peerconnection_factory_config.h");

        type ArcasPeerConnectionFactoryConfig;
        type ArcasVideoEncoderFactory = crate::video_encoding::ffi::ArcasVideoEncoderFactory;
        type ArcasVideoDecoderFactory = crate::video_decoding::ffi::ArcasVideoDecoderFactory;

        fn create_arcas_peerconnection_factory_config(
            video_encoder_factory: UniquePtr<ArcasVideoEncoderFactory>,
            video_decoder_factory: UniquePtr<ArcasVideoDecoderFactory>,
        ) -> UniquePtr<ArcasPeerConnectionFactoryConfig>;
    }
}
