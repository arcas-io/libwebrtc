use ffi::ArcasCxxVideoFrame;

use crate::api::ffi::ArcasAPI;
use crate::codec_specific_info::ffi::{ArcasCodecSpecificInfo, ArcasCxxCodecSpecificInfo};
use crate::encoded_image_factory::ffi::ArcasEncodedImageFactory;
use crate::ice_candidate::ffi::ArcasICECandidate;
use crate::peer_connection::ffi::{
    ArcasPeerConnection, ArcasRTCConfiguration, ArcasSessionDescription,
};
use crate::peer_connection_factory::ffi::ArcasPeerConnectionFactory;
use crate::peer_connection_observer::ffi::ArcasPeerConnectionObserver;
use crate::rtp_transceiver::ffi::ArcasRTPVideoTransceiver;
use crate::shared_bridge::ffi::ArcasCxxEncodedImage;
use crate::video_codec::ffi::{ArcasVideoCodec, ArcasVideoFrameEncodedImageData};
use crate::video_encoding::ffi::{
    ArcasEncodedImageCallback, ArcasVideoEncoderRateControlParameters, ArcasVideoEncoderSettings,
};
pub use crate::video_encoding::{
    VIDEO_CODEC_ENCODER_FAILURE, VIDEO_CODEC_ERROR, VIDEO_CODEC_ERR_PARAMETER,
    VIDEO_CODEC_ERR_SIMULCAST_PARAMETERS_NOT_SUPPORTED, VIDEO_CODEC_FALLBACK_SOFTWARE,
    VIDEO_CODEC_MEMORY, VIDEO_CODEC_NO_OUTPUT, VIDEO_CODEC_OK, VIDEO_CODEC_OK_REQUEST_KEYFRAME,
    VIDEO_CODEC_TARGET_BITRATE_OVERSHOOT, VIDEO_CODEC_UNINITIALIZED,
};
use crate::video_frame::ffi::{ArcasColorSpace, ArcasVideoFrameRawImageData};
use crate::video_track::ffi::ArcasVideoTrack;
use crate::video_track_source::ffi::ArcasVideoTrackSource;

pub mod api;
pub mod codec_specific_info;
pub mod data_channel;
pub mod encoded_image_factory;
pub mod error;
pub mod ice_candidate;
pub mod into;
pub mod logging;
pub mod media_stream;
pub mod peer_connection;
pub mod peer_connection_factory;
pub mod peer_connection_observer;
pub mod peerconnection_factory_config;
pub mod rtp_parameters;
pub mod rtp_receiver;
pub mod rtp_sender;
pub mod rtp_transceiver;
pub mod sdp_video_format;
pub mod session_description;
pub mod shared_bridge;
pub mod spatial_layer;
pub mod video_codec;
pub mod video_decoding;
pub mod video_encoder_factory_wrapper;
pub mod video_encoding;
pub mod video_encoding_wrapper;
pub mod video_frame;
pub mod video_frame_buffer;
pub mod video_frame_buffer_encoded;
pub mod video_track;
pub mod video_track_source;

pub use crate::peer_connection::{
    ArcasRustCreateSessionDescriptionObserver, ArcasRustSetSessionDescriptionObserver, *,
};
pub use crate::video_decoding::{VideoDecoderFactoryProxy, VideoDecoderProxy};
pub use crate::video_encoding::{
    VideoEncoderFactoryProxy, VideoEncoderProxy, VideoEncoderSelectorProxy,
};

pub use crate::video_encoder_factory_wrapper::EncodedImageCallbackHandler;

pub mod ffi {
    pub use crate::api::ffi::{
        ArcasAPI, ArcasPeerConnectionFactoryConfig, ArcasVideoEncoderFactory, *,
    };
    pub use crate::codec_specific_info::ffi::{ArcasCodecSpecificInfo, *};
    pub use crate::encoded_image_factory::ffi::{ArcasEncodedImageFactory, *};
    pub use crate::error::ffi::{ArcasRTCError, ArcasRTCErrorType, RTCError, *};
    pub use crate::ice_candidate::ffi::{ArcasICECandidate, *};
    pub use crate::logging::ffi::{set_arcas_log_level, set_arcas_log_to_stderr, LoggingSeverity};
    pub use crate::media_stream::ffi::{ArcasMediaStream, *};
    pub use crate::peer_connection::ffi::{ArcasPeerConnection, ArcasSessionDescription, *};
    pub use crate::peer_connection_factory::ffi::{
        create_rtc_configuration, ArcasICEServer, ArcasPeerConnectionFactory,
        ArcasRTCConfiguration, *,
    };
    pub use crate::peer_connection_observer::ffi::{
        create_peer_connection_observer, ArcasCandidatePairChangeEvent,
        ArcasPeerConnectionObserver, *,
    };
    pub use crate::peerconnection_factory_config::ffi::create_arcas_peerconnection_factory_config;
    pub use crate::rtp_parameters::ffi::{
        ArcasRTPCodecCapability, ArcasRTPHeaderExtensionCapability,
    };
    pub use crate::rtp_receiver::ffi::{
        ArcasRTPAudioReceiver, ArcasRTPReceiver, ArcasRTPVideoReceiver,
    };
    pub use crate::rtp_sender::ffi::{ArcasRTPAudioSender, ArcasRTPSender, ArcasRTPVideoSender};
    pub use crate::rtp_transceiver::ffi::{
        ArcasRTPAudioTransceiver, ArcasRTPTransceiver, ArcasRTPVideoTransceiver, *,
    };
    pub use crate::sdp_video_format::ffi::{
        create_sdp_video_format, create_sdp_video_format_list, sdp_video_format_get_name,
        sdp_video_format_to_string, video_format_get_parameters,
    };
    pub use crate::session_description::ffi::{create_arcas_session_description, *};
    pub use crate::video_decoding::ffi::{ArcasVideoDecoder, *};
    pub use crate::video_encoding::ffi::{
        create_arcas_video_encoder_factory, ArcasVideoEncoderFactoryCodecInfo,
        ArcasVideoEncoderFactoryCodecSupport, ArcasVideoEncodingErrCode, *,
    };
    pub use crate::video_encoding::ffi::{
        ArcasCxxVideoFrameBufferType, ArcasVideoEncoder, ArcasVideoEncoderDropReason,
        ArcasVideoEncoderInfo, ArcasVideoEncoderLossNotification,
        ArcasVideoEncoderRateControlParameters, ArcasVideoEncoderSettings, *,
    };
    pub use crate::video_encoding_wrapper::ffi::ArcasVideoEncoderWrapper;
    pub use crate::video_encoding_wrapper::ffi::*;
    pub use crate::video_frame::ffi::{
        ArcasColorSpace, ArcasCxxVideoFrame, ArcasCxxVideoFrameType, ArcasVideoFrameRawImageData, *,
    };
    pub use crate::video_frame_buffer::ffi::{ArcasVideoFrameBufferEmpty, *};
    pub use crate::video_frame_buffer_encoded::ffi::{
        create_arcas_video_frame_buffer_from_I420,
        create_arcas_video_frame_buffer_from_encoded_image, *,
    };
    pub use crate::video_track::ffi::{ArcasVideoTrack, *};
    pub use crate::video_track_source::ffi::*;
    pub use crate::video_track_source::ffi::{ArcasVideoTrackSource, *};
    pub use crate::{
        shared_bridge::ffi::{
            ArcasCxxEncodedImage, ArcasCxxInterLayerPredMode, ArcasCxxRtpTransceiverDirection,
            ArcasCxxSdpVideoFormat, ArcasCxxVideoCodec, ArcasCxxVideoCodecType,
            ArcasIceConnectionState, ArcasIceGatheringState, ArcasMediaType,
            ArcasPeerConnectionState, ArcasRTCSignalingState, ArcasRTPTransceiverDirection,
            ArcasSDPSemantics, ArcasSDPType, ArcasSdpVideoFormatInit, ArcasSdpVideoFormatVecInit,
            *,
        },
        spatial_layer::ffi::{ArcasSpatialLayer, *},
    };
    pub use crate::{
        video_codec::ffi::{ArcasVideoCodec, ArcasVideoFrameEncodedImageData, *},
        video_decoding::ffi::{
            create_arcas_video_decoder_factory, ArcasVideoDecoderFactory,
            ArcasVideoDecoderFactoryCodecSupport, *,
        },
    };
    pub use crate::{
        video_decoding::ffi::ArcasDecodedImageCallback,
        video_encoder_factory_wrapper::ffi::{ArcasSDPVideoFormatWrapper, *},
    };
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
 *      - the fix is to pub pub pub use crate::something like this: `rust::String(value.c_str())`
 *
 *  - UniquePtr represent objects which are generally safe to send across threads (generally but not always).
 *  - SharedPtr are good for immutable objects (const functions) which are safe to send & sync across threads.
 */

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
