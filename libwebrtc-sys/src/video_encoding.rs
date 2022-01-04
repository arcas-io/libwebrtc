use cxx::{CxxVector, UniquePtr};

use self::ffi::{
    ArcasCxxDataRate, ArcasCxxSdpVideoFormat, ArcasCxxVideoFrameType, ArcasEncodedImageCallback,
    ArcasVideoEncoderFactoryCodecInfo, ArcasVideoEncoderFactoryCodecSupport,
};

lazy_static::lazy_static! {
    static ref WEBRTC_VIDEO_ENCODING_ERR: self::ffi::ArcasVideoEncodingErrCode = self::ffi::get_arcas_video_encoding_err_codes();

    pub static ref VIDEO_CODEC_OK_REQUEST_KEYFRAME: i32 =
        WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_OK_REQUEST_KEYFRAME;

    pub static ref VIDEO_CODEC_NO_OUTPUT: i32 =
        WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_NO_OUTPUT;

    pub static ref VIDEO_CODEC_OK: i32 =
        WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_OK;

    pub static ref VIDEO_CODEC_ERROR: i32 =
        WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_ERROR;

    pub static ref VIDEO_CODEC_MEMORY: i32 =
        WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_MEMORY;

    pub static ref VIDEO_CODEC_ERR_PARAMETER: i32 =
        WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_ERR_PARAMETER;

    pub static ref VIDEO_CODEC_UNINITIALIZED: i32 =
        WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_UNINITIALIZED;

    pub static ref VIDEO_CODEC_FALLBACK_SOFTWARE: i32 =
        WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_FALLBACK_SOFTWARE;

    pub static ref VIDEO_CODEC_TARGET_BITRATE_OVERSHOOT: i32 =
        WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_TARGET_BITRATE_OVERSHOOT;

    pub static ref VIDEO_CODEC_ERR_SIMULCAST_PARAMETERS_NOT_SUPPORTED: i32 =
        WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_ERR_SIMULCAST_PARAMETERS_NOT_SUPPORTED;

    pub static ref VIDEO_CODEC_ENCODER_FAILURE: i32 =
        WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_ENCODER_FAILURE;
}

#[cxx::bridge]
pub mod ffi {
    #[derive(Debug)]
    struct ArcasVideoEncoderFactoryCodecInfo {
        has_internal_source: bool,
    }
    #[derive(Debug)]
    struct ArcasVideoEncoderFactoryCodecSupport {
        is_supported: bool,
        is_power_efficient: bool,
    }

    #[derive(Debug)]
    struct ArcasVideoEncodingErrCode {
        pub VIDEO_CODEC_OK_REQUEST_KEYFRAME: i32,
        pub VIDEO_CODEC_NO_OUTPUT: i32,
        pub VIDEO_CODEC_OK: i32,
        pub VIDEO_CODEC_ERROR: i32,
        pub VIDEO_CODEC_MEMORY: i32,
        pub VIDEO_CODEC_ERR_PARAMETER: i32,
        pub VIDEO_CODEC_UNINITIALIZED: i32,
        pub VIDEO_CODEC_FALLBACK_SOFTWARE: i32,
        pub VIDEO_CODEC_TARGET_BITRATE_OVERSHOOT: i32,
        pub VIDEO_CODEC_ERR_SIMULCAST_PARAMETERS_NOT_SUPPORTED: i32,
        pub VIDEO_CODEC_ENCODER_FAILURE: i32,
    }

    #[derive(Debug, Clone)]
    struct ArcasVideoEncoderResolutionBitrateLimits {
        frame_size_pixels: i32,
        min_start_bitrate_bps: i32,
        min_bitrate_bps: i32,
        max_bitrate_bps: i32,
    }

    #[derive(Debug, Clone)]
    struct ArcasVideoEncoderQpThresholds {
        low: i32,
        high: i32,
    }

    #[derive(Debug, Clone)]
    struct ArcasVideoEncoderScalingSettings {
        // When this is true other values are completely ignored.
        // used in rust -> C++ only (not the reverse)
        kOff: bool,

        // used in rust -> C++ only (not the reverse)
        low: i32,
        // used in rust -> C++ only (not the reverse)
        high: i32,

        min_pixels: i32,
        // Used as an "optional" type in C++.
        thresholds: Vec<ArcasVideoEncoderQpThresholds>,
    }

    #[derive(Debug, Clone)]
    struct ArcasVideoEncoderInfoFPSAllocation {
        allocation: Vec<u8>,
    }

    #[derive(Debug, Clone)]
    struct ArcasVideoEncoderInfo {
        scaling_settings: ArcasVideoEncoderScalingSettings,
        requested_resolution_alignment: i32,
        apply_alignment_to_all_simulcast_layers: bool,
        supports_native_handle: bool,
        implementation_name: String,
        has_trusted_rate_controller: bool,
        is_hardware_accelerated: bool,
        has_internal_source: bool,
        fps_allocation: Vec<ArcasVideoEncoderInfoFPSAllocation>,
        resolution_bitrate_limits: Vec<ArcasVideoEncoderResolutionBitrateLimits>,
        supports_simulcast: bool,
        preferred_pixel_formats: Vec<ArcasCxxVideoFrameBufferType>,
        // For some reason we can't use boolean in Vec here.
        is_qp_trusted: Vec<u8>, // 0 = false, 1 = true
    }

    #[derive(Debug)]
    struct ArcasVideoEncoderLossNotification {
        pub timestamp_of_last_decodable: u32,
        pub timestamp_of_last_received: u32,
        // we can't use bool here in the vec so we send a u8 0 = false, 1 = true
        pub dependencies_of_last_received_decodable: Vec<u8>,
        pub last_received_decodable: Vec<u8>,
    }

    #[derive(Debug)]
    struct ArcasEncodedImageCallbackResult {
        error: ArcasCxxEncodedImageCallbackResultError,
        frame_id: u32,
        drop_next_frame: bool,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasCxxVideoFrameBufferType {
        kNative,
        kI420,
        kI420A,
        kI444,
        kI010,
        kNV12,
    }

    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/video_encoder_factory.h");
        include!("include/encoded_image_callback.h");
        include!("include/video_encoding_wrapper.h");
        include!("include/video_encoder.h");
        include!("libwebrtc-sys/src/video_frame.rs.h");
        type ArcasVideoEncoderFactory;
        type ArcasCxxDataRate = crate::shared_bridge::ffi::ArcasCxxDataRate;
        type ArcasVideoEncoder;
        // type ArcasVideoEncoderWrapper;
        type ArcasVideoEncoderSettings;
        type ArcasVideoEncoderRateControlParameters;
        type ArcasEncodedImageCallback;
        type ArcasVideoCodec = crate::video_codec::ffi::ArcasVideoCodec;
        type ArcasCxxVideoFrameBufferType;
        type ArcasCxxEncodedImageCallbackResultError =
            crate::shared_bridge::ffi::ArcasCxxEncodedImageCallbackResultError;
        type ArcasCxxVideoBitrateAllocation =
            crate::shared_bridge::ffi::ArcasCxxVideoBitrateAllocation;
        type ArcasCxxEncodedImage = crate::shared_bridge::ffi::ArcasCxxEncodedImage;
        type ArcasCodecSpecificInfo = crate::codec_specific_info::ffi::ArcasCodecSpecificInfo;
        type ArcasCxxVideoCodec = crate::shared_bridge::ffi::ArcasCxxVideoCodec;
        type ArcasCxxVideoFrame = crate::video_frame::ffi::ArcasCxxVideoFrame;
        type ArcasCxxVideoFrameType = crate::video_frame::ffi::ArcasCxxVideoFrameType;
        type ArcasCxxSdpVideoFormat = crate::shared_bridge::ffi::ArcasCxxSdpVideoFormat;
        type ArcasVideoEncoderDropReason = crate::shared_bridge::ffi::ArcasVideoEncoderDropReason;

        fn create_arcas_video_codec() -> SharedPtr<ArcasVideoCodec>;
        fn create_arcas_video_encoder_settings(
            loss_notification: bool,
            number_of_cores: i32,
            max_payload_size: usize,
        ) -> SharedPtr<ArcasVideoEncoderSettings>;

        fn create_video_bitrate_allocation() -> UniquePtr<ArcasCxxVideoBitrateAllocation>;
        fn create_arcas_video_encoder_rate_control_parameters(
            bitrate: &ArcasCxxVideoBitrateAllocation,
            fps: f64,
        ) -> SharedPtr<ArcasVideoEncoderRateControlParameters>;

        // ArcasVideoEncoderRateControlParameters
        fn get_target_bitrate(
            self: &ArcasVideoEncoderRateControlParameters,
        ) -> &ArcasCxxVideoBitrateAllocation;

        fn get_bitrate(
            self: &ArcasVideoEncoderRateControlParameters,
        ) -> &ArcasCxxVideoBitrateAllocation;

        fn get_framerate_fps(self: &ArcasVideoEncoderRateControlParameters) -> f64;
        fn get_bytes_per_second(self: &ArcasVideoEncoderRateControlParameters) -> i64;

        // ArcasEncodedImageCallback
        unsafe fn on_encoded_image(
            self: &ArcasEncodedImageCallback,
            image: &ArcasCxxEncodedImage,
            info: *const ArcasCodecSpecificInfo,
        ) -> ArcasEncodedImageCallbackResult;

        // ArcasCxxEncodedImage
        #[cxx_name = "SetTimestamp"]
        fn set_timestamp(self: Pin<&mut ArcasCxxEncodedImage>, timestamp: u32);
        #[cxx_name = "SetEncodeTime"]
        fn set_encode_time(
            self: Pin<&mut ArcasCxxEncodedImage>,
            encode_start_time: i64,
            encode_end_time: i64,
        );
        fn size(self: &ArcasCxxEncodedImage) -> usize;
        fn data(self: &ArcasCxxEncodedImage) -> *const u8;
        #[cxx_name = "Timestamp"]
        fn timestamp(self: &ArcasCxxEncodedImage) -> u32;
        #[cxx_name = "NtpTimeMs"]
        fn ntp_time_ms(self: &ArcasCxxEncodedImage) -> i64;

        // ArcasCxxVideoBitrateAllocation
        #[cxx_name = "SetBitrate"]
        fn set_bitrate(
            self: Pin<&mut ArcasCxxVideoBitrateAllocation>,
            spatial_index: usize,
            temporal_index: usize,
            bitrate: u32,
        ) -> bool;

        #[cxx_name = "HasBitrate"]
        fn has_bitrate(
            self: &ArcasCxxVideoBitrateAllocation,
            spatial_index: usize,
            temporal_index: usize,
        ) -> bool;

        #[cxx_name = "IsSpatialLayerUsed"]
        fn is_spatial_layer_used(
            self: &ArcasCxxVideoBitrateAllocation,
            spatial_index: usize,
        ) -> bool;

        #[cxx_name = "GetSpatialLayerSum"]
        fn get_spatial_layer_sum(
            self: &ArcasCxxVideoBitrateAllocation,
            spatial_index: usize,
        ) -> u32;

        #[cxx_name = "GetTemporalLayerSum"]
        fn get_temporal_layer_sum(
            self: &ArcasCxxVideoBitrateAllocation,
            spatial_index: usize,
            temporal_index: usize,
        ) -> u32;

        #[cxx_name = "get_sum_bps"]
        fn get_sum_bps(self: &ArcasCxxVideoBitrateAllocation) -> u32;
        #[cxx_name = "get_sum_kbps"]
        fn get_sum_kbps(self: &ArcasCxxVideoBitrateAllocation) -> u32;
        #[cxx_name = "is_bw_limited"]
        fn is_bw_limited(self: &ArcasCxxVideoBitrateAllocation) -> bool;

        // Returns a type which is purely opaque to be passed into another C++ call.
        fn create_arcas_video_encoder_factory(
            factory: Box<VideoEncoderFactoryProxy>,
        ) -> UniquePtr<ArcasVideoEncoderFactory>;

        fn get_arcas_video_encoding_err_codes() -> ArcasVideoEncodingErrCode;

        // ArcasCxxDataRate
        fn bytes_per_sec(self: &ArcasCxxDataRate) -> i64;
        fn kbps(self: &ArcasCxxDataRate) -> i64;
        fn bps_or(self: &ArcasCxxDataRate, fallback: i64) -> i64;
        fn kbps_or(self: &ArcasCxxDataRate, fallback: i64) -> i64;
    }

    extern "Rust" {
        #[rust_name = "VideoEncoderFactoryProxy"]
        type ArcasRustVideoEncoderFactory;
        #[rust_name = "VideoEncoderSelectorProxy"]
        type ArcasRustVideoEncoderSelector;

        // ArcasRustVideoEncoderSelector
        fn on_current_encoder(self: &VideoEncoderSelectorProxy, format: &ArcasCxxSdpVideoFormat);

        fn on_available_bitrate(
            self: &VideoEncoderSelectorProxy,
            data_rate: &ArcasCxxDataRate,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

        fn on_encoder_broken(
            self: &VideoEncoderSelectorProxy,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

        fn get_supported_formats(
            self: &VideoEncoderFactoryProxy,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;
        fn get_implementations(
            self: &VideoEncoderFactoryProxy,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;
        fn query_video_encoder(
            self: &VideoEncoderFactoryProxy,
            format: &ArcasCxxSdpVideoFormat,
        ) -> ArcasVideoEncoderFactoryCodecInfo;

        fn query_codec_support(
            self: &VideoEncoderFactoryProxy,
            format: &ArcasCxxSdpVideoFormat,
            scalability_mode: Vec<String>,
        ) -> ArcasVideoEncoderFactoryCodecSupport;

        fn create_video_encoder(
            self: &VideoEncoderFactoryProxy,
            format: &ArcasCxxSdpVideoFormat,
        ) -> Box<VideoEncoderProxy>;

        fn get_encoder_selector(self: &VideoEncoderFactoryProxy) -> Vec<VideoEncoderSelectorProxy>;

    }

    extern "Rust" {
        #[rust_name = "VideoEncoderProxy"]
        type ArcasRustVideoEncoder;

        // ArcasRustVideoEncoder
        unsafe fn init_encode(
            self: &mut VideoEncoderProxy,
            codec_settings: *const ArcasCxxVideoCodec,
            number_of_cores: i32,
            max_payload_size: usize,
        ) -> i32;

        fn register_encode_complete_callback(
            self: &mut VideoEncoderProxy,
            callback: UniquePtr<ArcasEncodedImageCallback>,
        ) -> i32;

        fn release(self: &mut VideoEncoderProxy) -> i32;

        unsafe fn encode(
            self: &mut VideoEncoderProxy,
            frame: &ArcasCxxVideoFrame,
            frame_types: *const CxxVector<ArcasCxxVideoFrameType>,
        ) -> i32;

        fn set_rates(
            self: &mut VideoEncoderProxy,
            parameters: UniquePtr<ArcasVideoEncoderRateControlParameters>,
        );

        fn on_packet_loss_rate_update(self: &mut VideoEncoderProxy, packet_loss_rate: f32);

        fn on_rtt_update(self: &mut VideoEncoderProxy, rtt: i64);

        fn on_loss_notification(
            self: &mut VideoEncoderProxy,
            loss_notification: ArcasVideoEncoderLossNotification,
        );

        fn get_encoder_info(self: &VideoEncoderProxy) -> ArcasVideoEncoderInfo;

    }

    unsafe extern "C++" {
        include!("include/peerconnection_factory_config.h");
        type ArcasPeerConnectionFactoryConfig =
            crate::peerconnection_factory_config::ffi::ArcasPeerConnectionFactoryConfig;

        fn set_video_encoder_factory(
            self: Pin<&mut ArcasPeerConnectionFactoryConfig>,
            factory: Box<VideoEncoderFactoryProxy>,
        );
    }
}

pub trait VideoEncoderImpl {
    /// Initialize the encoder with the information from the codecSettings
    ///
    /// Input:
    ///          - codec_settings    : Codec settings
    ///          - settings          : Settings affecting the encoding itself.
    /// Input for deprecated version:
    ///          - number_of_cores   : Number of cores available for the encoder
    ///          - max_payload_size  : The maximum size each payload is allowed
    ///                                to have. Usually MTU - overhead.
    ///
    /// Return value                  : Set bit rate if OK
    ///                                 <0 - Errors:
    ///                                  WEBRTC_VIDEO_CODEC_ERR_PARAMETER
    ///                                  WEBRTC_VIDEO_CODEC_ERR_SIZE
    ///                                  WEBRTC_VIDEO_CODEC_MEMORY
    ///                                  WEBRTC_VIDEO_CODEC_ERROR
    unsafe fn init_encode(
        &mut self,
        codec_settings: *const self::ffi::ArcasCxxVideoCodec,
        number_of_cores: i32,
        max_payload_size: usize,
    ) -> i32;

    /// Register an encode complete callback object.
    ///
    /// Input:
    ///          - callback         : Callback object which handles encoded images.
    ///
    /// Return value                : WEBRTC_VIDEO_CODEC_OK if OK, < 0 otherwise.
    fn register_encode_complete_callback(
        &mut self,
        callback: UniquePtr<ArcasEncodedImageCallback>,
    ) -> i32;

    /// Free encoder memory.
    /// Return value                : WEBRTC_VIDEO_CODEC_OK if OK, < 0 otherwise.
    fn release(&mut self) -> i32;

    /// Encode an image (as a part of a video stream). The encoded image
    /// will be returned to the user through the encode complete callback.
    ///
    /// Input:
    ///          - frame             : Image to be encoded
    ///          - frame_types       : Frame type to be generated by the encoder.
    ///
    /// Return value                 : WEBRTC_VIDEO_CODEC_OK if OK
    ///                                <0 - Errors:
    ///                                  WEBRTC_VIDEO_CODEC_ERR_PARAMETER
    ///                                  WEBRTC_VIDEO_CODEC_MEMORY
    ///                                  WEBRTC_VIDEO_CODEC_ERROR
    unsafe fn encode(
        &mut self,
        frame: &self::ffi::ArcasCxxVideoFrame,
        frame_types: *const CxxVector<ArcasCxxVideoFrameType>,
    ) -> i32;

    /// Sets rate control parameters: bitrate, framerate, etc. These settings are
    /// instantaneous (i.e. not moving averages) and should apply from now until
    /// the next call to SetRates().
    fn set_rates(
        &mut self,
        parameters: UniquePtr<self::ffi::ArcasVideoEncoderRateControlParameters>,
    );

    /// Inform the encoder when the packet loss rate changes.
    ///
    /// Input:   - packet_loss_rate  : The packet loss rate (0.0 to 1.0).
    fn on_packet_loss_rate_update(&mut self, packet_loss_rate: f32);

    /// Inform the encoder when the round trip time changes.
    ///
    /// Input:   - rtt_ms            : The new RTT, in milliseconds.
    fn on_rtt_update(&mut self, rtt: i64);

    /// Called when a loss notification is received.
    fn on_loss_notification(
        &mut self,
        loss_notification: self::ffi::ArcasVideoEncoderLossNotification,
    );

    /// Returns meta-data about the encoder, such as implementation name.
    /// The output of this method may change during runtime. For instance if a
    /// hardware encoder fails, it may fall back to doing software encoding using
    /// an implementation with different characteristics.
    fn get_encoder_info(&self) -> self::ffi::ArcasVideoEncoderInfo;
}

pub struct VideoEncoderProxy {
    // Obviously for an encoder pipeline dynamic dispatch is not ideal.
    // We likely need to revisit this and write custom encoders only in C++.
    api: Box<dyn VideoEncoderImpl>,
}

impl VideoEncoderProxy {
    pub fn new(api: Box<dyn VideoEncoderImpl>) -> Self {
        Self { api }
    }

    pub unsafe fn init_encode(
        &mut self,
        codec_settings: *const self::ffi::ArcasCxxVideoCodec,
        number_of_cores: i32,
        max_payload_size: usize,
    ) -> i32 {
        self.api
            .init_encode(codec_settings, number_of_cores, max_payload_size)
    }

    pub fn register_encode_complete_callback(
        &mut self,
        callback: UniquePtr<ArcasEncodedImageCallback>,
    ) -> i32 {
        self.api.register_encode_complete_callback(callback)
    }

    pub fn release(&mut self) -> i32 {
        self.api.release()
    }

    pub unsafe fn encode(
        &mut self,
        frame: &self::ffi::ArcasCxxVideoFrame,
        frame_types: *const CxxVector<ArcasCxxVideoFrameType>,
    ) -> i32 {
        self.api.encode(frame, frame_types)
    }

    pub fn get_encoder_info(&self) -> self::ffi::ArcasVideoEncoderInfo {
        self.api.get_encoder_info()
    }

    pub fn set_rates(
        &mut self,
        parameters: UniquePtr<self::ffi::ArcasVideoEncoderRateControlParameters>,
    ) {
        self.api.set_rates(parameters)
    }

    pub fn on_packet_loss_rate_update(&mut self, packet_loss_rate: f32) {
        self.api.on_packet_loss_rate_update(packet_loss_rate)
    }

    pub fn on_rtt_update(&mut self, rtt: i64) {
        self.api.on_rtt_update(rtt)
    }

    pub fn on_loss_notification(
        &mut self,
        loss_notification: self::ffi::ArcasVideoEncoderLossNotification,
    ) {
        self.api.on_loss_notification(loss_notification)
    }
}

pub trait VideoEncoderFactoryImpl {
    /// Returns a list of supported video formats in order of preference, to use
    /// for signaling etc.
    fn get_supported_formats(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

    /// Returns a list of supported video formats in order of preference, that can
    /// also be tagged with additional information to allow the VideoEncoderFactory
    /// to separate between different implementations when CreateVideoEncoder is
    /// called.
    fn get_implementations(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

    /// Returns information about how this format will be encoded. The specified
    /// format must be one of the supported formats by this factory.
    fn query_video_encoder(
        &self,
        format: &ArcasCxxSdpVideoFormat,
    ) -> ArcasVideoEncoderFactoryCodecInfo;

    /// Query whether the specifed format is supported or not and if it will be
    /// power efficient, which is currently interpreted as if there is support for
    /// hardware acceleration.
    /// See https://w3c.github.io/webrtc-svc/#scalabilitymodes* for a specification
    /// of valid values for `scalability_mode`.
    fn query_codec_support(
        &self,
        format: &ArcasCxxSdpVideoFormat,
        scalability_mode: Vec<String>,
    ) -> ArcasVideoEncoderFactoryCodecSupport;

    /// Create video encoder returning a Box'ed trait object for the VideoEncoderImpl.
    fn create_video_encoder(&self, format: &ArcasCxxSdpVideoFormat) -> Box<VideoEncoderProxy>;

    /// Return an optional encoder selector (see `VideoEncoderSelectorImpl`) empty Vec is none.
    fn get_encoder_selector(&self) -> Option<VideoEncoderSelectorProxy>;
}
pub struct VideoEncoderFactoryProxy {
    api: Box<dyn VideoEncoderFactoryImpl>,
}

impl VideoEncoderFactoryProxy {
    pub fn new(api: Box<dyn VideoEncoderFactoryImpl>) -> Self {
        Self { api }
    }

    pub fn get_supported_formats(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.get_supported_formats()
    }
    pub fn get_implementations(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.get_implementations()
    }

    pub fn query_video_encoder(
        &self,
        format: &ArcasCxxSdpVideoFormat,
    ) -> ArcasVideoEncoderFactoryCodecInfo {
        self.api.query_video_encoder(format)
    }

    pub fn query_codec_support(
        &self,
        format: &ArcasCxxSdpVideoFormat,
        scalability_mode: Vec<String>,
    ) -> ArcasVideoEncoderFactoryCodecSupport {
        self.api.query_codec_support(format, scalability_mode)
    }

    pub fn create_video_encoder(&self, format: &ArcasCxxSdpVideoFormat) -> Box<VideoEncoderProxy> {
        self.api.create_video_encoder(format)
    }

    pub fn get_encoder_selector(&self) -> Vec<VideoEncoderSelectorProxy> {
        match self.api.get_encoder_selector() {
            Some(value) => {
                vec![value]
            }
            None => {
                vec![]
            }
        }
    }
}

pub trait VideoEncoderSelectorImpl {
    /// Informs the encoder selector about which encoder that is currently being
    /// used.
    fn on_current_encoder(&self, format: &ArcasCxxSdpVideoFormat);

    /// Called every time the available bitrate is updated. Should return a
    /// non-empty if an encoder switch should be performed.
    fn on_available_bitrate(
        &self,
        data_rate: &ArcasCxxDataRate,
    ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

    /// Called if the currently used encoder reports itself as broken. Should
    /// return a non-empty if an encoder switch should be performed.
    fn on_encoder_broken(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;
}

pub struct VideoEncoderSelectorProxy {
    api: Box<dyn VideoEncoderSelectorImpl>,
}

impl VideoEncoderSelectorProxy {
    pub fn new(api: Box<dyn VideoEncoderSelectorImpl>) -> Self {
        Self { api }
    }

    pub fn on_current_encoder(&self, format: &ArcasCxxSdpVideoFormat) {
        self.api.on_current_encoder(format);
    }

    pub fn on_available_bitrate(
        &self,
        data_rate: &ArcasCxxDataRate,
    ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.on_available_bitrate(data_rate)
    }

    pub fn on_encoder_broken(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.on_encoder_broken()
    }
}
