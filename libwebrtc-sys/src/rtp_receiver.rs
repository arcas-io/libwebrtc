#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/rtp_receiver.h");

        type ArcasRTPReceiver;
        type ArcasRTPVideoReceiver;
        type ArcasRTPAudioReceiver;

        fn gen_unique_rtp_receiver() -> UniquePtr<ArcasRTPReceiver>;
        fn gen_unique_rtp_audio_receiver() -> UniquePtr<ArcasRTPAudioReceiver>;
        fn gen_unique_rtp_video_receiver() -> UniquePtr<ArcasRTPVideoReceiver>;
    }
}
