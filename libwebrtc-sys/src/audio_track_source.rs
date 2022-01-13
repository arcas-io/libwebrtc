use cxx::SharedPtr;

#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("include/audio_track_source.h");

        type ArcasAudioTrackSource;

        fn create_audio_track_source() -> SharedPtr<ArcasAudioTrackSource>;

        fn push_zeroed_data(
            self: &ArcasAudioTrackSource,
            sample_rate: i32,
            number_of_channels: usize,
        );

        fn push_raw_s16be(
            self: &ArcasAudioTrackSource,
            audio_data: Vec<u8>,
            sample_rate: i32,
            number_of_channels: usize,
            number_of_frames: usize,
        );

        fn gen_shared_audio_track_source() -> SharedPtr<ArcasAudioTrackSource>;
    }
}
