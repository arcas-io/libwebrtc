#pragma once
#include "api/jsep_session_description.h"
#include "pc/session_description.h"
#include "rust/cxx.h"

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
    const cricket::SessionDescription* jsep_session_description() const
    {
        return api->description();
    }
};

ArcasCreateSessionDescriptionResult create_arcas_session_description(webrtc::SdpType type,
                                                                     rust::String sdp);
