#include "iostream"
#include "rust/cxx.h"
#include "libwebrtc-sys/src/lib.rs.h"
#include "libwebrtc-sys/include/internal_observer.h"

ArcasInternalPeerConnectionObserver::ArcasInternalPeerConnectionObserver(rust::Box<ArcasRustPeerConnectionObserver> observer) : observer(std::move(observer))
{
    std::cout << "Init Arcas Connection manager\n";
}

void ArcasInternalPeerConnectionObserver::OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState new_state)
{
    std::cout << "Signal change\n";
}

void ArcasInternalPeerConnectionObserver::OnAddStream(rtc::scoped_refptr<webrtc::MediaStreamInterface> stream)
{
    std::cout << "Add stream\n";
}

void ArcasInternalPeerConnectionObserver::OnRemoveStream(rtc::scoped_refptr<webrtc::MediaStreamInterface> stream)
{
    std::cout << "Remove stream\n";
}

void ArcasInternalPeerConnectionObserver::OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel)
{
    std::cout << "On data channel\n";
}

void ArcasInternalPeerConnectionObserver::OnRenegotiationNeeded()
{
    std::cout << "on renegotiation needed\n";
}

void ArcasInternalPeerConnectionObserver::OnNegotiationNeededEvent(uint32_t event_id)
{
    std::cout << "on renegotiation needed\n";
}

void ArcasInternalPeerConnectionObserver::OnIceConnectionChange(
    webrtc::PeerConnectionInterface::IceConnectionState new_state)
{
    std::cout << "on ice state change " << new_state << " \n";
}

void ArcasInternalPeerConnectionObserver::OnStandardizedIceConnectionChange(
    webrtc::PeerConnectionInterface::IceConnectionState new_state)
{
    std::cout << "on ice state change " << new_state << " \n";
}

void ArcasInternalPeerConnectionObserver::OnConnectionChange(
    webrtc::PeerConnectionInterface::PeerConnectionState new_state)
{
    std::cout << "connection change\n";
}

void ArcasInternalPeerConnectionObserver::OnIceGatheringChange(
    webrtc::PeerConnectionInterface::IceGatheringState new_state)
{
    std::cout << "gathering change\n";
};

void ArcasInternalPeerConnectionObserver::OnIceCandidate(const webrtc::IceCandidateInterface *candidate)
{
    std::cout << "on ice candidate";
};

void ArcasInternalPeerConnectionObserver::OnIceCandidateError(const std::string &host_candidate,
                                                              const std::string &url,
                                                              int error_code,
                                                              const std::string &error_text)
{
    std::cout << "on ice candidate error " << error_text;
}

// See https://w3c.github.io/webrtc-pc/#event-icecandidateerror
void ArcasInternalPeerConnectionObserver::OnIceCandidateError(const std::string &address,
                                                              int port,
                                                              const std::string &url,
                                                              int error_code,
                                                              const std::string &error_text)
{
    std::cout << "on ice candidate error " << error_text;
}

void ArcasInternalPeerConnectionObserver::OnIceCandidatesRemoved(
    const std::vector<cricket::Candidate> &candidates)
{
    std::cout << "on ice candidate removed\n";
}

void ArcasInternalPeerConnectionObserver::OnIceConnectionReceivingChange(bool receiving)
{
    std::cout << "on ice connection receiving change\n";
}

void ArcasInternalPeerConnectionObserver::OnIceSelectedCandidatePairChanged(
    const cricket::CandidatePairChangeEvent &event)
{
    std::cout << "ice selected candidate pair changed\n";
}

void ArcasInternalPeerConnectionObserver::OnAddTrack(
    rtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver,
    const std::vector<rtc::scoped_refptr<webrtc::MediaStreamInterface>> &streams)
{
    std::cout << "add track\n";
}

void ArcasInternalPeerConnectionObserver::OnTrack(
    rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver)
{
    std::cout << "on track\n";
}

void ArcasInternalPeerConnectionObserver::OnRemoveTrack(
    rtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver)
{
    std::cout << "on remove track\n";
}

void ArcasInternalPeerConnectionObserver::OnInterestingUsage(int usage_pattern)
{
    std::cout << "on interesting\n";
}