#![allow(unreachable_patterns)]
use cxx::UniquePtr;
use ffi::ArcasAPI;
use ffi::ArcasCodecSpecificInfo;
use ffi::ArcasColorSpace;
use ffi::ArcasCxxCodecSpecificInfo;
use ffi::ArcasCxxEncodedImage;
use ffi::ArcasCxxVideoFrame;
use ffi::ArcasEncodedImageCallback;
use ffi::ArcasICECandidate;

use ffi::ArcasPeerConnectionObserver;
use ffi::ArcasRTCConfiguration;
use ffi::ArcasRTPVideoTransceiver;
use ffi::ArcasSessionDescription;
use ffi::ArcasVideoCodec;
use ffi::{ArcasPeerConnection, ArcasPeerConnectionFactory};

use ffi::ArcasVideoEncoderRateControlParameters;
use ffi::ArcasVideoEncoderSettings;

use ffi::ArcasVideoFrameEncodedImageData;
use ffi::ArcasVideoFrameRawImageData;
use ffi::ArcasVideoTrack;
use ffi::ArcasVideoTrackSource;
extern crate lazy_static;

pub mod into;
pub mod peer_connection;
pub mod video_encoder;
pub mod video_encoder_factory;

use crate::ffi::ArcasEncodedImageFactory;
pub use crate::peer_connection::PeerConnectionObserverProxy;
pub use crate::video_encoder::{EncodedImageCallbackHandler, VideoEncoderProxy};
pub use crate::video_encoder_factory::{VideoEncoderFactoryProxy, VideoEncoderSelectorProxy};

lazy_static::lazy_static! {
    static ref WEBRTC_VIDEO_ENCODING_ERR: crate::ffi::ArcasVideoEncodingErrCode = crate::ffi::get_arcas_video_encoding_err_codes();

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

/*
 * Misc notes found while building bindings.
 *
 * - You may encounter the bug: `multiple definition of ...`.
 *      - The fix is to move the implementation (that's probably in a header file) to a .cc file.
 *      - This seems to only happen with global functions.
 *
 *  - `typeinfo for ... is missing`
 *      - typically a missing virtual function implementation.
 *      - probably indicates a visibility issue where we inherit from something that cannot be linked to.
 *
 *  -  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unsupported type
 *      - probably using the alias of the type instead of the concrete name in a method or fn impl.
 *
 *  - : undefined reference to `rust::cxxbridge1::String::String(std::__1::basic_string<char, std::__1::char_traits<char>, std::__1::allocator<char> > const&)'
 *      - CXX for whatever reason fails to convert from std::string to rust::String but it does so without a type error.
 *      - the fix is to use something like this: `rust::String(value.c_str())`
 *
 *  - UniquePtr represent objects which are generally safe to send across threads (generally but not always).
 *  - SharedPtr are good for immutable objects (const functions) which are safe to send & sync across threads.
 */

#[cxx::bridge]
pub mod ffi {

    #[derive(Debug)]
    struct ArcasRustDict {
        key: String,
        value: String,
    }

    struct ArcasSessionDescriptionError {
        line: String,
        description: String,
    }

    struct ArcasCreateSessionDescriptionResult {
        ok: bool,
        session: UniquePtr<ArcasSessionDescription>,
        error: ArcasSessionDescriptionError,
    }

    struct ArcasICECandidateError {
        line: String,
        description: String,
    }

    struct ArcasCreateICECandidateResult {
        ok: bool,
        candidate: UniquePtr<ArcasICECandidate>,
        error: ArcasICECandidateError,
    }

    struct ArcasTransceiverInit {
        stream_ids: Vec<String>,
        direction: ArcasCxxRtpTransceiverDirection,
    }

    #[namespace = "webrtc"]
    #[repr(u32)]
    #[derive(Debug)]
    enum VideoRotation {
        kVideoRotation_0 = 0,
        kVideoRotation_90 = 90,
        kVideoRotation_180 = 180,
        kVideoRotation_270 = 270,
    }

    #[repr(u8)]
    enum ArcasVideoEncoderDropReason {
        kDroppedByMediaOptimizations,
        kDroppedByEncoder,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasMediaType {
        MEDIA_TYPE_AUDIO,
        MEDIA_TYPE_VIDEO,
        MEDIA_TYPE_DATA,
        MEDIA_TYPE_UNSUPPORTED,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasSDPType {
        kOffer,
        kPrAnswer,
        kAnswer,
        kRollback,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasRTCSignalingState {
        kStable,
        kHaveLocalOffer,
        kHaveLocalPrAnswer,
        kHaveRemoteOffer,
        kHaveRemotePrAnswer,
        kClosed,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasCxxVideoCodecMode {
        kRealtimeVideo,
        kScreensharing,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasCxxVideoCodecComplexity {
        kComplexityNormal = 0,
        kComplexityHigh = 1,
        kComplexityHigher = 2,
        kComplexityMax = 3,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasCxxVideoCodecType {
        kVideoCodecGeneric = 0,
        kVideoCodecVP8,
        kVideoCodecVP9,
        kVideoCodecAV1,
        kVideoCodecH264,
        kVideoCodecMultiplex,
    }

    #[derive(Debug)]
    #[namespace = "rtc"]
    #[repr(u32)]
    enum LoggingSeverity {
        LS_VERBOSE,
        LS_INFO,
        LS_WARNING,
        LS_ERROR,
        LS_NONE,
        // We must have the following to appease CXX which expects the same
        // number of enum variants as the C++ version.
        INFO = 1,
        WARNING = 2,
        LERROR = 3,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasRTCErrorType {
        /// No error.
        NONE,

        /// An operation is valid, but currently unsupported.
        /// Maps to OperationError DOMException.
        UNSUPPORTED_OPERATION,

        /// A supplied parameter is valid, but currently unsupported.
        /// Maps to OperationError DOMException.
        UNSUPPORTED_PARAMETER,

        /// General error indicating that a supplied parameter is invalid.
        /// Maps to InvalidAccessError or TypeError DOMException depending on context.
        INVALID_PARAMETER,

        /// Slightly more specific than INVALID_PARAMETER; a parameter's value was
        /// outside the allowed range.
        /// Maps to RangeError DOMException.
        INVALID_RANGE,

        /// Slightly more specific than INVALID_PARAMETER; an error occurred while
        /// parsing string input.
        /// Maps to SyntaxError DOMException.
        SYNTAX_ERROR,

        /// The object does not support this operation in its current state.
        /// Maps to InvalidStateError DOMException.
        INVALID_STATE,

        /// An attempt was made to modify the object in an invalid way.
        /// Maps to InvalidModificationError DOMException.
        INVALID_MODIFICATION,

        /// An error occurred within an underlying network protocol.
        /// Maps to NetworkError DOMException.
        NETWORK_ERROR,

        /// Some resource has been exhausted; file handles, hardware resources, ports,
        /// etc.
        /// Maps to OperationError DOMException.
        RESOURCE_EXHAUSTED,

        /// The operation failed due to an internal error.
        /// Maps to OperationError DOMException.
        INTERNAL_ERROR,

        /// An error occured that has additional data.
        /// The additional data is specified in
        /// https://w3c.github.io/webrtc-pc/#rtcerror-interface
        /// Maps to RTCError DOMException.
        OPERATION_ERROR_WITH_DATA,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasCxxVideoFrameType {
        kEmptyFrame = 0,
        // Wire format for MultiplexEncodedImagePacker seems to depend on numerical
        // values of these constants.
        kVideoFrameKey = 3,
        kVideoFrameDelta = 4,
    }

    #[derive(Debug)]
    #[repr(u32)]
    // See https://w3c.github.io/webrtc-pc/#dom-rtcicegatheringstate
    enum ArcasIceGatheringState {
        kIceGatheringNew,
        kIceGatheringGathering,
        kIceGatheringComplete,
    }

    #[derive(Debug)]
    #[repr(u32)]
    // See https://w3c.github.io/webrtc-pc/#dom-rtcpeerconnectionstate
    enum ArcasPeerConnectionState {
        kNew,
        kConnecting,
        kConnected,
        kDisconnected,
        kFailed,
        kClosed,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasRTPTransceiverDirection {
        kSendRecv,
        kSendOnly,
        kRecvOnly,
        kInactive,
        kStopped,
    }

    #[derive(Debug)]
    #[repr(u32)]
    // See https://w3c.github.io/webrtc-pc/#dom-rtciceconnectionstate
    enum ArcasIceConnectionState {
        kIceConnectionNew,
        kIceConnectionChecking,
        kIceConnectionConnected,
        kIceConnectionCompleted,
        kIceConnectionFailed,
        kIceConnectionDisconnected,
        kIceConnectionClosed,
        kIceConnectionMax,
    }

    #[derive(Debug)]
    #[repr(u32)]
    // TLS certificate policy.
    enum ArcasTlsCertPolicy {
        // For TLS based protocols, ensure the connection is secure by not
        // circumventing certificate validation.
        kTlsCertPolicySecure,
        // For TLS based protocols, disregard security completely by skipping
        // certificate validation. This is insecure and should never be used unless
        // security is irrelevant in that particular context.
        kTlsCertPolicyInsecureNoCheck,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasSDPSemantics {
        kPlanB,
        kUnifiedPlan,
    }

    #[derive(Debug)]
    struct ArcasSdpVideoFormatInit {
        name: String,
        parameters: Vec<ArcasRustDict>,
    }

    #[derive(Debug)]
    struct ArcasSdpVideoFormatVecInit {
        list: Vec<ArcasSdpVideoFormatInit>,
    }

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
    #[repr(u32)]
    enum ArcasCxxEncodedImageCallbackResultError {
        OK,
        ERROR_SEND_FAILED,
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
    #[repr(u32)]
    enum ArcasCxxVideoFrameBufferType {
        kNative,
        kI420,
        kI420A,
        kI444,
        kI010,
        kNV12,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasCxxRtpTransceiverDirection {
        kSendRecv,
        kSendOnly,
        kRecvOnly,
        kInactive,
        kStopped,
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
    enum ArcasCxxInterLayerPredMode {
        kOff = 0,      // Inter-layer prediction is disabled.
        kOn = 1,       // Inter-layer prediction is enabled.
        kOnKeyPic = 2, // Inter-layer prediction is enabled but limited to key frames.
    }

    #[derive(Debug)]
    struct ArcasCandidatePairChangeEvent {
        selected_remote_id: String,
        selected_local_id: String,
        last_data_received_ms: i64,
        reason: String,
        estimated_disconnected_time_ms: i64,
    }

    #[derive(Debug, Clone)]
    struct ArcasICEServer {
        urls: Vec<String>,
        username: String,
        password: String,
    }

    #[derive(Debug)]
    struct ArcasPeerConnectionConfig {
        ice_servers: Vec<ArcasICEServer>,
        sdp_semantics: ArcasSDPSemantics,
    }

    // stats types
    #[derive(Debug)]
    struct ArcasVideoReceiverStats {
        pub ssrc: u32,
        pub packets_received: u32,
        pub packets_lost: i32,
        pub packets_repaired: u32,
        pub bytes_received: u64,
        pub frames_decoded: u32,
        pub keyframes_decoded: u32,
        pub frames_dropped: u32,
        pub total_decode_time: f64,
        pub frame_width: u32,
        pub frame_height: u32,
    }

    #[derive(Debug)]
    struct ArcasVideoSenderStats {
        pub ssrc: u32,
        pub packets_sent: u32,
        pub bytes_sent: u64,
        pub frames_encoded: u32,
        pub key_frames_encoded: u32,
        pub total_encode_time: f64,
        pub frame_width: u32,
        pub frame_height: u32,
        pub retransmitted_packets_sent: u64,
        pub retransmitted_bytes_sent: u64,
        pub total_packet_send_delay: f64,
        pub nack_count: u32,
        pub fir_count: u32,
        pub pli_count: u32,
        pub quality_limitation_reason: u32, // 0 - kNone, 1 - kCpu, 2 - kBandwidth, 3 - kOther
        pub quality_limitation_resolution_changes: u32,
        pub remote_packets_lost: i32,
        pub remote_jitter: f64,
        pub remote_round_trip_time: f64,
    }

    #[derive(Debug)]
    struct ArcasAudioReceiverStats {
        pub ssrc: u32,
        pub packets_received: u32,
        pub packets_lost: i32,
        pub bytes_received: u64,
        pub jitter: f64,
        pub frames_decoded: u32,
        pub total_decode_time: f64,
        pub audio_level: f64,
        pub total_audio_energy: f64,
    }

    #[derive(Debug)]
    struct ArcasAudioSenderStats {
        pub ssrc: u32,
        pub packets_sent: u32,
        pub bytes_sent: u64,
        pub remote_packets_lost: i32,
        pub remote_jitter: f64,
        pub remote_round_trip_time: f64,
        pub audio_level: f64,
        pub total_audio_energy: f64,
    }

    unsafe extern "C++" {
        include!("libwebrtc-sys/include/alias.h");
        include!("libwebrtc-sys/include/rust_entry.h");

        // cricket
        #[namespace = "cricket"]
        #[cxx_name = "Candidate"]
        type CricketCandidate;
        #[namespace = "cricket"]
        #[cxx_name = "CandidatePair"]
        type CricketCandidatePair;

        #[namespace = "webrtc"]
        type PeerConnectionInterface;

        #[namespace = "webrtc"]
        #[cxx_name = "VideoFrame"]
        type ArcasCxxVideoFrame;
        type ArcasCxxVideoEncoderRateControlParameters;
        type ArcasCxxVideoEncoderLossNotification;
        type ArcasCxxCodecSpecificInfo;
        #[namespace = "webrtc"]
        type VideoRotation;

        type ArcasSDPType;
        #[namespace = "webrtc"]
        type RTCError;
        type ArcasRTPTransceiverDirection;

        type ArcasMediaType;
        type ArcasCxxVideoCodecMode;
        type ArcasRTCSignalingState;
        type ArcasIceConnectionState;
        type ArcasPeerConnectionState;
        type ArcasIceGatheringState;
        type ArcasTlsCertPolicy;
        #[namespace = "webrtc"]
        #[cxx_name = "SdpSemantics"]
        type ArcasSDPSemantics;
        type ArcasVideoEncoderDropReason;

        #[namespace = "rtc"]
        type LoggingSeverity;

        // Our types

        // Should be left opaque in favor of the audio/video ones.
        type ArcasRTPSender;
        type ArcasRTPAudioSender;
        type ArcasRTPVideoSender;
        type ArcasVideoCodecSettings;
        type ArcasCxxVideoCodec;
        type ArcasEncodedImageCallback;
        type ArcasRTCError;
        // Should be left opaque as we can only use the video/audio concrete implementations in a useful way.
        type ArcasRTPTransceiver;
        type ArcasRTPVideoTransceiver;
        type ArcasRTPAudioTransceiver;

        // Should be left opaque in favor of the audio/video ones
        type ArcasRTPReceiver;
        type ArcasRTPVideoReceiver;
        type ArcasRTPAudioReceiver;
        type ArcasRTPCodecCapability;
        type ArcasVideoEncoder;
        type ArcasRTPHeaderExtensionCapability;
        type ArcasCodecSpecificInfo;
        type ArcasMediaStream;
        type ArcasDataChannel;
        type ArcasPeerConnectionFactory;
        type ArcasSessionDescription;
        type ArcasPeerConnection;
        type ArcasRTCConfiguration;
        type ArcasColorSpace;
        type ArcasICECandidate;
        type ArcasCreateSessionDescriptionObserver;
        type ArcasSetDescriptionObserver;
        type ArcasCxxEncodedImageCallbackResultError;
        type ArcasCxxVideoBitrateAllocation;
        type ArcasCxxVideoFrameBufferType;
        type ArcasCxxSdpVideoFormat;
        type ArcasCxxDataRate;

        // Information about the video encoder factory.
        type ArcasVideoEncoderFactory;
        type ArcasVideoEncoderRateControlParameters;
        type ArcasVideoTrack;
        type ArcasVideoTrackSource;
        type ArcasCxxEncodedImage;
        type ArcasCxxVideoCodecType;
        type ArcasSpatialLayer;
        type ArcasVideoCodec;
        type ArcasRTCErrorType;
        type ArcasCxxRefCountedEncodedImageBuffer;
        type ArcasOpaqueEncodedImageBuffer;
        type ArcasEncodedImageFactory;
        type ArcasPeerConnectionObserver;
        type ArcasRTCStatsCollectorCallback;
        type ArcasCxxRtpTransceiverDirection;
        type ArcasVideoFrameInternal;
        type ArcasVideoFrameEncodedImageData;
        type ArcasVideoFrameRawImageData;
        type ArcasCxxVideoCodecComplexity;
        type ArcasVideoFrameTypesCollection;
        /// This type must not cross a thread boundary.
        type ArcasVideoEncoderFactoryWrapper;
        type ArcasCxxInterLayerPredMode;
        type ArcasSDPVideoFormatWrapper;
        /// VideoEncoder's cannot pass a thread boundary.
        type ArcasVideoEncoderWrapper;
        type ArcasVideoEncoderSettings;
        type ArcasVideoFrameFactory;
        type ArcasAPI;
        type ArcasCxxVideoEncoder;
        type ArcasCxxVideoFrameType;
        type ArcasCxxVideoEncoderSettings;
        type ArcasCxxVideoEncoderEncoderInfo;
        type ArcasReactiveVideoEncoderWrapper;
        type ArcasVideoFrameBufferEmpty;

        // wrapper functions around constructors.
        fn create_arcas_api() -> UniquePtr<ArcasAPI>;

        fn create_arcas_video_track_source() -> UniquePtr<ArcasVideoTrackSource>;
        fn create_arcas_encoded_image_factory() -> UniquePtr<ArcasEncodedImageFactory>;
        fn create_arcas_codec_specific_info() -> UniquePtr<ArcasCodecSpecificInfo>;
        fn create_arcas_color_space() -> UniquePtr<ArcasColorSpace>;
        fn create_arcas_video_frame_factory() -> UniquePtr<ArcasVideoFrameFactory>;
        fn create_arcas_video_encoder_factory_from_builtin(
        ) -> UniquePtr<ArcasVideoEncoderFactoryWrapper>;
        fn create_arcas_session_description(
            sdp_type: ArcasSDPType,
            sdp: String,
        ) -> ArcasCreateSessionDescriptionResult;
        fn create_arcas_ice_candidate(
            sdp_mid: String,
            sdp_mline_index: u32,
            sdp: String,
        ) -> ArcasCreateICECandidateResult;

        fn extract_arcas_video_frame_to_raw_frame_buffer(
            video_frame: &ArcasCxxVideoFrame,
        ) -> UniquePtr<ArcasVideoFrameEncodedImageData>;

        fn create_peer_connection_observer(
            observer: Box<PeerConnectionObserverProxy>,
        ) -> UniquePtr<ArcasPeerConnectionObserver>;

        fn create_arcas_video_frame_types_collection(
            rust_array: Vec<ArcasCxxVideoFrameType>,
        ) -> SharedPtr<ArcasVideoFrameTypesCollection>;

        fn create_arcas_spatial_layer() -> SharedPtr<ArcasSpatialLayer>;
        unsafe fn create_arcas_video_codec_from_cxx(
            ptr: *const ArcasCxxVideoCodec,
        ) -> UniquePtr<ArcasVideoCodec>;
        fn create_arcas_video_codec() -> SharedPtr<ArcasVideoCodec>;
        fn create_arcas_video_encoder_settings(
            loss_notification: bool,
            number_of_cores: i32,
            max_payload_size: usize,
        ) -> SharedPtr<ArcasVideoEncoderSettings>;

        fn get_arcas_video_encoding_err_codes() -> ArcasVideoEncodingErrCode;
        fn create_video_bitrate_allocation() -> UniquePtr<ArcasCxxVideoBitrateAllocation>;
        fn create_arcas_video_encoder_rate_control_parameters(
            bitrate: &ArcasCxxVideoBitrateAllocation,
            fps: f64,
        ) -> SharedPtr<ArcasVideoEncoderRateControlParameters>;

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

        // Logging
        fn set_arcas_log_to_stderr(log: bool);
        fn set_arcas_log_level(level: LoggingSeverity);

        // ArcasVideoTrackSource
        fn push_frame(self: &ArcasVideoTrackSource, video_frame: &ArcasCxxVideoFrame);
        fn cxx_clone(self: &ArcasVideoTrackSource) -> UniquePtr<ArcasVideoTrackSource>;

        // ArcasAPI
        fn create_factory(self: &ArcasAPI) -> UniquePtr<ArcasPeerConnectionFactory>;
        fn create_factory_with_arcas_video_encoder_factory(
            self: &ArcasAPI,
            video_encoder_factory: UniquePtr<ArcasVideoEncoderFactory>,
        ) -> UniquePtr<ArcasPeerConnectionFactory>;

        // ArcasPeerConnectionFactory
        /// PeerConnection objects are threadsafe and can be shared between threads.
        /// the actual work happens on the worker thread.
        ///
        /// # Safety
        ///
        /// The observer must be kept alive as long as the peer connection object.
        ///
        unsafe fn create_peer_connection(
            self: &ArcasPeerConnectionFactory,
            config: UniquePtr<ArcasRTCConfiguration>,
            observer: *mut ArcasPeerConnectionObserver,
        ) -> SharedPtr<ArcasPeerConnection>;

        fn create_video_track(
            self: &ArcasPeerConnectionFactory,
            id: String,
            source: &ArcasVideoTrackSource,
        ) -> UniquePtr<ArcasVideoTrack>;

        // ArcasPeerConnection
        fn create_offer(
            self: &ArcasPeerConnection,
            observer: Box<ArcasRustCreateSessionDescriptionObserver>,
        );
        fn create_answer(
            self: &ArcasPeerConnection,
            observer: Box<ArcasRustCreateSessionDescriptionObserver>,
        );
        fn set_local_description(
            self: &ArcasPeerConnection,
            observer: Box<ArcasRustSetSessionDescriptionObserver>,
            session: UniquePtr<ArcasSessionDescription>,
        );

        fn set_remote_description(
            self: &ArcasPeerConnection,
            observer: Box<ArcasRustSetSessionDescriptionObserver>,
            session: UniquePtr<ArcasSessionDescription>,
        );

        fn add_video_transceiver(self: &ArcasPeerConnection)
            -> UniquePtr<ArcasRTPVideoTransceiver>;

        fn add_video_transceiver_with_track(
            self: &ArcasPeerConnection,
            track: UniquePtr<ArcasVideoTrack>,
            init: ArcasTransceiverInit,
        ) -> UniquePtr<ArcasRTPVideoTransceiver>;

        fn add_audio_transceiver(self: &ArcasPeerConnection)
            -> UniquePtr<ArcasRTPAudioTransceiver>;

        fn add_video_track(
            self: &ArcasPeerConnection,
            track: UniquePtr<ArcasVideoTrack>,
            stream_ids: Vec<String>,
        );

        fn get_stats(self: &ArcasPeerConnection, callback: Box<ArcasRustRTCStatsCollectorCallback>);
        fn add_ice_candidate(self: &ArcasPeerConnection, candidate: UniquePtr<ArcasICECandidate>);
        fn close(self: &ArcasPeerConnection);

        // session description
        #[cxx_name = "to_string"]
        fn cxx_to_string(self: &ArcasSessionDescription) -> String;
        fn get_type(self: &ArcasSessionDescription) -> ArcasSDPType;
        #[cxx_name = "clone"]
        fn clone_cxx(self: &ArcasSessionDescription) -> UniquePtr<ArcasSessionDescription>;

        fn create_rtc_configuration(
            config: ArcasPeerConnectionConfig,
        ) -> UniquePtr<ArcasRTCConfiguration>;

        // ArcasRTPVideoSender
        fn set_track(self: &ArcasRTPVideoSender, track: &ArcasVideoTrack) -> bool;

        // ArcasRTPVideoTransceiver
        fn mid(self: &ArcasRTPVideoTransceiver) -> String;
        fn media_type(self: &ArcasRTPVideoTransceiver) -> ArcasMediaType;
        fn get_sender(self: &ArcasRTPVideoTransceiver) -> UniquePtr<ArcasRTPVideoSender>;
        fn get_receiver(self: &ArcasRTPVideoTransceiver) -> UniquePtr<ArcasRTPVideoReceiver>;
        fn stopped(self: &ArcasRTPVideoTransceiver) -> bool;
        fn stopping(self: &ArcasRTPVideoTransceiver) -> bool;
        fn direction(self: &ArcasRTPVideoTransceiver) -> ArcasRTPTransceiverDirection;
        fn stop(self: &ArcasRTPVideoTransceiver) -> UniquePtr<ArcasRTCError>;

        fn header_extensions_to_offer(
            self: &ArcasRTPVideoTransceiver,
        ) -> UniquePtr<CxxVector<ArcasRTPHeaderExtensionCapability>>;

        fn header_extensions_to_negotiated(
            self: &ArcasRTPVideoTransceiver,
        ) -> UniquePtr<CxxVector<ArcasRTPHeaderExtensionCapability>>;

        fn codec_preferences(
            self: &ArcasRTPVideoTransceiver,
        ) -> UniquePtr<CxxVector<ArcasRTPCodecCapability>>;

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

        fn set_offerred_rtp_header_extensions(
            self: &ArcasRTPAudioTransceiver,
            extensions: UniquePtr<CxxVector<ArcasRTPHeaderExtensionCapability>>,
        ) -> UniquePtr<ArcasRTCError>;

        // ArcasRTCError
        fn ok(self: &ArcasRTCError) -> bool;
        fn kind(self: &ArcasRTCError) -> ArcasRTCErrorType;
        fn message(self: &ArcasRTCError) -> String;

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

        // ArcasCxxSdpVideoFormat
        #[cxx_name = "IsSameCodec"]
        unsafe fn is_same_codec(
            self: &ArcasCxxSdpVideoFormat,
            other: &ArcasCxxSdpVideoFormat,
        ) -> bool;

        #[cxx_name = "sdp_video_format_get_parameters"]
        fn video_format_get_parameters(format: &ArcasCxxSdpVideoFormat) -> Vec<ArcasRustDict>;

        #[cxx_name = "sdp_video_format_get_name"]
        fn sdp_video_format_get_name(format: &ArcasCxxSdpVideoFormat) -> &CxxString;

        #[cxx_name = "sdp_video_format_to_string"]
        fn sdp_video_format_to_string(format: &ArcasCxxSdpVideoFormat) -> String;

        // Helper for use with the VideoFactory methods but generically useful too
        //
        // Where optional types are used this is also how we pass "none" (empty vec)
        // These types are relevant:
        //
        //  - absl::optional<SdpVideoFormat>
        //  - std::vector<SdpVideoFormat>
        //
        fn create_sdp_video_format_list(
            format_list: ArcasSdpVideoFormatVecInit,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

        fn create_sdp_video_format(
            init: ArcasSdpVideoFormatInit,
        ) -> UniquePtr<ArcasCxxSdpVideoFormat>;

        // Returns a type which is purely opaque to be passed into another C++ call.
        fn create_arcas_video_encoder_factory(
            factory: Box<VideoEncoderFactoryProxy>,
        ) -> UniquePtr<ArcasVideoEncoderFactory>;

        // ArcasCxxDataRate
        fn bytes_per_sec(self: &ArcasCxxDataRate) -> i64;
        fn kbps(self: &ArcasCxxDataRate) -> i64;
        fn bps_or(self: &ArcasCxxDataRate, fallback: i64) -> i64;
        fn kbps_or(self: &ArcasCxxDataRate, fallback: i64) -> i64;

        // ArcasVideoEncoderRateControlParameters
        fn get_target_bitrate(
            self: &ArcasVideoEncoderRateControlParameters,
        ) -> &ArcasCxxVideoBitrateAllocation;

        fn get_bitrate(
            self: &ArcasVideoEncoderRateControlParameters,
        ) -> &ArcasCxxVideoBitrateAllocation;

        fn get_framerate_fps(self: &ArcasVideoEncoderRateControlParameters) -> f64;
        fn get_bytes_per_second(self: &ArcasVideoEncoderRateControlParameters) -> i64;

        // ArcasICECandidate
        fn id(self: &ArcasICECandidate) -> String;
        fn to_string(self: &ArcasICECandidate) -> String;
        fn sdp_mid(self: &ArcasICECandidate) -> String;
        fn sdp_mline_index(self: &ArcasICECandidate) -> u32;

        // ArcasEncodedImageFactory

        /// Create a new ArcasEncodedImageFactory
        ///
        /// # Safety
        ///
        /// This will *not* copy underlying memory.
        unsafe fn create_encoded_image_buffer(
            self: &ArcasEncodedImageFactory,
            data: *const u8,
            size: usize,
        ) -> SharedPtr<ArcasOpaqueEncodedImageBuffer>;

        fn create_encoded_image(self: &ArcasEncodedImageFactory)
            -> UniquePtr<ArcasCxxEncodedImage>;

        fn create_empty_encoded_image_buffer(
            self: &ArcasEncodedImageFactory,
        ) -> SharedPtr<ArcasOpaqueEncodedImageBuffer>;

        fn set_encoded_image_buffer(
            self: &ArcasEncodedImageFactory,
            video_frame: &ArcasCxxVideoFrame,
            image: UniquePtr<ArcasCxxEncodedImage>,
            buffer: &ArcasOpaqueEncodedImageBuffer,
        ) -> UniquePtr<ArcasCxxEncodedImage>;

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

        // ArcasCodecSpecificInfo
        fn set_codec_type(self: &ArcasCodecSpecificInfo, codec_type: ArcasCxxVideoCodecType);
        fn set_end_of_picture(self: &ArcasCodecSpecificInfo, set_end_of_picture: bool);
        fn get_codec_type(self: &ArcasCodecSpecificInfo) -> ArcasCxxVideoCodecType;
        #[cxx_name = "as_ref"]
        fn as_cxx_ref(self: &ArcasCodecSpecificInfo) -> &ArcasCxxCodecSpecificInfo;

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

        // ArcasVideoFrameRawImageData
        fn width(self: &ArcasVideoFrameRawImageData) -> i32;
        fn height(self: &ArcasVideoFrameRawImageData) -> i32;

        // NOTE: This clone does not copy the underlying memory and uses ref counting to keep it alive.
        fn clone(
            self: &ArcasVideoFrameEncodedImageData,
        ) -> UniquePtr<ArcasVideoFrameEncodedImageData>;

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

        // ArcasVideoEncoderFactoryWrapper
        fn get_supported_formats(
            self: &ArcasVideoEncoderFactoryWrapper,
        ) -> UniquePtr<CxxVector<ArcasSDPVideoFormatWrapper>>;

        fn cxx_get_supported_formats(
            self: &ArcasVideoEncoderFactoryWrapper,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

        fn create_encoder(
            self: &ArcasVideoEncoderFactoryWrapper,
            format: &ArcasSDPVideoFormatWrapper,
            callback: Box<EncodedImageCallbackHandler>,
        ) -> UniquePtr<ArcasVideoEncoderWrapper>;

        /// Intended to be used by rust calling back into C++ factories.
        fn create_encoder_reactive(
            self: &ArcasVideoEncoderFactoryWrapper,
            format: &ArcasCxxSdpVideoFormat,
        ) -> UniquePtr<ArcasReactiveVideoEncoderWrapper>;

        // ArcasSDPVideoFormatWrapper
        fn get_name(self: &ArcasSDPVideoFormatWrapper) -> String;
        fn get_parameters(self: &ArcasSDPVideoFormatWrapper) -> Vec<ArcasRustDict>;
        fn to_string(self: &ArcasSDPVideoFormatWrapper) -> String;
        fn clone(self: &ArcasSDPVideoFormatWrapper) -> UniquePtr<ArcasSDPVideoFormatWrapper>;
        fn cxx_format_list(
            self: &ArcasSDPVideoFormatWrapper,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

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

        // ArcasSpatialLayer
        fn get_width(self: &ArcasSpatialLayer) -> i32;
        fn get_height(self: &ArcasSpatialLayer) -> i32;
        fn get_max_framerate(self: &ArcasSpatialLayer) -> f32;
        fn get_number_of_temporal_layers(self: &ArcasSpatialLayer) -> u8;
        fn get_max_bitrate(self: &ArcasSpatialLayer) -> u32;
        fn get_target_bitrate(self: &ArcasSpatialLayer) -> u32;
        fn get_min_bitrate(self: &ArcasSpatialLayer) -> u32;
        fn get_qp_max(self: &ArcasSpatialLayer) -> u32;
        fn get_active(self: &ArcasSpatialLayer) -> bool;
        fn set_width(self: &ArcasSpatialLayer, width: i32);
        fn set_height(self: &ArcasSpatialLayer, height: i32);
        fn set_max_framerate(self: &ArcasSpatialLayer, max_frame_rate: f32);
        fn set_number_of_temporal_layers(self: &ArcasSpatialLayer, number_of_temporal_layers: u8);
        fn set_max_bitrate(self: &ArcasSpatialLayer, max_bitrate: u32);
        fn set_target_bitrate(self: &ArcasSpatialLayer, target_bitrate: u32);
        fn set_min_bitrate(self: &ArcasSpatialLayer, min_bitrate: u32);
        fn set_qp_max(self: &ArcasSpatialLayer, qp_max: u32);
        fn set_active(self: &ArcasSpatialLayer, active: bool);

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

        // XXX: Hacks to ensure CXX generates the unique ptr bindings for these return types.
        fn gen_unique_ptr1() -> UniquePtr<ArcasDataChannel>;
        fn gen_unique_ptr2() -> UniquePtr<ArcasMediaStream>;
        fn gen_unique_ptr3() -> UniquePtr<ArcasVideoCodecSettings>;
        fn gen_unique_ptr4() -> UniquePtr<ArcasPeerConnectionConfig>;
        fn gen_unique_ptr5() -> UniquePtr<ArcasSpatialLayer>;

        fn gen_shared_ptr1() -> SharedPtr<ArcasCxxEncodedImage>;
        fn gen_shared_ptr2() -> SharedPtr<ArcasCodecSpecificInfo>;
    }

    extern "Rust" {
        #[rust_name = "PeerConnectionObserverProxy"]
        type ArcasRustPeerConnectionObserver;
        type ArcasRustCreateSessionDescriptionObserver;
        type ArcasRustSetSessionDescriptionObserver;
        #[rust_name = "VideoEncoderFactoryProxy"]
        type ArcasRustVideoEncoderFactory;
        #[rust_name = "VideoEncoderProxy"]
        type ArcasRustVideoEncoder;
        #[rust_name = "VideoEncoderSelectorProxy"]
        type ArcasRustVideoEncoderSelector;
        type ArcasRustRTCStatsCollectorCallback;
        #[rust_name = "EncodedImageCallbackHandler"]
        type ArcasRustEncodedImageCallbackHandler;

        // Stats callbacks
        fn on_stats_delivered(
            self: &ArcasRustRTCStatsCollectorCallback,
            video_rx: Vec<ArcasVideoReceiverStats>,
            audio_rx: Vec<ArcasAudioReceiverStats>,
            video_tx: Vec<ArcasVideoSenderStats>,
            audio_tx: Vec<ArcasAudioSenderStats>,
        );

        // ArcasRustCreateSessionDescriptionObserver
        fn on_success(
            self: &ArcasRustCreateSessionDescriptionObserver,
            success: UniquePtr<ArcasSessionDescription>,
        );
        fn on_failure(
            self: &ArcasRustCreateSessionDescriptionObserver,
            failure: UniquePtr<ArcasRTCError>,
        );
        // ArcasRustSetSessionDescriptionObserver
        fn on_success(self: &ArcasRustSetSessionDescriptionObserver);
        fn on_failure(
            self: &ArcasRustSetSessionDescriptionObserver,
            failure: UniquePtr<ArcasRTCError>,
        );

        fn on_signaling_state_change(
            self: &PeerConnectionObserverProxy,
            state: ArcasRTCSignalingState,
        );
        fn on_add_stream(self: &PeerConnectionObserverProxy, stream: UniquePtr<ArcasMediaStream>);
        fn on_remove_stream(
            self: &PeerConnectionObserverProxy,
            stream: UniquePtr<ArcasMediaStream>,
        );
        fn on_datachannel(
            self: &PeerConnectionObserverProxy,
            data_channel: UniquePtr<ArcasDataChannel>,
        );
        fn on_renegotiation_needed(self: &PeerConnectionObserverProxy);
        fn on_renegotiation_needed_event(self: &PeerConnectionObserverProxy, event: u32);
        fn on_ice_connection_change(
            self: &PeerConnectionObserverProxy,
            state: ArcasIceConnectionState,
        );
        fn on_connection_change(
            self: &PeerConnectionObserverProxy,
            state: ArcasPeerConnectionState,
        );
        fn on_ice_gathering_change(
            self: &PeerConnectionObserverProxy,
            state: ArcasIceGatheringState,
        );
        fn on_ice_candidate(
            self: &PeerConnectionObserverProxy,
            candidate: UniquePtr<ArcasICECandidate>,
        );
        fn on_ice_candidate_error(
            self: &PeerConnectionObserverProxy,
            host_candidate: String,
            url: String,
            error_code: i32,
            error_text: String,
        );

        fn on_ice_candidate_error_address_port(
            self: &PeerConnectionObserverProxy,
            address: String,
            port: i32,
            url: String,
            error_code: i32,
            error_text: String,
        );

        fn on_ice_candidates_removed(self: &PeerConnectionObserverProxy, removed: Vec<String>);

        fn on_ice_connection_receiving_change(self: &PeerConnectionObserverProxy, receiving: bool);

        fn on_ice_selected_candidate_pair_change(
            self: &PeerConnectionObserverProxy,
            event: ArcasCandidatePairChangeEvent,
        );

        fn on_add_track(
            self: &PeerConnectionObserverProxy,
            receiver: UniquePtr<ArcasRTPReceiver>,
            // TODO: Need a collection type that we can use here.
            // streams: UniquePtr<CxxVector<UniquePtr<ArcasMediaStream>>>,
        );

        fn on_video_track(
            self: &PeerConnectionObserverProxy,
            transceiver: UniquePtr<ArcasRTPVideoTransceiver>,
        );

        fn on_audio_track(
            self: &PeerConnectionObserverProxy,
            transceiver: UniquePtr<ArcasRTPAudioTransceiver>,
        );

        fn on_remove_track(
            self: &PeerConnectionObserverProxy,
            receiver: UniquePtr<ArcasRTPReceiver>,
        );

        fn on_interesting_usage(self: &PeerConnectionObserverProxy, pattern: i32);

        // ArcasRustVideoEncoderFactory / VideoEncoderFactoryProxy

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

        // ArcasRustVideoEncoderSelector
        fn on_current_encoder(self: &VideoEncoderSelectorProxy, format: &ArcasCxxSdpVideoFormat);

        fn on_available_bitrate(
            self: &VideoEncoderSelectorProxy,
            data_rate: &ArcasCxxDataRate,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

        fn on_encoder_broken(
            self: &VideoEncoderSelectorProxy,
        ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

        // EncodedImageCallbackHandler
        fn trigger_encoded_image(
            self: &EncodedImageCallbackHandler,
            image: UniquePtr<ArcasCxxEncodedImage>,
            codec_info: UniquePtr<ArcasCodecSpecificInfo>,
        );

        fn trigger_dropped(self: &EncodedImageCallbackHandler, reason: ArcasVideoEncoderDropReason);
    }
}

pub struct ArcasRustCreateSessionDescriptionObserver {
    success: Box<dyn Fn(UniquePtr<crate::ffi::ArcasSessionDescription>)>,
    failure: Box<dyn Fn(UniquePtr<crate::ffi::ArcasRTCError>)>,
}

impl ArcasRustCreateSessionDescriptionObserver {
    pub fn new(
        success: Box<dyn Fn(UniquePtr<crate::ffi::ArcasSessionDescription>)>,
        failure: Box<dyn Fn(UniquePtr<crate::ffi::ArcasRTCError>)>,
    ) -> Self {
        Self { success, failure }
    }

    fn on_success(&self, description: UniquePtr<crate::ffi::ArcasSessionDescription>) {
        (self.success)(description);
    }
    fn on_failure(&self, err: UniquePtr<crate::ffi::ArcasRTCError>) {
        (self.failure)(err);
    }
}

pub struct ArcasRustSetSessionDescriptionObserver {
    success: Box<dyn Fn()>,
    failure: Box<dyn Fn(UniquePtr<crate::ffi::ArcasRTCError>)>,
}

impl ArcasRustSetSessionDescriptionObserver {
    pub fn new(
        success: Box<dyn Fn()>,
        failure: Box<dyn Fn(UniquePtr<crate::ffi::ArcasRTCError>)>,
    ) -> Self {
        Self { success, failure }
    }

    fn on_success(&self) {
        (self.success)();
    }
    fn on_failure(&self, err: UniquePtr<crate::ffi::ArcasRTCError>) {
        (self.failure)(err);
    }
}

type StatsCallbackFn = dyn Fn(
    Vec<crate::ffi::ArcasVideoReceiverStats>,
    Vec<crate::ffi::ArcasAudioReceiverStats>,
    Vec<crate::ffi::ArcasVideoSenderStats>,
    Vec<crate::ffi::ArcasAudioSenderStats>,
);

pub struct ArcasRustRTCStatsCollectorCallback {
    cb: Box<StatsCallbackFn>,
}

impl ArcasRustRTCStatsCollectorCallback {
    pub fn new(cb: Box<StatsCallbackFn>) -> Self {
        Self { cb }
    }

    fn on_stats_delivered(
        self: &ArcasRustRTCStatsCollectorCallback,
        video_rx: Vec<crate::ffi::ArcasVideoReceiverStats>,
        audio_rx: Vec<crate::ffi::ArcasAudioReceiverStats>,
        video_tx: Vec<crate::ffi::ArcasVideoSenderStats>,
        audio_tx: Vec<crate::ffi::ArcasAudioSenderStats>,
    ) {
        (self.cb)(video_rx, audio_rx, video_tx, audio_tx)
    }
}

unsafe impl Send for ArcasAPI {}
unsafe impl Sync for ArcasAPI {}
unsafe impl Send for ArcasVideoTrackSource {}
// This is actually unsafe and we rely on the wrappers in libwebrtc crate to ensure concurrent
// writes are not happening.
unsafe impl Sync for ArcasVideoTrackSource {}
unsafe impl Send for ArcasICECandidate {}
unsafe impl Send for ArcasRTCConfiguration {}
unsafe impl Sync for ArcasPeerConnectionObserver {}
unsafe impl Send for ArcasPeerConnectionObserver {}
unsafe impl Send for ArcasEncodedImageFactory {}
unsafe impl Send for ArcasCxxEncodedImage {}
unsafe impl Sync for ArcasCxxEncodedImage {}
unsafe impl Send for ArcasCodecSpecificInfo {}
unsafe impl Sync for ArcasCodecSpecificInfo {}
unsafe impl Send for ArcasVideoFrameEncodedImageData {}
unsafe impl Send for ArcasCxxVideoFrame {}
unsafe impl Send for ArcasVideoCodec {}
unsafe impl Send for ArcasVideoEncoderSettings {}
unsafe impl Send for ArcasVideoEncoderRateControlParameters {}
unsafe impl Sync for ArcasVideoEncoderRateControlParameters {}
unsafe impl Send for ArcasVideoFrameRawImageData {}
unsafe impl Send for ArcasColorSpace {}
unsafe impl Send for ArcasVideoTrack {}
/// There are special afforances for video tracks returend by CreateVideoTrack.
/// See peer_connection_factory.cc for details.
unsafe impl Sync for ArcasVideoTrack {}
unsafe impl Send for ArcasRTPVideoTransceiver {}
unsafe impl Sync for ArcasRTPVideoTransceiver {}
unsafe impl Sync for ArcasPeerConnectionFactory {}
unsafe impl Send for ArcasPeerConnectionFactory {}
unsafe impl Sync for ArcasPeerConnection {}
/// There are special affordances for the peer connection object that make it threadsafe.
/// libwebrtc will wrap the underlying peer connection interface in a wrapper which will forward
/// calls to the appropriate thread.
unsafe impl Send for ArcasPeerConnection {}
unsafe impl Send for ArcasSessionDescription {}
unsafe impl Sync for ArcasSessionDescription {}
unsafe impl Send for ArcasEncodedImageCallback {}
unsafe impl Sync for ArcasEncodedImageCallback {}
unsafe impl Send for ArcasCxxCodecSpecificInfo {}
unsafe impl Sync for ArcasCxxCodecSpecificInfo {}
