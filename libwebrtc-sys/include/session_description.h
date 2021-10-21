#pragma once
#include "rust/cxx.h"
#include "libwebrtc-sys/include/webrtc_api.h"

class ArcasSessionDescription
{
private:
    std::unique_ptr<webrtc::SessionDescriptionInterface> api;

public:
    ArcasSessionDescription(std::unique_ptr<webrtc::SessionDescriptionInterface> api);

    rust::String to_string() const;
    webrtc::SdpType get_type() const;
    std::unique_ptr<ArcasSessionDescription> clone() const;
    std::unique_ptr<webrtc::SessionDescriptionInterface> clone_sdp() const;
};
