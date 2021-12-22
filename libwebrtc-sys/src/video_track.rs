#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/video_track.h");
        type ArcasVideoTrack;

        fn gen_unique_video_track() -> UniquePtr<ArcasVideoTrack>;
    }
}
