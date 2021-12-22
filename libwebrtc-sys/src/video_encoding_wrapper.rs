#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/video_encoding_wrapper.h");
        include!("libwebrtc-sys/src/shared_bridge.rs.h");
        include!("include/reactive_video_encoder_wrapper.h");
        include!("libwebrtc-sys/src/video_encoding.rs.h");
        type ArcasVideoEncoderWrapper;
        type ArcasVideoCodec = crate::video_codec::ffi::ArcasVideoCodec;
        type ArcasCxxVideoCodec = crate::shared_bridge::ffi::ArcasCxxVideoCodec;
        type ArcasVideoEncoderSettings = crate::video_encoding::ffi::ArcasVideoEncoderSettings;
        type ArcasCxxVideoFrame = crate::video_frame::ffi::ArcasCxxVideoFrame;
        type ArcasVideoFrameTypesCollection =
            crate::video_frame::ffi::ArcasVideoFrameTypesCollection;
        type ArcasCxxVideoFrameType = crate::video_frame::ffi::ArcasCxxVideoFrameType;
        type ArcasVideoEncoderRateControlParameters =
            crate::video_encoding::ffi::ArcasVideoEncoderRateControlParameters;
        type ArcasCxxVideoEncoderRateControlParameters =
            crate::shared_bridge::ffi::ArcasCxxVideoEncoderRateControlParameters;
        type ArcasVideoEncoderLossNotification =
            crate::video_encoding::ffi::ArcasVideoEncoderLossNotification;
        type ArcasVideoEncoderInfo = crate::video_encoding::ffi::ArcasVideoEncoderInfo;
        type ArcasSDPVideoFormatWrapper;
        type ArcasCxxSdpVideoFormat = crate::shared_bridge::ffi::ArcasCxxSdpVideoFormat;
        type ArcasRustDict = crate::shared_bridge::ffi::ArcasRustDict;

        fn gen_unique_sdp_video_format_wrapper() -> UniquePtr<ArcasSDPVideoFormatWrapper>;
        fn gen_unique_vector_sdp_video_format_wrapper(
        ) -> UniquePtr<CxxVector<ArcasSDPVideoFormatWrapper>>;
        fn gen_unique_video_encoder_wrapper() -> UniquePtr<ArcasVideoEncoderWrapper>;

        // ArcasVideoEncoderWrapper
        fn init_encode(
            self: &ArcasVideoEncoderWrapper,
            codec: &ArcasVideoCodec,
            settings: &ArcasVideoEncoderSettings,
        ) -> i32;

        unsafe fn cxx_init_encode(
            self: &ArcasVideoEncoderWrapper,
            codec: *const ArcasCxxVideoCodec,
            number_of_cores: i32,
            max_payload_size: usize,
        ) -> i32;

        fn release(self: &ArcasVideoEncoderWrapper) -> i32;

        fn encode(
            self: &ArcasVideoEncoderWrapper,
            frame: &ArcasCxxVideoFrame,
            frame_types: &ArcasVideoFrameTypesCollection,
        ) -> i32;

        unsafe fn cxx_encode(
            self: &ArcasVideoEncoderWrapper,
            frame: &ArcasCxxVideoFrame,
            frame_types: *const CxxVector<ArcasCxxVideoFrameType>,
        ) -> i32;

        fn set_rates(
            self: &ArcasVideoEncoderWrapper,
            rates: &ArcasVideoEncoderRateControlParameters,
        );

        fn cxx_set_rates(
            self: &ArcasVideoEncoderWrapper,
            rates: &ArcasCxxVideoEncoderRateControlParameters,
        );

        fn on_rtt_update(self: &ArcasVideoEncoderWrapper, rtt: i64);
        fn on_loss_notification(
            self: &ArcasVideoEncoderWrapper,
            loss: ArcasVideoEncoderLossNotification,
        );
        fn on_packet_loss_rate_update(self: &ArcasVideoEncoderWrapper, packet_loss_rate: f32);
        fn get_encoder_info(self: &ArcasVideoEncoderWrapper) -> ArcasVideoEncoderInfo;

        // ArcasSDPVideoFormatWrapper
        fn get_name(self: &ArcasSDPVideoFormatWrapper) -> String;
        fn get_parameters(self: &ArcasSDPVideoFormatWrapper) -> Vec<ArcasRustDict>;
        fn to_string(self: &ArcasSDPVideoFormatWrapper) -> String;
        fn clone(self: &ArcasSDPVideoFormatWrapper) -> UniquePtr<ArcasSDPVideoFormatWrapper>;
        fn cxx_format_list(
            self: &ArcasSDPVideoFormatWrapper,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;
    }
}
