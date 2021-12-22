#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("include/codec_specific_info.h");
        type ArcasCxxVideoCodecType = crate::shared_bridge::ffi::ArcasCxxVideoCodecType;
        type ArcasCxxCodecSpecificInfo = crate::shared_bridge::ffi::ArcasCxxCodecSpecificInfo;
        type ArcasCodecSpecificInfo;

        fn create_arcas_codec_specific_info() -> UniquePtr<ArcasCodecSpecificInfo>;

        // ArcasCodecSpecificInfo
        fn set_codec_type(self: &ArcasCodecSpecificInfo, codec_type: ArcasCxxVideoCodecType);
        fn set_end_of_picture(self: &ArcasCodecSpecificInfo, set_end_of_picture: bool);
        fn get_codec_type(self: &ArcasCodecSpecificInfo) -> ArcasCxxVideoCodecType;
        #[cxx_name = "as_ref"]
        fn as_cxx_ref(self: &ArcasCodecSpecificInfo) -> &ArcasCxxCodecSpecificInfo;
    }
}
