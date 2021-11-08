#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "rust/cxx.h"

class ArcasICECandidate
{
private:
    std::unique_ptr<webrtc::IceCandidateInterface> api;

public:
    ArcasICECandidate(std::unique_ptr<webrtc::IceCandidateInterface> api) : api(std::move(api)) {}

    rust::String id() const
    {
        return rust::String(api->candidate().id());
    }

    rust::String to_string() const
    {
        std::string out;
        api->ToString(&out);
        return rust::String(out.c_str());
    }

    rust::String sdp_mid() const
    {
        return rust::String(api->sdp_mid().c_str());
    }

    uint32_t sdp_mline_index() const
    {
        return api->sdp_mline_index();
    }

    // After this method is called the object *must not* be used.
    std::unique_ptr<webrtc::IceCandidateInterface> take()
    {
        return std::move(api);
    }
};

ArcasCreateICECandidateResult create_arcas_ice_candidate(
    rust::String sdp_mid,
    uint32_t sdp_mline_index,
    rust::String sdp);