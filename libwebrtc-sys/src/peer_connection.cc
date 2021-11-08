#include "iostream"
#include "rust/cxx.h"
#include "libwebrtc-sys/include/peer_connection_observer.h"
#include "libwebrtc-sys/include/peer_connection.h"
#include "libwebrtc-sys/include/peer_connection_stats_callback.h"
#include "libwebrtc-sys/src/lib.rs.h"
#include <iostream>

ArcasPeerConnection::ArcasPeerConnection(
    rtc::scoped_refptr<webrtc::PeerConnectionInterface> api) : api(std::move(api))
{
    RTC_LOG(LS_VERBOSE) << "ArcasPeerConnection";
}

void ArcasPeerConnection::create_offer(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const
{
    webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
    auto ref_counted = rtc::make_ref_counted<ArcasCreateSessionDescriptionObserver>(std::move(observer));
    api->CreateOffer(ref_counted, options);
}

void ArcasPeerConnection::create_answer(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const
{
    // TODO: This probably has to live longer elsewhere.
    webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
    auto ref_counted = rtc::make_ref_counted<ArcasCreateSessionDescriptionObserver>(std::move(observer));
    api->CreateAnswer(ref_counted, options);
}

void ArcasPeerConnection::set_local_description(rust::Box<ArcasRustSetSessionDescriptionObserver> observer, std::unique_ptr<ArcasSessionDescription> sdp) const
{
    auto ref_counted = rtc::make_ref_counted<ArcasSetDescriptionObserver>(std::move(observer));
    api->SetLocalDescription(std::move(sdp->clone_sdp()), ref_counted);
}

void ArcasPeerConnection::set_remote_description(rust::Box<ArcasRustSetSessionDescriptionObserver> observer, std::unique_ptr<ArcasSessionDescription> sdp) const
{
    auto ref_counted = rtc::make_ref_counted<ArcasSetDescriptionObserver>(std::move(observer));
    api->SetRemoteDescription(std::move(sdp->clone_sdp()), ref_counted);
}

std::unique_ptr<ArcasRTPVideoTransceiver> ArcasPeerConnection::add_video_transceiver() const
{
    auto result = api->AddTransceiver(cricket::MEDIA_TYPE_VIDEO);

    if (result.ok())
    {
        return std::make_unique<ArcasRTPVideoTransceiver>(result.MoveValue());
    }

    // TODO: Handle error cases.
    return nullptr;
}

std::unique_ptr<ArcasRTPVideoTransceiver> ArcasPeerConnection::add_video_transceiver_with_track(std::unique_ptr<ArcasVideoTrack> track, ArcasTransceiverInit init) const
{
    webrtc::RtpTransceiverInit transceiver_init;
    transceiver_init.direction = init.direction;

    for (auto stream_id : init.stream_ids)
    {
        transceiver_init.stream_ids.push_back(std::string(stream_id.c_str()));
    }

    RTC_LOG(LS_VERBOSE) << "ArcasPeerConnection::add_video_transceiver_with_track " << transceiver_init.stream_ids.size();
    auto result = api->AddTransceiver(track->ref(), transceiver_init);
    if (result.ok())
    {
        return std::make_unique<ArcasRTPVideoTransceiver>(result.MoveValue());
    }

    // TODO: Handle error cases.
    return nullptr;
}

std::unique_ptr<ArcasRTPAudioTransceiver> ArcasPeerConnection::add_audio_transceiver() const
{
    auto result = api->AddTransceiver(cricket::MEDIA_TYPE_AUDIO);

    if (result.ok())
    {
        return std::make_unique<ArcasRTPAudioTransceiver>(result.MoveValue());
    }

    // TODO: Handle error cases.
    return nullptr;
}

void ArcasPeerConnection::get_stats(rust::Box<ArcasRustRTCStatsCollectorCallback> cb) const
{
    auto cb_ = rtc::make_ref_counted<ArcasRTCStatsCollectorCallback>(std::move(cb));
    api->GetStats(cb_);
}

void ArcasPeerConnection::add_ice_candidate(std::unique_ptr<ArcasICECandidate> candidate) const
{
    auto value = candidate->take();
    api->AddIceCandidate(value.get());
}