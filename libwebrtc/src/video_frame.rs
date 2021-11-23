use bytes::Bytes;
use cxx::UniquePtr;
use libwebrtc_sys::ffi::{
    create_arcas_color_space, create_arcas_video_frame_buffer_from_I420,
    create_arcas_video_frame_buffer_from_encoded_image, create_arcas_video_frame_factory,
    ArcasCodecSpecificInfo, ArcasColorSpace, ArcasCxxEncodedImage, ArcasCxxVideoFrame,
    ArcasVideoFrameEncodedImageData, ArcasVideoFrameRawImageData,
};

use crate::error::{self, Result, WebRTCError};

pub trait AsCxxVideoFrame {
    fn as_cxx_video_frame_ref(&self) -> Result<&ArcasCxxVideoFrame>;
}

// We use empty video frame for a number of tricks including triggering the encoder pipeline to start without
// actually passing through data.
pub struct EmptyVideoFrame {
    // Hold the underlying C++ object alive as long as this VideoFrame is alive.
    #[allow(dead_code)]
    color_space: UniquePtr<ArcasColorSpace>,
    // Bytes reference to ensure as long as this video frame is alive it's pointer is valid.
    video_frame: UniquePtr<ArcasCxxVideoFrame>,
}

impl EmptyVideoFrame {
    pub fn create(timestamp_ms: u64) -> Result<Self> {
        let frame_factory = create_arcas_video_frame_factory();
        let color_space = create_arcas_color_space();
        let color_space_ref = color_space
            .as_ref()
            .ok_or_else(|| WebRTCError::CXXUnwrapError("failed to get color space ref".into()))?;

        frame_factory.set_timestamp_ms(timestamp_ms);
        frame_factory.set_empty_video_frame_buffer();
        frame_factory.set_color_space(color_space_ref);
        frame_factory.set_ntp_time_ms(timestamp_ms as i64);
        frame_factory.set_timestamp_rtp(timestamp_ms as u32);
        let video_frame = frame_factory.build();

        Ok(Self {
            video_frame,
            color_space,
        })
    }
}

impl AsCxxVideoFrame for EmptyVideoFrame {
    fn as_cxx_video_frame_ref(&self) -> Result<&ArcasCxxVideoFrame> {
        self.video_frame.as_ref().ok_or_else(|| {
            error::WebRTCError::CXXUnwrapError("failed to unwrap video frame".into())
        })
    }
}

pub struct RawVideoFrame {
    // Hold the underlying C++ object alive as long as this VideoFrame is alive.
    #[allow(dead_code)]
    video_frame_buffer: UniquePtr<ArcasVideoFrameRawImageData>,
    // Hold the underlying C++ object alive as long as this VideoFrame is alive.
    #[allow(dead_code)]
    color_space: UniquePtr<ArcasColorSpace>,
    // Bytes reference to ensure as long as this video frame is alive it's pointer is valid.
    #[allow(dead_code)]
    bytes: Bytes,
    video_frame: UniquePtr<ArcasCxxVideoFrame>,
}

impl RawVideoFrame {
    pub fn create(width: i32, height: i32, timestamp_ms: u64, bytes: Bytes) -> Result<Self> {
        let frame_factory = create_arcas_video_frame_factory();
        let color_space = create_arcas_color_space();
        let color_space_ref = color_space
            .as_ref()
            .ok_or_else(|| WebRTCError::CXXUnwrapError("failed to get color space ref".into()))?;

        let video_frame_buffer =
            unsafe { create_arcas_video_frame_buffer_from_I420(width, height, bytes.as_ptr()) };

        let video_frame_buffer_ref = video_frame_buffer.as_ref().ok_or_else(|| {
            WebRTCError::CXXUnwrapError("failed to unwrap video frame buffer".into())
        })?;

        frame_factory.set_timestamp_ms(timestamp_ms);
        frame_factory.set_raw_video_frame_buffer(video_frame_buffer_ref);
        frame_factory.set_color_space(color_space_ref);
        frame_factory.set_ntp_time_ms(timestamp_ms as i64);
        frame_factory.set_timestamp_rtp(timestamp_ms as u32);
        let video_frame = frame_factory.build();

        Ok(Self {
            video_frame_buffer,
            bytes,
            video_frame,
            color_space,
        })
    }
}

impl AsCxxVideoFrame for RawVideoFrame {
    fn as_cxx_video_frame_ref(&self) -> Result<&ArcasCxxVideoFrame> {
        self.video_frame.as_ref().ok_or_else(|| {
            error::WebRTCError::CXXUnwrapError("failed to unwrap video frame".into())
        })
    }
}

pub struct EncodedVideoFrame {
    // Hold the underlying C++ object alive as long as this VideoFrame is alive.
    #[allow(dead_code)]
    video_frame_buffer: UniquePtr<ArcasVideoFrameEncodedImageData>,
    video_frame: UniquePtr<ArcasCxxVideoFrame>,
}

impl EncodedVideoFrame {
    pub fn create(
        encoded_image: UniquePtr<ArcasCxxEncodedImage>,
        codec_specific_info: UniquePtr<ArcasCodecSpecificInfo>,
        timestamp_ms: u64,
    ) -> Result<Self> {
        let frame_factory = create_arcas_video_frame_factory();

        let encoded_image_ref = encoded_image.as_ref().ok_or_else(|| {
            error::WebRTCError::CXXUnwrapError("failed to unwrap encoded image".into())
        })?;

        let codec_specific_info_ref = codec_specific_info.as_ref().ok_or_else(|| {
            error::WebRTCError::CXXUnwrapError("failed to unwrap codec specific info".into())
        })?;

        let video_frame_buffer = create_arcas_video_frame_buffer_from_encoded_image(
            encoded_image_ref,
            codec_specific_info_ref,
        );

        let video_frame_buffer_ref = video_frame_buffer.as_ref().ok_or_else(|| {
            WebRTCError::CXXUnwrapError("failed to unwrap video frame buffer".into())
        })?;

        frame_factory.set_timestamp_ms(timestamp_ms);
        frame_factory.set_encoded_video_frame_buffer(video_frame_buffer_ref);
        let video_frame = frame_factory.build();

        Ok(Self {
            video_frame,
            video_frame_buffer,
        })
    }
}

impl AsCxxVideoFrame for EncodedVideoFrame {
    fn as_cxx_video_frame_ref(&self) -> Result<&ArcasCxxVideoFrame> {
        self.video_frame.as_ref().ok_or_else(|| {
            error::WebRTCError::CXXUnwrapError("failed to unwrap video frame".into())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_raw_video_frame() {
        let slice = [0x01u8, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let bytes = Bytes::copy_from_slice(&slice);
        let video_frame = RawVideoFrame::create(10, 10, 0, bytes.clone()).unwrap();
        let _video_frame_ref = video_frame.as_cxx_video_frame_ref().unwrap();

        // Drop tests.
        {
            RawVideoFrame::create(10, 10, 0, bytes.clone()).unwrap();
            RawVideoFrame::create(10, 10, 0, bytes.clone()).unwrap();
            RawVideoFrame::create(10, 10, 0, bytes.clone()).unwrap();
            RawVideoFrame::create(10, 10, 0, bytes).unwrap();
        }
    }
}
