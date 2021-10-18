#pragma once
#include "api/create_peerconnection_factory.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/internal_observer.h"

class ArcasCreateSessionDescriptionObserver : public webrtc::CreateSessionDescriptionObserver
{
private:
    rust::Box<ArcasRustCreateSessionDescriptionObserver> observer;

public:
    ArcasCreateSessionDescriptionObserver(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer);
    void OnSuccess(webrtc::SessionDescriptionInterface *desc);
    void OnFailure(webrtc::RTCError error);
};

class ArcasPeerConnection
{
private:
    rtc::scoped_refptr<webrtc::PeerConnectionInterface> api;

public:
    ArcasPeerConnection(rtc::scoped_refptr<webrtc::PeerConnectionInterface> api);
    void create_offer(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const;
};
