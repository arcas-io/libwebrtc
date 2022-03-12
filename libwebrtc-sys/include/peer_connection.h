#pragma once
#include "api/create_peerconnection_factory.h"
#include "libwebrtc-sys/include/audio_track.h"
#include "libwebrtc-sys/include/data_channel.h"
#include "libwebrtc-sys/include/ice_candidate.h"
#include "libwebrtc-sys/include/peer_connection_observer.h"
#include "libwebrtc-sys/include/peer_connection_session_observers.h"
#include "libwebrtc-sys/include/rtp_receiver.h"
#include "libwebrtc-sys/include/rtp_sender.h"
#include "libwebrtc-sys/include/rtp_transceiver.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "libwebrtc-sys/include/session_description.h"
#include "libwebrtc-sys/include/video_track.h"
#include "rust/cxx.h"
#include "rust_shared.h"
#include "session_description.h"
#include "video_track.h"

class ArcasDataChannelInit;
class ArcasPeerConnection
{
private:
    rtc::scoped_refptr<webrtc::PeerConnectionInterface> api;
    // Hold reference for refcounting purposes helps rust ensure we don't need exact ordering in drops.
    rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> factory;

public:
    ArcasPeerConnection(rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> factory, rtc::scoped_refptr<webrtc::PeerConnectionInterface> api)
    : api(api)
    , factory(factory){};
    ~ArcasPeerConnection()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasPeerConnection";
    }
    // NOTE: The object behind the shared_ptr must outlive the ArcasPeerConnection
    void create_offer(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const;
    // NOTE: The object behind the shared_ptr must outlive the ArcasPeerConnection
    void create_answer(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const;
    // NOTE: The object behind the shared_ptr must outlive the ArcasPeerConnection
    void set_local_description(rust::Box<ArcasRustSetSessionDescriptionObserver> observer, std::unique_ptr<ArcasSessionDescription> session) const;
    // NOTE: The object behind the shared_ptr must outlive the ArcasPeerConnection
    void set_remote_description(rust::Box<ArcasRustSetSessionDescriptionObserver> observer, std::unique_ptr<ArcasSessionDescription> session) const;

    std::unique_ptr<ArcasRTPVideoTransceiver> add_video_transceiver() const;
    std::unique_ptr<ArcasRTPAudioTransceiver> add_audio_transceiver() const;
    std::unique_ptr<ArcasRTPVideoTransceiver> add_video_transceiver_with_track(std::unique_ptr<ArcasVideoTrack> track,
                                                                               ArcasTransceiverInit init) const;
    std::unique_ptr<ArcasRTPAudioTransceiver> add_audio_transceiver_with_track(std::unique_ptr<ArcasAudioTrack> track,
                                                                               ArcasTransceiverInit init) const;
    void close() const
    {
        api->Close();
    }

    void add_video_track(std::unique_ptr<ArcasVideoTrack> track, rust::Vec<rust::String> rust_stream_ids) const
    {
        std::vector<std::string> stream_ids;

        for (auto item : rust_stream_ids) { stream_ids.push_back(std::string(item.c_str())); }

        auto ptr = track->ref();
        api->AddTrack(ptr, stream_ids);
    }

    void add_audio_track(std::unique_ptr<ArcasAudioTrack> track, rust::Vec<rust::String> rust_stream_ids) const
    {
        std::vector<std::string> stream_ids;

        for (auto item : rust_stream_ids) { stream_ids.push_back(item.c_str()); }
        api->AddTrack(track->ref(), stream_ids);
    }

    void get_stats(rust::Box<ArcasRustRTCStatsCollectorCallback> cb) const;
    void get_tranceiver_stats(rust::Box<ArcasRustRTCStatsCollectorCallback> cb, ArcasRTPTransceiver const& transceiver) const;
    void get_video_tranceiver_stats(rust::Box<ArcasRustRTCStatsCollectorCallback> cb, ArcasRTPVideoTransceiver const& transceiver) const;
    void get_audio_tranceiver_stats(rust::Box<ArcasRustRTCStatsCollectorCallback> cb, ArcasRTPAudioTransceiver const& transceiver) const;
    void get_tranceiver_stats_impl(rust::Box<ArcasRustRTCStatsCollectorCallback> cb, webrtc::RtpTransceiverInterface const& transceiver) const;
    void add_ice_candidate(std::unique_ptr<ArcasICECandidate> candidate) const;

    std::unique_ptr<ArcasDataChannel> create_data_channel(rust::String label, const ArcasDataChannelInit& init) const;

    std::unique_ptr<std::vector<ArcasRTPTransceiver>> get_transceivers() const;
};

std::shared_ptr<ArcasPeerConnection> gen_shared_peer_connection();
