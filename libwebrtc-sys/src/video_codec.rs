#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/video_codec.h");
        type ArcasVideoCodec;
        type ArcasSpatialLayer = crate::spatial_layer::ffi::ArcasSpatialLayer;
        type ArcasCxxVideoCodecMode = crate::shared_bridge::ffi::ArcasCxxVideoCodecMode;
        type ArcasCxxVideoCodecComplexity = crate::shared_bridge::ffi::ArcasCxxVideoCodecComplexity;
        type ArcasCxxInterLayerPredMode = crate::shared_bridge::ffi::ArcasCxxInterLayerPredMode;
        type ArcasCodecSpecificInfo = crate::codec_specific_info::ffi::ArcasCodecSpecificInfo;
        type ArcasVideoFrameEncodedImageData =
            crate::video_frame_buffer::ffi::ArcasVideoFrameEncodedImageData;
        type ArcasCxxVideoCodecType = crate::shared_bridge::ffi::ArcasCxxVideoCodecType;
        type ArcasCxxVideoCodec = crate::shared_bridge::ffi::ArcasCxxVideoCodec;

        fn gen_shared_video_codec() -> SharedPtr<ArcasVideoCodec>;

        unsafe fn create_arcas_video_codec_from_cxx(
            ptr: *const ArcasCxxVideoCodec,
        ) -> UniquePtr<ArcasVideoCodec>;

        // ArcasVideoCodec
        fn get_scalability_mode(self: &ArcasVideoCodec) -> String;
        fn get_width(self: &ArcasVideoCodec) -> i32;
        fn get_height(self: &ArcasVideoCodec) -> i32;
        fn get_max_bitrate(self: &ArcasVideoCodec) -> u32;
        fn get_min_bitrate(self: &ArcasVideoCodec) -> u32;
        fn get_start_bitrate(self: &ArcasVideoCodec) -> u32;
        fn get_max_framerate(self: &ArcasVideoCodec) -> u32;
        fn get_active(self: &ArcasVideoCodec) -> bool;
        fn get_qp_max(self: &ArcasVideoCodec) -> u32;
        fn get_number_of_simulcast_streams(self: &ArcasVideoCodec) -> u8;
        fn spatial_layers(self: &ArcasVideoCodec) -> UniquePtr<CxxVector<ArcasSpatialLayer>>;
        fn simulcast_streams(self: &ArcasVideoCodec) -> UniquePtr<CxxVector<ArcasSpatialLayer>>;
        fn set_scalability_mode(self: &ArcasVideoCodec, scalability_mode: String);
        fn set_codec_type(self: &ArcasVideoCodec, codec_type: ArcasCxxVideoCodecType);
        fn set_width(self: &ArcasVideoCodec, width: u16);
        fn set_height(self: &ArcasVideoCodec, height: u16);
        fn set_max_bitrate(self: &ArcasVideoCodec, max_bitrate: u32);
        fn set_min_bitrate(self: &ArcasVideoCodec, min_bitrate: u32);
        fn set_start_bitrate(self: &ArcasVideoCodec, start_bitrate: u32);
        fn set_max_framerate(self: &ArcasVideoCodec, max_frame_rate: u32);
        fn set_active(self: &ArcasVideoCodec, active: bool);
        fn set_qp_max(self: &ArcasVideoCodec, qp_max: u32);
        fn set_number_of_simulcast_streams(self: &ArcasVideoCodec, number_of_simulcast_streams: u8);
        fn set_simulcast_stream_at(self: &ArcasVideoCodec, index: u8, layer: &ArcasSpatialLayer);
        fn set_spatial_layer_at(self: &ArcasVideoCodec, index: u8, layer: &ArcasSpatialLayer);
        fn set_mode(self: &ArcasVideoCodec, mode: ArcasCxxVideoCodecMode);
        fn set_expect_encode_from_texture(self: &ArcasVideoCodec, expect_encode_from_texture: bool);
        fn set_buffer_pool_size(self: &ArcasVideoCodec, buffer_pool_size: i32);
        fn set_timing_frame_trigger_thresholds(
            self: &ArcasVideoCodec,
            delay_ms: i64,
            outlier_ratio_percent: u16,
        );
        fn set_legacy_conference_mode(self: &ArcasVideoCodec, legacy_conference_mode: bool);
        fn vp8_set_codec_complexity(
            self: &ArcasVideoCodec,
            complexity: ArcasCxxVideoCodecComplexity,
        );
        fn vp8_set_number_of_temporal_layers(self: &ArcasVideoCodec, number_of_temporal_layers: u8);
        fn vp8_set_denoising_on(self: &ArcasVideoCodec, denoising_on: bool);
        fn vp8_set_automatic_resize_on(self: &ArcasVideoCodec, automatic_resize: bool);
        fn vp8_set_frame_dropping_on(self: &ArcasVideoCodec, frame_dropping: bool);
        fn vp8_set_key_frame_interval(self: &ArcasVideoCodec, key_frame_interval: i32);
        fn vp9_set_codec_complexity(
            self: &ArcasVideoCodec,
            complexity: ArcasCxxVideoCodecComplexity,
        );
        fn vp9_set_number_of_temporal_layers(self: &ArcasVideoCodec, number_of_temporal_layers: u8);
        fn vp9_set_denoising_on(self: &ArcasVideoCodec, denoising_on: bool);
        fn vp9_set_frame_dropping_on(self: &ArcasVideoCodec, frame_dropping: bool);
        fn vp9_set_key_frame_interval(self: &ArcasVideoCodec, key_frame_interval: i32);
        fn vp9_set_adaptive_qp_on(self: &ArcasVideoCodec, adaptive_qp: bool);
        fn vp9_set_automatic_resize_on(self: &ArcasVideoCodec, automatic_resize: bool);
        fn vp9_set_number_of_spatial_layers(self: &ArcasVideoCodec, number_of_spatial_layers: u8);
        fn vp9_set_flexible_mode(self: &ArcasVideoCodec, flexible_mode: bool);
        fn vp9_set_inter_layer_pred(
            self: &ArcasVideoCodec,
            inter_layer_pred: ArcasCxxInterLayerPredMode,
        );
        fn h264_set_frame_dropping_on(self: &ArcasVideoCodec, frame_dropping: bool);
        fn h264_set_key_frame_interval(self: &ArcasVideoCodec, key_frame_interval: i32);
        fn h264_set_number_of_temporal_layers(
            self: &ArcasVideoCodec,
            number_of_temporal_layers: u8,
        );
        fn cxx_clone(self: &ArcasVideoCodec) -> UniquePtr<ArcasVideoCodec>;

    }
}
