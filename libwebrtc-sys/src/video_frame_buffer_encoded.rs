#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/video_frame_buffer_encoded.h");
        type ArcasVideoFrameEncodedImageData;
        type ArcasVideoFrameRawImageData;
        type ArcasCxxEncodedImage = crate::shared_bridge::ffi::ArcasCxxEncodedImage;
        type ArcasCodecSpecificInfo = crate::codec_specific_info::ffi::ArcasCodecSpecificInfo;
        type ArcasCxxCodecSpecificInfo = crate::codec_specific_info::ffi::ArcasCxxCodecSpecificInfo;

        fn create_arcas_video_frame_buffer_from_encoded_image(
            encoded_image: &ArcasCxxEncodedImage,
            codec_specific_info: &ArcasCodecSpecificInfo,
        ) -> UniquePtr<ArcasVideoFrameEncodedImageData>;

        // NOTE: This *does* copy the data passed in.
        unsafe fn create_arcas_video_frame_buffer_from_I420(
            width: i32,
            height: i32,
            data: *const u8,
        ) -> UniquePtr<ArcasVideoFrameRawImageData>;

        // ArcasVideoFrameRawImageData
        fn width(self: &ArcasVideoFrameRawImageData) -> i32;
        fn height(self: &ArcasVideoFrameRawImageData) -> i32;

        // ArcasVideoFrameEncodedImageData
        fn width(self: &ArcasVideoFrameEncodedImageData) -> i32;
        fn height(self: &ArcasVideoFrameEncodedImageData) -> i32;
        fn size(self: &ArcasVideoFrameEncodedImageData) -> u32;
        fn data(self: &ArcasVideoFrameEncodedImageData) -> *const u8;
        fn encoded_image_ref(self: &ArcasVideoFrameEncodedImageData) -> &ArcasCxxEncodedImage;
        fn codec_specific_info_ref(
            self: &ArcasVideoFrameEncodedImageData,
        ) -> &ArcasCxxCodecSpecificInfo;
        fn arcas_codec_specific_info(
            self: &ArcasVideoFrameEncodedImageData,
        ) -> UniquePtr<ArcasCodecSpecificInfo>;

        // NOTE: This clone does not copy the underlying memory and uses ref counting to keep it alive.
        fn clone(
            self: &ArcasVideoFrameEncodedImageData,
        ) -> UniquePtr<ArcasVideoFrameEncodedImageData>;
    }
}
