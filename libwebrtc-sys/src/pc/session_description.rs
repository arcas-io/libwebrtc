#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("pc/session_description.h");
        #[namespace = "cricket"]
        type SessionDescription;

        #[cxx_name = "Clone"]
        fn cxx_clone(self: &SessionDescription) -> UniquePtr<SessionDescription>;
    }
}
