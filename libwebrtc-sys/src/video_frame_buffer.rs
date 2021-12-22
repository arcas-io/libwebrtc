#[cxx::bridge]
pub mod ffi {

    unsafe extern "C++" {
        include!("include/video_frame_buffer_empty.h");
        include!("include/video_frame_buffer_encoded.h");

        type ArcasVideoFrameBufferEmpty;
        type ArcasVideoFrameEncodedImageData =
            crate::video_frame_buffer_encoded::ffi::ArcasVideoFrameEncodedImageData;
        type ArcasCxxEncodedImage = crate::shared_bridge::ffi::ArcasCxxEncodedImage;
        type ArcasCxxCodecSpecificInfo = crate::shared_bridge::ffi::ArcasCxxCodecSpecificInfo;
        type ArcasCodecSpecificInfo = crate::codec_specific_info::ffi::ArcasCodecSpecificInfo;

    }
}
