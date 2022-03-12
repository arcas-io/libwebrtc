#[allow(unreachable_patterns)]
#[cxx::bridge]
pub mod ffi {
    #[derive(Debug)]
    struct ArcasRustDict {
        key: String,
        value: String,
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
    #[repr(u32)]
    enum ArcasCxxEncodedImageCallbackResultError {
        OK,
        ERROR_SEND_FAILED,
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
    struct ArcasEncodedImageCallbackResult {
        error: ArcasCxxEncodedImageCallbackResultError,
        frame_id: u32,
        drop_next_frame: bool,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasCxxInterLayerPredMode {
        kOff = 0,
        // Inter-layer prediction is disabled.
        kOn = 1,
        // Inter-layer prediction is enabled.
        kOnKeyPic = 2, // Inter-layer prediction is enabled but limited to key frames.
    }

    unsafe extern "C++" {
        include!("libwebrtc-sys/include/alias.h");
        include!("libwebrtc-sys/include/session_description.h");
        include!("libwebrtc-sys/include/ice_candidate.h");

        #[namespace = "webrtc"]
        type VideoRotation;
        type ArcasVideoEncoderDropReason;
        type ArcasMediaType;
        type ArcasSDPType;
        type ArcasRTCSignalingState;
        type ArcasCxxVideoCodecMode;
        type ArcasCxxVideoCodecComplexity;
        type ArcasCxxVideoCodecType;
        #[namespace = "rtc"]
        type LoggingSeverity;
        type ArcasRTCErrorType;
        type ArcasIceGatheringState;
        type ArcasPeerConnectionState;
        type ArcasRTPTransceiverDirection;
        type ArcasIceConnectionState;
        type ArcasTlsCertPolicy;
        type ArcasSDPSemantics;
        type ArcasCxxEncodedImageCallbackResultError;
        type ArcasCxxRtpTransceiverDirection;
        type ArcasCxxInterLayerPredMode;
        // TODO: Link
        type ArcasSessionDescription;
        // TODO: Link
        type ArcasICECandidate;
        type ArcasRTCConfiguration;
        type ArcasCxxSdpVideoFormat;
        type ArcasCxxEncodedImage;
        type ArcasCxxCodecSpecificInfo;
        type ArcasCxxVideoCodec;
        type ArcasCxxDataRate;
        type ArcasCxxVideoBitrateAllocation;
        type ArcasCxxVideoEncoderRateControlParameters;

        fn gen_unique_cxx_video_format_wrapper() -> UniquePtr<ArcasCxxSdpVideoFormat>;
        fn gen_unique_sdp_video_format_vector() -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;
        fn gen_shared_cxx_encoded_image() -> SharedPtr<ArcasCxxEncodedImage>;
        fn gen_unique_cxx_encoded_image() -> UniquePtr<ArcasCxxEncodedImage>;
        fn gen_unique_cxx_video_bitrate_allocation() -> UniquePtr<ArcasCxxVideoBitrateAllocation>;
        fn gen_unique_cxx_rtc_configuration() -> UniquePtr<ArcasRTCConfiguration>;
    }
}
