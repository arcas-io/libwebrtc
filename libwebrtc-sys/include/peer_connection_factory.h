#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/peer_connection.h"
#include "libwebrtc-sys/include/internal_observer.h"

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

    std::unique_ptr<ArcasPeerConnection> create_peer_connection(ArcasRTCPeerConnectionConfig config, rust::Box<ArcasRustPeerConnectionObserver>) const;
};