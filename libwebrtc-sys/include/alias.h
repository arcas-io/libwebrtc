#pragma once
#include "api/media_types.h"
#include "api/peer_connection_interface.h"
#include "api/video/video_frame.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/video_encoder_factory.h"

/**
 * @brief Naming Conventions
 *
 * This file exposes types within webrtc::* namespace (and it's sub namespaces) to the
 * top level with a prefix. This is helpful for binding ffi.
 *
 * ArcasCxx<> = A existing C++ wrapper we're exposing to Rust
 * Arcas<> = A new C++ wrapper we're exposing to Rust
 * Arcas<> = Also applies to shared types
 */

using ArcasRTCSignalingState = webrtc::PeerConnectionInterface::SignalingState;
using ArcasSDPSemantics = webrtc::SdpSemantics;
using ArcasIceGatheringState = webrtc::PeerConnectionInterface::IceGatheringState;
using ArcasPeerConnectionState = webrtc::PeerConnectionInterface::PeerConnectionState;
using ArcasIceConnectionState = webrtc::PeerConnectionInterface::IceConnectionState;
using ArcasTlsCertPolicy = webrtc::PeerConnectionInterface::TlsCertPolicy;
using ArcasICETransportType = webrtc::PeerConnectionInterface::IceTransportsType;
using ArcasCxxPeerConnectionObserver = webrtc::PeerConnectionObserver;
using ArcasMediaType = cricket::MediaType;
using ArcasSDPType = webrtc::SdpType;
using ArcasRTCConfiguration = webrtc::PeerConnectionInterface::RTCConfiguration;
using ArcasRTPTransceiverDirection = webrtc::RtpTransceiverDirection;
using ArcasVideoEncoderDropReason = webrtc::EncodedImageCallback::DropReason;
using ArcasCxxVideoEncoderRateControlParameters = webrtc::VideoEncoder::RateControlParameters;
using ArcasCxxVideoEncoderLossNotification = webrtc::VideoEncoder::LossNotification;
using ArcasCxxVideoFrameBufferType = webrtc::VideoFrameBuffer::Type;
using ArcasCxxVideoBitrateAllocation = webrtc::VideoBitrateAllocation;
using ArcasCxxEncodedImageCallbackResultError = webrtc::EncodedImageCallback::Result::Error;
using ArcasCxxSdpVideoFormat = webrtc::SdpVideoFormat;
using ArcasCxxDataRate = webrtc::DataRate;
using ArcasCxxVideoEncoderOptionalSelectorPointer =
    std::unique_ptr<webrtc::VideoEncoderFactory::EncoderSelectorInterface>;
using ArcasCxxEncodedImage = webrtc::EncodedImage;
using ArcasCxxRefCountedEncodedImageBuffer = rtc::scoped_refptr<webrtc::EncodedImageBuffer>;
using ArcasCxxVideoCodecType = webrtc::VideoCodecType;
using ArcasCxxRtpTransceiverDirection = webrtc::RtpTransceiverDirection;
using ArcasCxxVideoCodecMode = webrtc::VideoCodecMode;
using ArcasCxxCodecSpecificInfo = webrtc::CodecSpecificInfo;
using ArcasCxxVideoCodec = webrtc::VideoCodec;
using ArcasCxxVideoFrameType = webrtc::VideoFrameType;
using ArcasCxxVideoCodecComplexity = webrtc::VideoCodecComplexity;
using ArcasCxxInterLayerPredMode = webrtc::InterLayerPredMode;
using ArcasRTCErrorType = webrtc::RTCErrorType;
using ArcasCxxVideoEncoder = webrtc::VideoEncoder;
using ArcasCxxVideoEncoderSettings = webrtc::VideoEncoder::Settings;
using ArcasCxxVideoEncoderEncoderInfo = webrtc::VideoEncoder::EncoderInfo;
using ArcasCxxVideoFrame = webrtc::VideoFrame;

// using ArcasIceGatheringState = webrtc::PeerConnectionInterface::IceGatheringState;
// using ArcasIceGatheringState = webrtc::PeerConnectionInterface::IceGatheringState;
// using ArcasIceGatheringState = webrtc::PeerConnectionInterface::IceGatheringState;
std::unique_ptr<ArcasCxxSdpVideoFormat> gen_unique_cxx_video_format_wrapper();
std::unique_ptr<std::vector<ArcasCxxSdpVideoFormat>> gen_unique_sdp_video_format_vector();
std::shared_ptr<ArcasCxxEncodedImage> gen_shared_cxx_encoded_image();
std::unique_ptr<ArcasCxxEncodedImage> gen_unique_cxx_encoded_image();
std::unique_ptr<ArcasCxxVideoBitrateAllocation> gen_unique_cxx_video_bitrate_allocation();
std::unique_ptr<ArcasRTCConfiguration> gen_unique_cxx_rtc_configuration();
