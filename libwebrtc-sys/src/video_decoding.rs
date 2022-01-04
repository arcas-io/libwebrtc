use std::pin::Pin;

use cxx::{CxxVector, UniquePtr};

use crate::sdp_video_format::ffi::ArcasCxxSdpVideoFormat;

use self::ffi::ArcasVideoDecoderFactoryCodecSupport;

#[cxx::bridge]
pub mod ffi {
    #[derive(Debug)]
    struct ArcasVideoDecoderFactoryCodecSupport {
        is_supported: bool,
        is_power_efficient: bool,
    }

    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/video_decoder_factory.h");
        include!("include/video_decoder.h");
        include!("include/video_frame.h");
        include!("libwebrtc-sys/src/video_frame.rs.h");
        include!("libwebrtc-sys/src/shared_bridge.rs.h");

        type ArcasVideoDecoderFactory;
        type ArcasCxxSdpVideoFormat = crate::shared_bridge::ffi::ArcasCxxSdpVideoFormat;
        type ArcasVideoDecoder;
        type ArcasDecodedImageCallback;
        type ArcasCxxEncodedImage = crate::shared_bridge::ffi::ArcasCxxEncodedImage;
        type ArcasCxxVideoFrame = crate::video_frame::ffi::ArcasCxxVideoFrame;

        fn create_arcas_video_decoder_factory(
            factory: Box<VideoDecoderFactoryProxy>,
        ) -> UniquePtr<ArcasVideoDecoderFactory>;

        // ArcasDecodedImageCallback
        fn decoded(self: &ArcasDecodedImageCallback, frame: Pin<&mut ArcasCxxVideoFrame>) -> i32;
    }

    extern "Rust" {
        #[rust_name = "VideoDecoderFactoryProxy"]
        type ArcasRustVideoDecoderFactory;
        #[rust_name = "VideoDecoderProxy"]
        type ArcasRustVideoDecoder;

        // ArcasRustVideoDecoder
        fn decode(
            self: &mut VideoDecoderProxy,
            image: &ArcasCxxEncodedImage,
            missing_frames: bool,
            render_time_ms: i64,
            callback: &ArcasDecodedImageCallback,
        ) -> i32;

        fn release(self: &mut VideoDecoderProxy) -> i32;

        fn get_num_frames_received(self: &VideoDecoderProxy) -> i32;

        // ArcasRustVideoDecoderFactory
        fn get_supported_formats(
            self: &VideoDecoderFactoryProxy,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

        fn query_codec_support(
            self: &VideoDecoderFactoryProxy,
            format: &ArcasCxxSdpVideoFormat,
            reference_scaling: bool,
        ) -> ArcasVideoDecoderFactoryCodecSupport;

        fn create_video_decoder(
            self: &mut VideoDecoderFactoryProxy,
            format: &ArcasCxxSdpVideoFormat,
        ) -> Box<VideoDecoderProxy>;
    }

    unsafe extern "C++" {
        include!("include/peerconnection_factory_config.h");
        type ArcasPeerConnectionFactoryConfig =
            crate::peerconnection_factory_config::ffi::ArcasPeerConnectionFactoryConfig;

        fn set_video_decoder_factory(
            self: Pin<&mut ArcasPeerConnectionFactoryConfig>,
            factory: Box<VideoDecoderFactoryProxy>,
        );
    }
}

pub trait VideoDecoderFactoryImpl {
    /// Returns a list of supported video formats in order of preference, to use
    /// for signaling etc.
    fn get_supported_formats(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

    /// Query whether the specifed format is supported or not and if it will be
    /// power efficient, which is currently interpreted as if there is support for
    /// hardware acceleration.
    /// The parameter `reference_scaling` is used to query support for prediction
    /// across spatial layers. An example where support for reference scaling is
    /// needed is if the video stream is produced with a scalability mode that has
    /// a dependency between the spatial layers. See
    /// https://w3c.github.io/webrtc-svc/#scalabilitymodes* for a specification of
    /// different scalabilty modes. NOTE: QueryCodecSupport is currently an
    /// experimental feature that is subject to change without notice.
    fn query_codec_support(
        &self,
        format: &ArcasCxxSdpVideoFormat,
        reference_scaling: bool,
    ) -> ArcasVideoDecoderFactoryCodecSupport;

    /// Creates a VideoDecoder for the specified format.
    fn create_video_decoder(&mut self, format: &ArcasCxxSdpVideoFormat) -> Box<VideoDecoderProxy>;
}

pub struct VideoDecoderFactoryProxy {
    api: Box<dyn VideoDecoderFactoryImpl>,
}

impl VideoDecoderFactoryProxy {
    pub fn new(api: Box<dyn VideoDecoderFactoryImpl>) -> Self {
        Self { api }
    }

    pub fn get_supported_formats(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.get_supported_formats()
    }

    pub fn query_codec_support(
        &self,
        format: &ArcasCxxSdpVideoFormat,
        reference_scaling: bool,
    ) -> ArcasVideoDecoderFactoryCodecSupport {
        self.api.query_codec_support(format, reference_scaling)
    }

    pub fn create_video_decoder(
        &mut self,
        format: &ArcasCxxSdpVideoFormat,
    ) -> Box<VideoDecoderProxy> {
        self.api.create_video_decoder(format)
    }
}

pub struct DecodedImageCallback<'a> {
    cb: &'a self::ffi::ArcasDecodedImageCallback,
}

impl<'a> DecodedImageCallback<'a> {
    pub fn new(cb: &'a self::ffi::ArcasDecodedImageCallback) -> Self {
        Self { cb }
    }

    pub fn decoded(&self, frame: Pin<&mut self::ffi::ArcasCxxVideoFrame>) -> i32 {
        self.cb.decoded(frame)
    }
}

pub trait VideoDecoderImpl {
    fn decode(
        &mut self,
        image: &self::ffi::ArcasCxxEncodedImage,
        missing_frames: bool,
        render_times_ms: i64,
        cb: DecodedImageCallback,
    ) -> i32;

    fn release(&mut self) -> i32;

    fn get_num_frames_received(&self) -> i32;
}

pub struct VideoDecoderProxy {
    decoder: Box<dyn VideoDecoderImpl>,
}

impl VideoDecoderProxy {
    pub fn new(decoder: Box<dyn VideoDecoderImpl>) -> Self {
        Self { decoder }
    }

    pub fn decode(
        &mut self,
        image: &self::ffi::ArcasCxxEncodedImage,
        missing_frames: bool,
        render_times_ms: i64,
        decoded_image_callback: &self::ffi::ArcasDecodedImageCallback,
    ) -> i32 {
        self.decoder.decode(
            image,
            missing_frames,
            render_times_ms,
            DecodedImageCallback::new(decoded_image_callback),
        )
    }

    pub fn release(&mut self) -> i32 {
        self.decoder.release()
    }

    pub fn get_num_frames_received(&self) -> i32 {
        self.decoder.get_num_frames_received()
    }
}
