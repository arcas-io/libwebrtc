#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/peer_connection_factory.h"

std::unique_ptr<ArcasPeerConnectionFactory> create_factory();
rust::String session_description_to_string(const webrtc::SessionDescriptionInterface &sdp);