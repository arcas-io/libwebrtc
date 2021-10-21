#include "iostream"
#include "rust/cxx.h"
#include "libwebrtc-sys/src/lib.rs.h"
#include "libwebrtc-sys/include/peer_connection_observer.h"

void ArcasPeerConnectionObserver::OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState new_state)
{
    observer->on_signaling_state_change(new_state);
};

void ArcasPeerConnectionObserver::OnAddStream(rtc::scoped_refptr<webrtc::MediaStreamInterface> stream)
{
    auto rust = std::make_unique<ArcasMediaStream>(stream);
    observer->on_add_stream(std::move(rust));
};

void ArcasPeerConnectionObserver::OnRemoveStream(rtc::scoped_refptr<webrtc::MediaStreamInterface> stream)
{

    auto rust = std::make_unique<ArcasMediaStream>(stream);
    observer->on_remove_stream(std::move(rust));
};

void ArcasPeerConnectionObserver::OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel)
{

    auto rust = std::make_unique<ArcasDataChannel>(data_channel);
    observer->on_datachannel(std::move(rust));
};

void ArcasPeerConnectionObserver::OnRenegotiationNeeded()
{
    observer->on_renegotiation_needed();
};

void ArcasPeerConnectionObserver::OnNegotiationNeededEvent(uint32_t event_id)
{
    observer->on_renegotiation_needed_event(event_id);
};

void ArcasPeerConnectionObserver::OnIceConnectionChange(
    webrtc::PeerConnectionInterface::IceConnectionState new_state){

    // XXX: Do we need this?
    // observer->on_ice_connection_change(new_state);
};

void ArcasPeerConnectionObserver::OnStandardizedIceConnectionChange(
    webrtc::PeerConnectionInterface::IceConnectionState new_state)
{

    observer->on_ice_connection_change(new_state);
};

void ArcasPeerConnectionObserver::OnConnectionChange(
    webrtc::PeerConnectionInterface::PeerConnectionState new_state)
{
    observer->on_connection_change(new_state);
};

void ArcasPeerConnectionObserver::OnIceGatheringChange(
    webrtc::PeerConnectionInterface::IceGatheringState new_state)
{

    observer->on_ice_gathering_change(new_state);
};

void ArcasPeerConnectionObserver::OnIceCandidate(const webrtc::IceCandidateInterface *candidate)
{
    ArcasICECandidate rust;
    rust.id = rust::String(candidate->candidate().id().c_str());
    rust.sdp_mid = rust::String(candidate->sdp_mid().c_str());
    rust.sdp_mline_index = candidate->sdp_mline_index();
    std::string sdp;
    candidate->ToString(&sdp);
    rust.sdp = rust::String(sdp.c_str());
    observer->on_ice_candidate(rust);
}

void ArcasPeerConnectionObserver::OnIceCandidateError(const std::string &host_candidate,
                                                      const std::string &url,
                                                      int error_code,
                                                      const std::string &error_text)
{
    observer->on_ice_candidate_error(rust::String(host_candidate.c_str()), rust::String(url.c_str()), error_code, rust::String(error_text.c_str()));
};

// See https://w2c.github.io/webrtc-pc/#event-icecandidateerror
void ArcasPeerConnectionObserver::OnIceCandidateError(const std::string &address,
                                                      int port,
                                                      const std::string &url,
                                                      int error_code,
                                                      const std::string &error_text)
{

    observer->on_ice_candidate_error_address_port(rust::String(address.c_str()), port, rust::String(url.c_str()), error_code, rust::String(error_text.c_str()));
};

void ArcasPeerConnectionObserver::OnIceCandidatesRemoved(
    const std::vector<cricket::Candidate> &candidates)
{
    rust::Vec<rust::String> list;

    for (auto candidate : candidates)
    {
        list.push_back(candidate.id().c_str());
    }

    observer->on_ice_candidates_removed(list);
};

void ArcasPeerConnectionObserver::OnIceConnectionReceivingChange(bool receiving)
{
    observer->on_ice_connection_receiving_change(receiving);
};

void ArcasPeerConnectionObserver::OnIceSelectedCandidatePairChanged(
    const cricket::CandidatePairChangeEvent &event)
{
    ArcasCandidatePairChangeEvent rust;
    rust.selected_remote_id = rust::String(event.selected_candidate_pair.remote.id().c_str());
    rust.selected_local_id = rust::String(event.selected_candidate_pair.local.id().c_str());
    rust.last_data_received_ms = event.last_data_received_ms;
    rust.reason = rust::String(event.reason.c_str());
    rust.estimated_disconnected_time_ms = event.estimated_disconnected_time_ms;
    observer->on_ice_selected_candidate_pair_change(rust);
};

void ArcasPeerConnectionObserver::OnAddTrack(
    rtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver,
    const std::vector<rtc::scoped_refptr<webrtc::MediaStreamInterface>> &streams)
{
    auto rust = std::make_unique<ArcasRTPReceiver>(receiver);
    observer->on_add_track(std::move(rust));
};

void ArcasPeerConnectionObserver::OnTrack(
    rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver)
{
    auto rust = std::make_unique<ArcasRTPTransceiver>(transceiver);
    observer->on_track(std::move(rust));
};

void ArcasPeerConnectionObserver::OnRemoveTrack(
    rtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver)
{
    auto rust = std::make_unique<ArcasRTPReceiver>(receiver);
    observer->on_remove_track(std::move(rust));
};

void ArcasPeerConnectionObserver::OnInterestingUsage(int usage_pattern)
{
    observer->on_interesting_usage(usage_pattern);
};
