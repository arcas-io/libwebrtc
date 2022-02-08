#include "libwebrtc-sys/include/peer_connection.h"
#include "libwebrtc-sys/include/peer_connection_observer.h"
#include "libwebrtc-sys/include/peer_connection_stats_callback.h"
#include "libwebrtc-sys/src/data_channel.rs.h"
#include "libwebrtc-sys/src/peer_connection.rs.h"
#include "libwebrtc-sys/src/shared_bridge.rs.h"
#include "rust/cxx.h"
#include <iostream>
#include <vector>

void ArcasPeerConnection::create_offer(
    rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const
{
    webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
    auto ref_counted =
        rtc::make_ref_counted<ArcasCreateSessionDescriptionObserver>(std::move(observer));
    api->CreateOffer(ref_counted, options);
}

void ArcasPeerConnection::create_answer(
    rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const
{
    // TODO: This probably has to live longer elsewhere.
    webrtc::PeerConnectionInterface::RTCOfferAnswerOptions options;
    auto ref_counted =
        rtc::make_ref_counted<ArcasCreateSessionDescriptionObserver>(std::move(observer));
    api->CreateAnswer(ref_counted, options);
}

void ArcasPeerConnection::set_local_description(
    rust::Box<ArcasRustSetSessionDescriptionObserver> observer,
    std::unique_ptr<ArcasSessionDescription> sdp) const
{
    auto ref_counted = rtc::make_ref_counted<ArcasSetDescriptionObserver>(std::move(observer));
    api->SetLocalDescription(std::move(sdp->clone_sdp()), ref_counted);
}

void ArcasPeerConnection::set_remote_description(
    rust::Box<ArcasRustSetSessionDescriptionObserver> observer,
    std::unique_ptr<ArcasSessionDescription> sdp) const
{
    auto ref_counted = rtc::make_ref_counted<ArcasSetDescriptionObserver>(std::move(observer));
    api->SetRemoteDescription(std::move(sdp->clone_sdp()), ref_counted);
}

std::unique_ptr<ArcasRTPVideoTransceiver> ArcasPeerConnection::add_video_transceiver() const
{
    auto result = api->AddTransceiver(cricket::MEDIA_TYPE_VIDEO);

    if (result.ok())
    {
        return std::make_unique<ArcasRTPVideoTransceiver>(*api, result.MoveValue());
    }

    // TODO: Handle error cases.
    return nullptr;
}

std::unique_ptr<ArcasRTPVideoTransceiver>
ArcasPeerConnection::add_video_transceiver_with_track(std::unique_ptr<ArcasVideoTrack> track,
                                                      ArcasTransceiverInit init) const
{
    webrtc::RtpTransceiverInit transceiver_init;
    transceiver_init.direction = init.direction;

    for (auto stream_id : init.stream_ids)
    {
        transceiver_init.stream_ids.push_back(std::string(stream_id.c_str()));
    }

    RTC_LOG(LS_VERBOSE) << "ArcasPeerConnection::add_video_transceiver_with_track "
                        << transceiver_init.stream_ids.size();
    auto result = api->AddTransceiver(track->ref(), transceiver_init);
    if (result.ok())
    {
        return std::make_unique<ArcasRTPVideoTransceiver>(*api, result.MoveValue());
    }

    // TODO: Handle error cases.
    return nullptr;
}

std::unique_ptr<ArcasRTPAudioTransceiver>
ArcasPeerConnection::add_audio_transceiver_with_track(std::unique_ptr<ArcasAudioTrack> track,
                                                      ArcasTransceiverInit init) const
{
    webrtc::RtpTransceiverInit transceiver_init;
    transceiver_init.direction = init.direction;

    for (auto stream_id : init.stream_ids)
    {
        transceiver_init.stream_ids.push_back(std::string(stream_id.c_str()));
    }

    RTC_LOG(LS_VERBOSE) << "ArcasPeerConnection::add_video_transceiver_with_track "
                        << transceiver_init.stream_ids.size();
    auto result = api->AddTransceiver(track->ref(), transceiver_init);
    if (result.ok())
    {
        return std::make_unique<ArcasRTPAudioTransceiver>(*api, result.MoveValue());
    }

    // TODO: Handle error cases.
    return nullptr;
}


std::unique_ptr<ArcasRTPAudioTransceiver> ArcasPeerConnection::add_audio_transceiver() const
{
    auto result = api->AddTransceiver(cricket::MEDIA_TYPE_AUDIO);

    if (result.ok())
    {
        return std::make_unique<ArcasRTPAudioTransceiver>(*api, result.MoveValue());
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
    std::string sdp_out;
    webrtc::SdpParseError error;
    // XXX: Do *not* make further calls into candidate after this.
    auto jsep = candidate->take();
    jsep->ToString(&sdp_out);
    auto add_candidate = std::unique_ptr<webrtc::IceCandidateInterface>(
        webrtc::CreateIceCandidate(jsep->sdp_mid(), jsep->sdp_mline_index(), sdp_out, &error));

    api->AddIceCandidate(std::move(add_candidate),
                         [](webrtc::RTCError err)
                         {
                             if (!err.ok())
                             {
                                 RTC_LOG(LS_ERROR)
                                     << "ArcasPeerConnection::add_ice_candidate error: "
                                     << err.message();
                             }
                         });
}

std::unique_ptr<std::vector<ArcasRTPTransceiver>> ArcasPeerConnection::get_transceivers() const
{
    auto transceivers = api->GetTransceivers();
    std::vector<ArcasRTPTransceiver> result;
    for (auto txrx : transceivers) { result.emplace_back(*api, txrx); }
    return std::make_unique<std::vector<ArcasRTPTransceiver>>(std::move(result));
}

std::unique_ptr<ArcasDataChannel>
ArcasPeerConnection::create_data_channel(rust::String label,
                                         const ArcasDataChannelInit& init_rust) const
{
    webrtc::DataChannelInit init;
    const std::string& cxx_label = label.c_str();

    init.reliable = init_rust.reliable;
    init.ordered = init_rust.ordered;

    if (init_rust.max_retransmit_time.size() > 0)
    {
        init.maxRetransmitTime = init_rust.max_retransmit_time[0];
    }

    if (init_rust.max_retransmits.size() > 0)
    {
        init.maxRetransmits = init_rust.max_retransmits[0];
    }

    init.protocol.assign(init_rust.protocol.data(), init_rust.protocol.size());

    if (init_rust.id.size() > 0)
    {
        init.id = init_rust.id[0];
    }

    if (init_rust.priority.size() > 0)
    {
        init.priority = init_rust.priority[0];
    }

    return std::make_unique<ArcasDataChannel>(api->CreateDataChannel(cxx_label, &init));
}
