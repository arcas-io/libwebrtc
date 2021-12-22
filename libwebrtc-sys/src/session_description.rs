#[cxx::bridge]
pub mod ffi {
    struct ArcasSessionDescriptionError {
        line: String,
        description: String,
    }

    struct ArcasCreateSessionDescriptionResult {
        ok: bool,
        session: UniquePtr<ArcasSessionDescription>,
        error: ArcasSessionDescriptionError,
    }

    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/session_description.h");
        type ArcasSessionDescription;
        type ArcasSDPType = crate::shared_bridge::ffi::ArcasSDPType;

        fn create_arcas_session_description(
            sdp_type: ArcasSDPType,
            sdp: String,
        ) -> ArcasCreateSessionDescriptionResult;

        // ArcasSessionDescription
        #[cxx_name = "to_string"]
        fn cxx_to_string(self: &ArcasSessionDescription) -> String;
        fn get_type(self: &ArcasSessionDescription) -> ArcasSDPType;
        #[cxx_name = "clone"]
        fn clone_cxx(self: &ArcasSessionDescription) -> UniquePtr<ArcasSessionDescription>;
    }
}
