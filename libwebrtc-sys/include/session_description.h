#pragma once
#include "rust/cxx.h"
#include "api/jsep_session_description.h"

struct ArcasCreateSessionDescriptionResult;

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

ArcasCreateSessionDescriptionResult create_arcas_session_description(webrtc::SdpType type, rust::String sdp);
