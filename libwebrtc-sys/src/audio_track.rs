#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("libwebrtc-sys/include/audio_track.h");
        type ArcasAudioTrack;

        fn gen_unique_audio_track() -> UniquePtr<ArcasAudioTrack>;
    }
}
