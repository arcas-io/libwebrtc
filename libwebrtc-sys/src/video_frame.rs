#[cxx::bridge]
pub mod ffi {
    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasCxxVideoFrameType {
        kEmptyFrame = 0,
        // Wire format for MultiplexEncodedImagePacker seems to depend on numerical
        // values of these constants.
        kVideoFrameKey = 3,
        kVideoFrameDelta = 4,
    }

    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/color_space.h");
        include!("include/video_frame.h");
        type ArcasVideoFrameFactory;
        type ArcasColorSpace;
        type ArcasCxxVideoFrameType;
        type ArcasVideoFrameTypesCollection;
        type ArcasCxxVideoFrame;
        type ArcasVideoFrameEncodedImageData =
            crate::video_frame_buffer_encoded::ffi::ArcasVideoFrameEncodedImageData;
        type ArcasVideoFrameRawImageData =
            crate::video_frame_buffer_encoded::ffi::ArcasVideoFrameRawImageData;

        fn create_arcas_video_frame_factory() -> UniquePtr<ArcasVideoFrameFactory>;
        fn create_arcas_color_space() -> UniquePtr<ArcasColorSpace>;
        fn gen_unique_cxx_video_frame() -> UniquePtr<ArcasCxxVideoFrame>;

        fn create_arcas_video_frame_types_collection(
            rust_array: Vec<ArcasCxxVideoFrameType>,
        ) -> SharedPtr<ArcasVideoFrameTypesCollection>;

        fn extract_arcas_video_frame_to_raw_frame_buffer(
            video_frame: &ArcasCxxVideoFrame,
        ) -> UniquePtr<ArcasVideoFrameEncodedImageData>;

        // ArcasVideoFrameFactory
        fn set_encoded_video_frame_buffer(
            self: &ArcasVideoFrameFactory,
            buffer: &ArcasVideoFrameEncodedImageData,
        );
        fn set_raw_video_frame_buffer(
            self: &ArcasVideoFrameFactory,
            buffer: &ArcasVideoFrameRawImageData,
        );
        fn set_empty_video_frame_buffer(self: &ArcasVideoFrameFactory);
        fn set_timestamp_ms(self: &ArcasVideoFrameFactory, timestamp_ms: u64);
        fn set_timestamp_rtp(self: &ArcasVideoFrameFactory, timestamp_ms: u32);
        fn set_ntp_time_ms(self: &ArcasVideoFrameFactory, timestamp_ms: i64);
        fn set_color_space(self: &ArcasVideoFrameFactory, color_space: &ArcasColorSpace);
        fn build(self: &ArcasVideoFrameFactory) -> UniquePtr<ArcasCxxVideoFrame>;
    }
}
