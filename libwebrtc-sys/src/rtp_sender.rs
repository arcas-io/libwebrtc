#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("include/rtp_sender.h");
        type ArcasRTPSender;
        type ArcasRTPVideoSender;
        type ArcasRTPAudioSender;
        type ArcasVideoTrack = crate::video_track::ffi::ArcasVideoTrack;

        fn gen_unique_rtp_audio_sender() -> UniquePtr<ArcasRTPAudioSender>;
        fn gen_unique_rtp_video_sender() -> UniquePtr<ArcasRTPVideoSender>;

        // ArcasRTPVideoSender
        fn set_track(self: &ArcasRTPVideoSender, track: &ArcasVideoTrack) -> bool;
    }
}
