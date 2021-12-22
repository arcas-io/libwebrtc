#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/video_track_source.h");
        type ArcasVideoTrackSource;
        type ArcasCxxVideoFrame = crate::video_frame::ffi::ArcasCxxVideoFrame;

        fn create_arcas_video_track_source() -> UniquePtr<ArcasVideoTrackSource>;
        // ArcasVideoTrackSource
        fn push_frame(self: &ArcasVideoTrackSource, video_frame: &ArcasCxxVideoFrame);
        fn cxx_clone(self: &ArcasVideoTrackSource) -> UniquePtr<ArcasVideoTrackSource>;
    }

    extern "Rust" {}
}
