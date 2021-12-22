#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("include/rtp_parameters.h");
        type ArcasRTPCodecCapability;
        type ArcasRTPHeaderExtensionCapability;

        fn gen_unique_vector_rtp_header_extension_capabilities(
        ) -> UniquePtr<CxxVector<ArcasRTPHeaderExtensionCapability>>;
        fn gen_unique_vector_rtp_codec_capabilities(
        ) -> UniquePtr<CxxVector<ArcasRTPCodecCapability>>;
    }
}
