#pragma once
#include "api/peer_connection_interface.h"

using ArcasRTCSignalingState = webrtc::PeerConnectionInterface::SignalingState;
using ArcasSDPSemantics = webrtc::SdpSemantics;
using ArcasIceGatheringState = webrtc::PeerConnectionInterface::IceGatheringState;
using ArcasPeerConnectionState = webrtc::PeerConnectionInterface::PeerConnectionState;
using ArcasIceConnectionState = webrtc::PeerConnectionInterface::IceConnectionState;
using ArcasTlsCertPolicy = webrtc::PeerConnectionInterface::TlsCertPolicy;
using ArcasICETransportType = webrtc::PeerConnectionInterface::IceTransportsType;
using ArcasCxxPeerConnectionObserver = webrtc::PeerConnectionObserver;
// using ArcasIceGatheringState = webrtc::PeerConnectionInterface::IceGatheringState;
// using ArcasIceGatheringState = webrtc::PeerConnectionInterface::IceGatheringState;
// using ArcasIceGatheringState = webrtc::PeerConnectionInterface::IceGatheringState;