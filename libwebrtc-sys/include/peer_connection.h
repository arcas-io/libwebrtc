#pragma once
#include "api/create_peerconnection_factory.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/rtp_transceiver.h"
#include "libwebrtc-sys/include/session_description.h"
#include "libwebrtc-sys/include/peer_connection_observer.h"
#include "libwebrtc-sys/include/peer_connection_session_observers.h"
#include "libwebrtc-sys/include/rtp_transceiver.h"
#include "rust/cxx.h"

class ArcasPeerConnection
{
private:
    rtc::scoped_refptr<webrtc::PeerConnectionInterface> api;
    rtc::scoped_refptr<ArcasPeerConnectionObserver> observer;

public:
    ArcasPeerConnection(rtc::scoped_refptr<webrtc::PeerConnectionInterface> api, rtc::scoped_refptr<ArcasPeerConnectionObserver> observer);
    // NOTE: The object behind the shared_ptr must outlive the ArcasPeerConnection
    void create_offer(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const;
    // NOTE: The object behind the shared_ptr must outlive the ArcasPeerConnection
    void create_answer(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer) const;
    // NOTE: The object behind the shared_ptr must outlive the ArcasPeerConnection
    void set_local_description(rust::Box<ArcasRustSetSessionDescriptionObserver> observer, std::unique_ptr<ArcasSessionDescription> session) const;
    // NOTE: The object behind the shared_ptr must outlive the ArcasPeerConnection
    void set_remote_description(rust::Box<ArcasRustSetSessionDescriptionObserver> observer, std::unique_ptr<ArcasSessionDescription> session) const;
    std::unique_ptr<ArcasRTPTransceiver> add_simple_media_transceiver(cricket::MediaType media) const;
};
