use std::error;
use std::fmt;
use std::sync::Arc;

use cxx::memory::SharedPtrTarget;
use cxx::CxxString;
use cxx::CxxVector;
use cxx::SharedPtr;
use cxx::UniquePtr;
use parking_lot::lock_api::RawMutex;
use parking_lot::Mutex;
use std::os::raw::c_char;
use std::os::raw::c_uint;
#[macro_use]
extern crate lazy_static;

pub mod peer_connection;
pub mod video_encoder;
pub mod video_encoder_factory;

pub use crate::peer_connection::PeerConnectionObserverProxy;
pub use crate::video_encoder::VideoEncoderProxy;
pub use crate::video_encoder_factory::{VideoEncoderFactoryProxy, VideoEncoderSelectorProxy};

lazy_static::lazy_static! {
    static ref WEBRTC_VIDEO_ENCODING_ERR: crate::ffi::ArcasVideoEncodingErrCode = crate::ffi::get_arcas_video_encoding_err_codes();
}

/**
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
 */

#[cxx::bridge]
pub mod ffi {

    #[derive(Debug)]
    struct ArcasRustDict {
        key: String,
        value: String,
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
    enum ArcasCxxVideoCodecType {
        kVideoCodecGeneric = 0,
        kVideoCodecVP8,
        kVideoCodecVP9,
        kVideoCodecAV1,
        kVideoCodecH264,
        kVideoCodecMultiplex,
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
    #[namespace = "webrtc"]
    #[cxx_name = "VideoFrameType"]
    enum ArcasVideoFrameType {
        kEmptyFrame = 0,
        // Wire format for MultiplexEncodedImagePacker seems to depend on numerical
        // values of these constants.
        kVideoFrameKey = 3,
        kVideoFrameDelta = 4,
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

    #[derive(Debug)]
    struct ArcasVideoEncoderResolutionBitrateLimits {
        frame_size_pixels: i32,
        min_start_bitrate_bps: i32,
        min_bitrate_bps: i32,
        max_bitrate_bps: i32,
    }

    #[derive(Debug)]
    struct ArcasVideoEncoderQpThresholds {
        low: i32,
        high: i32,
    }

    #[derive(Debug)]
    struct ArcasVideoEncoderScalingSettings {
        // When this is true other values are completely ignored.
        kOff: bool,
        low: i32,
        high: i32,
        min_pixels: i32,
        // Used as an "optional" type in C++.
        thresholds: Vec<ArcasVideoEncoderQpThresholds>,
    }

    #[derive(Debug)]
    struct ArcasVideoEncoderInfo {
        scaling_settings: ArcasVideoEncoderScalingSettings,
        requested_resolution_alignment: i32,
        apply_alignment_to_all_simulcast_layers: bool,
        supports_native_handle: bool,
        implementation_name: String,
        has_trusted_rate_controller: bool,
        is_hardware_accelerated: bool,
        has_internal_source: bool,
        fps_allocation: Vec<u8>,
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
    struct ArcasVideoEncodingErrCode {
        VIDEO_CODEC_OK_REQUEST_KEYFRAME: i32,
        VIDEO_CODEC_NO_OUTPUT: i32,
        VIDEO_CODEC_OK: i32,
        VIDEO_CODEC_ERROR: i32,
        VIDEO_CODEC_MEMORY: i32,
        VIDEO_CODEC_ERR_PARAMETER: i32,
        VIDEO_CODEC_UNINITIALIZED: i32,
        VIDEO_CODEC_FALLBACK_SOFTWARE: i32,
        VIDEO_CODEC_TARGET_BITRATE_OVERSHOOT: i32,
        VIDEO_CODEC_ERR_SIMULCAST_PARAMETERS_NOT_SUPPORTED: i32,
        VIDEO_CODEC_ENCODER_FAILURE: i32,
    }

    #[derive(Debug)]
    struct ArcasVideoEncoderLossNotification {
        timestamp_of_last_decodable: u32,
        timestamp_of_last_received: u32,
        // we can't use bool here in the vec so we send a u8 0 = false, 1 = true
        dependencies_of_last_received_decodable: Vec<u8>,
        last_received_decodable: Vec<u8>,
    }

    #[derive(Debug)]
    struct ArcasEncodedImageCallbackResult {
        error: ArcasCxxEncodedImageCallbackResultError,
        frame_id: u32,
        drop_next_frame: bool,
    }

    #[derive(Debug)]
    struct ArcasICECandidate {
        id: String,
        sdp_mid: String,
        sdp_mline_index: i32,
        sdp: String,
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
        type CxxVideoFrame;
        #[namespace = "webrtc"]
        #[cxx_name = "VideoFrameType"]
        type ArcasVideoFrameType;
        type ArcasCxxVideoEncoderRateControlParameters;
        type ArcasCxxVideoEncoderLossNotification;
        #[namespace = "webrtc"]
        #[cxx_name = "CodecSpecificInfo"]
        type ArcasCxxCodecSpecificInfo;

        type ArcasSDPType;
        #[namespace = "webrtc"]
        type RTCError;
        type ArcasRTPTransceiverDirection;

        type ArcasMediaType;
        type ArcasRTCSignalingState;
        type ArcasIceConnectionState;
        type ArcasPeerConnectionState;
        type ArcasIceGatheringState;
        type ArcasTlsCertPolicy;
        #[namespace = "webrtc"]
        #[cxx_name = "SdpSemantics"]
        type ArcasSDPSemantics;
        type ArcasVideoEncoderDropReason;

        // Our types

        // Should be left opaque in favor of the audio/video ones.
        type ArcasRTPSender;
        type ArcasRTPAudioSender;
        type ArcasRTPVideoSender;
        type ArcasVideoCodecSettings;
        type ArcasVideoCodec;
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
        type ArcasMediaStream;
        type ArcasDataChannel;
        type ArcasPeerConnectionFactory<'a>;
        type ArcasSessionDescription;
        type ArcasPeerConnection<'a>;
        type ArcasRTCConfiguration;
        type ArcasCreateSessionDescriptionObserver;
        type ArcasSetDescriptionObserver;
        type ArcasCxxEncodedImageCallbackResultError;
        type ArcasCxxVideoBitrateAllocation;
        type ArcasCxxVideoFrameBufferType;
        type ArcasCxxSdpVideoFormat;
        type ArcasCxxDataRate;
        type ArcasVideoEncoderFactory;
        type ArcasVideoEncoderRateControlParameters;
        type ArcasVideoTrack<'a>;
        type ArcasVideoTrackSource<'a>;
        type ArcasCxxEncodedImage;
        type ArcasCxxVideoCodecType;
        type ArcasCxxRefCountedEncodedImageBuffer;
        type ArcasVideoBitrateAllocation;
        type ArcasOpaqueEncodedImageBuffer;
        type ArcasEncodedImageFactory;
        type ArcasPeerConnectionObserver;
        type ArcasCodecSpecificInfo;
        type ArcasRTCStatsCollectorCallback;

        // wrapper functions around constructors.
        fn create_factory<'a>() -> UniquePtr<ArcasPeerConnectionFactory<'a>>;
        fn create_factory_with_arcas_video_encoder_factory<'a>(
            video_encoder_factory: UniquePtr<ArcasVideoEncoderFactory>,
        ) -> UniquePtr<ArcasPeerConnectionFactory<'a>>;
        fn create_arcas_video_track_source() -> SharedPtr<ArcasVideoTrackSource<'static>>;
        fn create_arcas_encoded_image_factory() -> UniquePtr<ArcasEncodedImageFactory>;
        fn create_arcas_codec_specific_info() -> UniquePtr<ArcasCodecSpecificInfo>;
        unsafe fn push_i420_to_video_track_source<'a>(
            source: SharedPtr<ArcasVideoTrackSource<'a>>,
            width: i32,
            height: i32,
            stride_y: i32,
            stride_u: i32,
            stride_v: i32,
            data: *mut u8,
        );
        fn get_arcas_video_encoding_err_codes() -> ArcasVideoEncodingErrCode;

        fn create_peer_connection_observer(
            observer: Box<PeerConnectionObserverProxy>,
        ) -> SharedPtr<ArcasPeerConnectionObserver>;

        // ArcasPeerConnectionFactory
        unsafe fn create_peer_connection<'a>(
            self: &ArcasPeerConnectionFactory<'a>,
            config: UniquePtr<ArcasRTCConfiguration>,
            observer: SharedPtr<ArcasPeerConnectionObserver>,
        ) -> UniquePtr<ArcasPeerConnection<'a>>;

        unsafe fn create_video_track<'a>(
            self: Pin<&mut ArcasPeerConnectionFactory<'a>>,
            id: String,
            source: SharedPtr<ArcasVideoTrackSource<'a>>,
        ) -> UniquePtr<ArcasVideoTrack<'a>>;

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

        fn add_audio_transceiver(self: &ArcasPeerConnection)
            -> UniquePtr<ArcasRTPAudioTransceiver>;

        fn add_video_track(
            self: &ArcasPeerConnection,
            track: UniquePtr<ArcasVideoTrack>,
            stream_ids: Vec<String>,
        );

        fn get_stats(self: &ArcasPeerConnection, callback: Box<ArcasRustRTCStatsCollectorCallback>);

        // session description
        fn to_string(self: &ArcasSessionDescription) -> String;
        fn get_type(self: &ArcasSessionDescription) -> ArcasSDPType;
        fn clone(self: &ArcasSessionDescription) -> UniquePtr<ArcasSessionDescription>;

        fn create_rtc_configuration(
            config: ArcasPeerConnectionConfig,
        ) -> UniquePtr<ArcasRTCConfiguration>;

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
        ) -> UniquePtr<ArcasVideoBitrateAllocation>;

        fn get_bitrate(
            self: &ArcasVideoEncoderRateControlParameters,
        ) -> UniquePtr<ArcasVideoBitrateAllocation>;

        fn get_framerate_fps(self: &ArcasVideoEncoderRateControlParameters) -> f64;
        fn get_bytes_per_second(self: &ArcasVideoEncoderRateControlParameters) -> i64;

        // ArcasEncodedImageFactory
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
            image: UniquePtr<ArcasCxxEncodedImage>,
            buffer: SharedPtr<ArcasOpaqueEncodedImageBuffer>,
        ) -> UniquePtr<ArcasCxxEncodedImage>;

        // ArcasEncodedImageCallback
        unsafe fn on_encoded_image(
            self: Pin<&mut ArcasEncodedImageCallback>,
            image: &ArcasCxxEncodedImage,
            info: *const ArcasCodecSpecificInfo,
        ) -> ArcasEncodedImageCallbackResult;

        // ArcasCxxEncodedImage
        #[cxx_name = "SetTimestamp"]
        fn set_timestamp(self: Pin<&mut ArcasCxxEncodedImage>, timestamp: u32);

        // ArcasCodecSpecificInfo
        unsafe fn set_codec_type(
            self: Pin<&mut ArcasCodecSpecificInfo>,
            codec_type: ArcasCxxVideoCodecType,
        );
        unsafe fn set_end_of_picture(
            self: Pin<&mut ArcasCodecSpecificInfo>,
            set_end_of_picture: bool,
        );
        fn get_codec_type(self: &ArcasCodecSpecificInfo) -> ArcasCxxVideoCodecType;

        unsafe fn push_i420_data(
            self: Pin<&mut ArcasVideoTrackSource>,
            width: i32,
            height: i32,
            stride_y: i32,
            stride_u: i32,
            stride_v: i32,
            data: *mut u8,
        );

        // XXX: Hacks to ensure CXX generates the unique ptr bindings for these return types.
        fn gen_unique_ptr1() -> UniquePtr<ArcasDataChannel>;
        fn gen_unique_ptr2() -> UniquePtr<ArcasMediaStream>;
        fn gen_unique_ptr3() -> UniquePtr<ArcasVideoCodecSettings>;
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
        fn on_ice_candidate(self: &PeerConnectionObserverProxy, candidate: ArcasICECandidate);
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

        fn on_track(
            self: &PeerConnectionObserverProxy,
            transceiver: UniquePtr<ArcasRTPTransceiver>,
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
        fn init_encode(
            self: &VideoEncoderProxy,
            codec_settings: UniquePtr<ArcasVideoCodec>,
            number_of_cores: i32,
            max_payload_size: usize,
        ) -> i32;

        fn register_encode_complete_callback(
            self: &VideoEncoderProxy,
            callback: UniquePtr<ArcasEncodedImageCallback>,
        ) -> i32;

        fn release(self: &VideoEncoderProxy) -> i32;

        unsafe fn encode(
            self: &VideoEncoderProxy,
            frame: &CxxVideoFrame,
            frame_types: *const CxxVector<ArcasVideoFrameType>,
        ) -> i32;

        fn set_rates(
            self: &VideoEncoderProxy,
            parameters: UniquePtr<ArcasVideoEncoderRateControlParameters>,
        );

        fn on_packet_loss_rate_update(self: &VideoEncoderProxy, packet_loss_rate: f32);

        fn on_rtt_update(self: &VideoEncoderProxy, rtt: i64);

        fn on_loss_notification(
            self: &VideoEncoderProxy,
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
    }
}

pub struct ArcasRustCreateSessionDescriptionObserver {
    success: Box<dyn Fn(UniquePtr<crate::ffi::ArcasSessionDescription>) -> ()>,
    failure: Box<dyn Fn(UniquePtr<crate::ffi::ArcasRTCError>) -> ()>,
}

impl ArcasRustCreateSessionDescriptionObserver {
    pub fn new(
        success: Box<dyn Fn(UniquePtr<crate::ffi::ArcasSessionDescription>) -> ()>,
        failure: Box<dyn Fn(UniquePtr<crate::ffi::ArcasRTCError>) -> ()>,
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
    success: Box<Fn() -> ()>,
    failure: Box<Fn(UniquePtr<crate::ffi::ArcasRTCError>) -> ()>,
}

impl ArcasRustSetSessionDescriptionObserver {
    pub fn new(
        success: Box<Fn() -> ()>,
        failure: Box<Fn(UniquePtr<crate::ffi::ArcasRTCError>) -> ()>,
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

type StatsCallbackFn = Fn(
    Vec<crate::ffi::ArcasVideoReceiverStats>,
    Vec<crate::ffi::ArcasAudioReceiverStats>,
    Vec<crate::ffi::ArcasVideoSenderStats>,
    Vec<crate::ffi::ArcasAudioSenderStats>,
) -> ();

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
        println!("RUST: stats delivered");
        (self.cb)(video_rx, audio_rx, video_tx, audio_tx)
    }
}
