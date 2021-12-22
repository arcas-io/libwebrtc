#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/error.h");
        type ArcasRTCError;
        type ArcasRTCErrorType = crate::shared_bridge::ffi::ArcasRTCErrorType;
        #[namespace = "webrtc"]
        type RTCError;

        fn gen_unique_ptr_error() -> UniquePtr<ArcasRTCError>;

        // ArcasRTCError
        fn ok(self: &ArcasRTCError) -> bool;
        fn kind(self: &ArcasRTCError) -> ArcasRTCErrorType;
        fn message(self: &ArcasRTCError) -> String;
    }
}
