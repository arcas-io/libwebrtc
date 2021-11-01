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
    class impl;
    std::shared_ptr<impl> api;

public:
    ArcasPeerConnectionFactory(
        webrtc::PeerConnectionFactoryInterface *factory,
        std::unique_ptr<rtc::Thread> signal_thread,
        std::unique_ptr<rtc::Thread> worker_thread,
        std::unique_ptr<rtc::Thread> network_thread,
        rtc::scoped_refptr<webrtc::AudioDeviceModule> adm);

    std::unique_ptr<ArcasPeerConnection> create_peer_connection(std::unique_ptr<webrtc::PeerConnectionInterface::RTCConfiguration> config, std::shared_ptr<ArcasPeerConnectionObserver> observer) const;
    std::unique_ptr<ArcasVideoTrack> create_video_track(rust::String id, std::shared_ptr<ArcasVideoTrackSource>);
};

std::unique_ptr<ArcasPeerConnectionFactory> create_factory();
std::unique_ptr<ArcasPeerConnectionFactory> create_factory_with_arcas_video_encoder_factory(std::unique_ptr<ArcasVideoEncoderFactory> video_encoder_factory);
std::unique_ptr<webrtc::PeerConnectionInterface::RTCConfiguration> create_rtc_configuration(ArcasPeerConnectionConfig config);
std::shared_ptr<ArcasPeerConnectionObserver> create_peer_connection_observer(rust::Box<ArcasRustPeerConnectionObserver>);