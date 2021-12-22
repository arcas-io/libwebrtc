#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/sdp_video_format.h");
        include!("libwebrtc-sys/src/shared_bridge.rs.h");
        type ArcasCxxSdpVideoFormat = crate::shared_bridge::ffi::ArcasCxxSdpVideoFormat;
        type ArcasRustDict = crate::shared_bridge::ffi::ArcasRustDict;
        type ArcasSdpVideoFormatInit = crate::shared_bridge::ffi::ArcasSdpVideoFormatInit;
        type ArcasSdpVideoFormatVecInit = crate::shared_bridge::ffi::ArcasSdpVideoFormatVecInit;

        // ArcasCxxSdpVideoFormat
        #[cxx_name = "IsSameCodec"]
        unsafe fn is_same_codec(
            self: &ArcasCxxSdpVideoFormat,
            other: &ArcasCxxSdpVideoFormat,
        ) -> bool;

        #[cxx_name = "sdp_video_format_get_parameters"]
        fn video_format_get_parameters(format: &ArcasCxxSdpVideoFormat) -> Vec<ArcasRustDict>;

        #[cxx_name = "sdp_video_format_get_name"]
        fn sdp_video_format_get_name(format: &ArcasCxxSdpVideoFormat) -> &'static CxxString;

        #[cxx_name = "sdp_video_format_to_string"]
        fn sdp_video_format_to_string(format: &ArcasCxxSdpVideoFormat) -> String;

        // Helper for use with the VideoFactory methods but generically useful too
        //
        // Where optional types are used this is also how we pass "none" (empty vec)
        // These types are relevant:
        //
        //  - absl::optional<SdpVideoFormat>
        //  - std::vector<SdpVideoFormat>
        //
        fn create_sdp_video_format_list(
            format_list: ArcasSdpVideoFormatVecInit,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

        fn create_sdp_video_format(
            init: ArcasSdpVideoFormatInit,
        ) -> UniquePtr<ArcasCxxSdpVideoFormat>;
    }
}
