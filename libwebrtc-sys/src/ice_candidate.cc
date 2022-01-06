#include "libwebrtc-sys/include/ice_candidate.h"
#include "libwebrtc-sys/src/ice_candidate.rs.h"

ArcasCreateICECandidateResult
create_arcas_ice_candidate(rust::String sdp_mid, uint32_t sdp_mline_index, rust::String sdp)
{
    webrtc::SdpParseError         error;
    ArcasCreateICECandidateResult result;

    auto candidate =
        webrtc::CreateIceCandidate(sdp_mid.c_str(), sdp_mline_index, sdp.c_str(), &error);
    auto api = webrtc::CreateIceCandidate(candidate->sdp_mid(),
                                          candidate->sdp_mline_index(),
                                          candidate->candidate());

    if (error.line.size() > 0)
    {
        result.ok = false;
        result.error.line = rust::String(error.line.c_str());
        result.error.description = rust::String(error.description.c_str());
        return result;
    }
    else
    {
        result.ok = true;
        result.candidate = std::make_unique<ArcasICECandidate>(std::move(api));
        return result;
    }
}
