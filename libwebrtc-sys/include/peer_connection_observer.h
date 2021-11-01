
#pragma once
#include "rust/cxx.h"
// #include "libwebrtc-sys/src/lib.rs.h"
#include "api/peer_connection_interface.h"
#include "libwebrtc-sys/include/rtp_receiver.h"
#include "libwebrtc-sys/include/rtp_transceiver.h"
#include "libwebrtc-sys/include/media_stream.h"
#include "libwebrtc-sys/include/data_channel.h"
#include "libwebrtc-sys/include/rust_shared.h"

class ArcasPeerConnectionObserver : public webrtc::PeerConnectionObserver, public rtc::RefCountedBase
{

private:
    rust::Box<ArcasRustPeerConnectionObserver> observer;

public:
    ArcasPeerConnectionObserver(rust::Box<ArcasRustPeerConnectionObserver> observer) : observer(std::move(observer))
    {
    }

    ~ArcasPeerConnectionObserver()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasPeerConnectionObserver";
    }

    void OnSignalingChange(webrtc::PeerConnectionInterface::SignalingState new_state);

    void OnAddStream(rtc::scoped_refptr<webrtc::MediaStreamInterface> stream);

    void OnRemoveStream(rtc::scoped_refptr<webrtc::MediaStreamInterface> stream);

    void OnDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel);

    void OnRenegotiationNeeded();

    void OnNegotiationNeededEvent(uint32_t event_id);

    void OnIceConnectionChange(
        webrtc::PeerConnectionInterface::IceConnectionState new_state);

    void OnStandardizedIceConnectionChange(
        webrtc::PeerConnectionInterface::IceConnectionState new_state);

    void OnConnectionChange(
        webrtc::PeerConnectionInterface::PeerConnectionState new_state);

    void OnIceGatheringChange(
        webrtc::PeerConnectionInterface::IceGatheringState new_state);

    void OnIceCandidate(const webrtc::IceCandidateInterface *candidate);

    void OnIceCandidateError(const std::string &host_candidate,
                             const std::string &url,
                             int error_code,
                             const std::string &error_text);

    // See https://w2c.github.io/webrtc-pc/#event-icecandidateerror
    void OnIceCandidateError(const std::string &address,
                             int port,
                             const std::string &url,
                             int error_code,
                             const std::string &error_text);

    void OnIceCandidatesRemoved(
        const std::vector<cricket::Candidate> &candidates);

    void OnIceConnectionReceivingChange(bool receiving);

    void OnIceSelectedCandidatePairChanged(
        const cricket::CandidatePairChangeEvent &event);

    void OnAddTrack(
        rtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver,
        const std::vector<rtc::scoped_refptr<webrtc::MediaStreamInterface>> &streams);

    void OnTrack(
        rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver);
    void OnRemoveTrack(
        rtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver);
    void OnInterestingUsage(int usage_pattern);
};
