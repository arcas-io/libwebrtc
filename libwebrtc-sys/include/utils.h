#pragma once
#include "libwebrtc-sys/include/peer_connection_factory.h"
#include "rust/cxx.h"

rust::String session_description_to_string(const webrtc::SessionDescriptionInterface &sdp);
