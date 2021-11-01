#pragma once
#include "api/create_peerconnection_factory.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/rtp_sender.h"
#include "libwebrtc-sys/include/rtp_receiver.h"
#include "libwebrtc-sys/include/rtp_transceiver.h"
#include "libwebrtc-sys/include/video_track.h"
#include "libwebrtc-sys/include/session_description.h"
#include "libwebrtc-sys/include/peer_connection_observer.h"
#include "libwebrtc-sys/include/peer_connection_session_observers.h"
#include "libwebrtc-sys/include/rtp_transceiver.h"
#include "rust/cxx.h"

class ArcasPeerConnection
{
private:
    rtc::scoped_refptr<webrtc::PeerConnectionInterface> api;

public:
    ArcasPeerConnection(rtc::scoped_refptr<webrtc::PeerConnectionInterface> api);
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

    void add_video_track(std::unique_ptr<ArcasVideoTrack> track, rust::Vec<rust::String> rust_stream_ids) const
    {
        std::vector<std::string> stream_ids;

        for (auto item : rust_stream_ids)
        {
            stream_ids.push_back(std::string(item.c_str()));
        }

        api->AddTrack(track->ref(), stream_ids);
    }
};
