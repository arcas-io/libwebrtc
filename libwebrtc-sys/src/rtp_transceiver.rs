#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/rtp_transceiver.h");

        type ArcasRTPTransceiver;
        type ArcasRTPVideoTransceiver;
        type ArcasRTPAudioTransceiver;
        type ArcasRTPTransceiverDirection = crate::shared_bridge::ffi::ArcasRTPTransceiverDirection;
        type ArcasMediaType = crate::shared_bridge::ffi::ArcasMediaType;
        type ArcasRTPVideoSender = crate::rtp_sender::ffi::ArcasRTPVideoSender;
        type ArcasRTPVideoReceiver = crate::rtp_receiver::ffi::ArcasRTPVideoReceiver;
        type ArcasRTPAudioSender = crate::rtp_sender::ffi::ArcasRTPAudioSender;
        type ArcasRTPAudioReceiver = crate::rtp_receiver::ffi::ArcasRTPAudioReceiver;
        type ArcasRTCError = crate::error::ffi::ArcasRTCError;
        type ArcasRTPCodecCapability = crate::rtp_parameters::ffi::ArcasRTPCodecCapability;
        type ArcasRTPHeaderExtensionCapability =
            crate::rtp_parameters::ffi::ArcasRTPHeaderExtensionCapability;

        fn gen_unique_vector_rtp_transceivers() -> UniquePtr<CxxVector<ArcasRTPTransceiver>>;

        // ArcasRTPTransceiver
        fn video_transceiver_from_base(
            base: &ArcasRTPTransceiver,
        ) -> UniquePtr<ArcasRTPVideoTransceiver>;
        fn audio_transceiver_from_base(
            base: &ArcasRTPTransceiver,
        ) -> UniquePtr<ArcasRTPAudioTransceiver>;
        fn direction(self: &ArcasRTPTransceiver) -> ArcasRTPTransceiverDirection;
        fn media_type(self: &ArcasRTPTransceiver) -> ArcasMediaType;

        // ArcasRTPVideoTransceiver
        fn mid(self: &ArcasRTPVideoTransceiver) -> String;
        fn media_type(self: &ArcasRTPVideoTransceiver) -> ArcasMediaType;
        fn get_sender(self: &ArcasRTPVideoTransceiver) -> UniquePtr<ArcasRTPVideoSender>;
        fn get_receiver(self: &ArcasRTPVideoTransceiver) -> UniquePtr<ArcasRTPVideoReceiver>;
        fn stopped(self: &ArcasRTPVideoTransceiver) -> bool;
        fn stopping(self: &ArcasRTPVideoTransceiver) -> bool;
        fn direction(self: &ArcasRTPVideoTransceiver) -> ArcasRTPTransceiverDirection;
        fn stop(self: &ArcasRTPVideoTransceiver) -> UniquePtr<ArcasRTCError>;
        fn clone(self: &ArcasRTPVideoTransceiver) -> UniquePtr<ArcasRTPVideoTransceiver>;

        fn header_extensions_to_offer(
            self: &ArcasRTPVideoTransceiver,
        ) -> UniquePtr<CxxVector<ArcasRTPHeaderExtensionCapability>>;

        fn header_extensions_to_negotiated(
            self: &ArcasRTPVideoTransceiver,
        ) -> UniquePtr<CxxVector<ArcasRTPHeaderExtensionCapability>>;

        fn codec_preferences(
            self: &ArcasRTPVideoTransceiver,
        ) -> UniquePtr<CxxVector<ArcasRTPCodecCapability>>;

        fn set_direction(
            self: &ArcasRTPVideoTransceiver,
            direction: ArcasRTPTransceiverDirection,
        ) -> UniquePtr<ArcasRTCError>;

        fn set_codec_preferences(
            self: &ArcasRTPVideoTransceiver,
            codec_preferences: UniquePtr<CxxVector<ArcasRTPCodecCapability>>,
        ) -> UniquePtr<ArcasRTCError>;

        fn set_offerred_rtp_header_extensions(
            self: &ArcasRTPVideoTransceiver,
            extensions: UniquePtr<CxxVector<ArcasRTPHeaderExtensionCapability>>,
        ) -> UniquePtr<ArcasRTCError>;

        // ArcasRTPAudioTransceiver
        fn mid(self: &ArcasRTPAudioTransceiver) -> String;
        fn media_type(self: &ArcasRTPAudioTransceiver) -> ArcasMediaType;
        fn get_sender(self: &ArcasRTPAudioTransceiver) -> UniquePtr<ArcasRTPAudioSender>;
        fn get_receiver(self: &ArcasRTPAudioTransceiver) -> UniquePtr<ArcasRTPAudioReceiver>;
        fn stopped(self: &ArcasRTPAudioTransceiver) -> bool;
        fn stopping(self: &ArcasRTPAudioTransceiver) -> bool;
        fn direction(self: &ArcasRTPAudioTransceiver) -> ArcasRTPTransceiverDirection;
        fn stop(self: &ArcasRTPAudioTransceiver) -> UniquePtr<ArcasRTCError>;
        fn clone(self: &ArcasRTPAudioTransceiver) -> UniquePtr<ArcasRTPAudioTransceiver>;

        fn header_extensions_to_offer(
            self: &ArcasRTPAudioTransceiver,
        ) -> UniquePtr<CxxVector<ArcasRTPHeaderExtensionCapability>>;

        fn header_extensions_to_negotiated(
            self: &ArcasRTPAudioTransceiver,
        ) -> UniquePtr<CxxVector<ArcasRTPHeaderExtensionCapability>>;

        fn codec_preferences(
            self: &ArcasRTPAudioTransceiver,
        ) -> UniquePtr<CxxVector<ArcasRTPCodecCapability>>;

        fn set_codec_preferences(
            self: &ArcasRTPAudioTransceiver,
            codec_preferences: UniquePtr<CxxVector<ArcasRTPCodecCapability>>,
        ) -> UniquePtr<ArcasRTCError>;

        fn set_direction(
            self: &ArcasRTPAudioTransceiver,
            direction: ArcasRTPTransceiverDirection,
        ) -> UniquePtr<ArcasRTCError>;

        fn set_offerred_rtp_header_extensions(
            self: &ArcasRTPAudioTransceiver,
            extensions: UniquePtr<CxxVector<ArcasRTPHeaderExtensionCapability>>,
        ) -> UniquePtr<ArcasRTCError>;
    }
}
