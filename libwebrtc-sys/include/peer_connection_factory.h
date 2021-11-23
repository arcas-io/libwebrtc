#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/peer_connection.h"
#include "libwebrtc-sys/include/video_track.h"
#include "libwebrtc-sys/include/audio_track.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "libwebrtc-sys/include/video_encoder_factory.h"
#include "libwebrtc-sys/include/video_track_source.h"
#include "libwebrtc-sys/include/audio_device_module.h"

class ArcasAPIInternal;

class ArcasPeerConnectionFactory
{
private:
    // Held to ensure we don't need to keep references alive in rust.
    rtc::scoped_refptr<ArcasAPIInternal> internal_api;
    rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> api;

public:
    ArcasPeerConnectionFactory(
        rtc::scoped_refptr<ArcasAPIInternal> internal_api,
        rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> api);

    std::shared_ptr<ArcasPeerConnection> create_peer_connection(std::unique_ptr<webrtc::PeerConnectionInterface::RTCConfiguration> config, ArcasPeerConnectionObserver *observer) const
    {
        webrtc::PeerConnectionDependencies deps(observer);
        auto result = api->CreatePeerConnectionOrError(*config, std::move(deps));

        if (!result.ok())
        {
            RTC_LOG(LS_ERROR) << "Error creating peer connection: " << result.error().message();
            return nullptr;
        }
        auto out = std::make_shared<ArcasPeerConnection>(api, std::move(result.MoveValue()));
        return out;
    }
    std::unique_ptr<ArcasVideoTrack> create_video_track(rust::String id, const ArcasVideoTrackSource &video_source) const
    {
        auto track = api->CreateVideoTrack(std::string(id.c_str()), video_source.ref());
        return std::make_unique<ArcasVideoTrack>(track);
    }
};

std::unique_ptr<webrtc::PeerConnectionInterface::RTCConfiguration> create_rtc_configuration(ArcasPeerConnectionConfig config);
std::unique_ptr<ArcasPeerConnectionObserver> create_peer_connection_observer(rust::Box<ArcasRustPeerConnectionObserver>);
