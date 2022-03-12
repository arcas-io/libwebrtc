#include <candidate.h>
#include <libwebrtc-sys/src/candidate.rs.h>

#include <rtc_base/checks.h>

int to_int(CandidateComponent cc)
{
    switch (cc)
    {
    case CandidateComponent::Default: return cricket::ICE_CANDIDATE_COMPONENT_DEFAULT;
    case CandidateComponent::Rtp: return cricket::ICE_CANDIDATE_COMPONENT_RTP;
    case CandidateComponent::Rtcp: return cricket::ICE_CANDIDATE_COMPONENT_RTCP;
    default: RTC_DCHECK(false); return cricket::ICE_CANDIDATE_COMPONENT_DEFAULT;
    }
}

void ArcasCandidate::set_component(CandidateComponent val)
{
    _candidate.set_component(to_int(val));
}
