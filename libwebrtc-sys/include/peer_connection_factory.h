#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/peer_connection.h"
#include "libwebrtc-sys/include/video_track.h"
#include "libwebrtc-sys/include/audio_track.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "libwebrtc-sys/include/video_encoder_factory.h"
#include "libwebrtc-sys/include/video_track_source.h"
#include "libwebrtc-sys/include/audio_device_module.h"

class ArcasPeerConnectionFactory
{
private:
    rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> api;

public:
    ArcasPeerConnectionFactory(rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> api) : api(api){};

    ~ArcasPeerConnectionFactory()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasPeerConnectionFactory";
    }

    std::unique_ptr<ArcasPeerConnection> create_peer_connection(std::unique_ptr<webrtc::PeerConnectionInterface::RTCConfiguration> config, ArcasPeerConnectionObserver *observer) const
    {
        webrtc::PeerConnectionDependencies deps(observer);
        RTC_LOG(LS_VERBOSE) << "BEFOEW BEFORE DEREF ";
        RTC_LOG(LS_VERBOSE) << "BEFORE DEREF ";
        auto result = api->CreatePeerConnectionOrError(*config, std::move(deps));

        if (!result.ok())
        {
            return nullptr;
        }
        else
        {
            RTC_LOG(LS_ERROR) << result.error().message();
        }
        RTC_LOG(LS_VERBOSE) << "AFTER DEREF";
        auto out = std::make_unique<ArcasPeerConnection>(std::move(result.MoveValue()));
        return out;
    }
    std::unique_ptr<ArcasVideoTrack> create_video_track(rust::String id, ArcasVideoTrackSource &video_source) const
    {
        auto track = api->CreateVideoTrack(std::string(id.c_str()), video_source.ref());
        return std::make_unique<ArcasVideoTrack>(track);
    }
};

std::unique_ptr<webrtc::PeerConnectionInterface::RTCConfiguration> create_rtc_configuration(ArcasPeerConnectionConfig config);
std::unique_ptr<ArcasPeerConnectionObserver> create_peer_connection_observer(rust::Box<ArcasRustPeerConnectionObserver>);
