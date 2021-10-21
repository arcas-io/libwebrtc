#pragma once
#include "rust/cxx.h"
#include "libwebrtc-sys/include/session_description.h"
#include "api/peer_connection_interface.h"
#include "libwebrtc-sys/include/rust_shared.h"

class ArcasCreateSessionDescriptionObserver : public webrtc::CreateSessionDescriptionObserver
{
private:
    rust::Box<ArcasRustCreateSessionDescriptionObserver> observer;

public:
    ArcasCreateSessionDescriptionObserver(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer);
    // XXX: This implementation requires this be held by a single ref
    void AddRef() const {}
    rtc::RefCountReleaseStatus Release() const
    {
        return rtc::RefCountReleaseStatus::kDroppedLastRef;
    }

    void OnSuccess(webrtc::SessionDescriptionInterface *desc);
    void OnFailure(webrtc::RTCError error);
};

class ArcasSetDescriptionObserver : public webrtc::SetLocalDescriptionObserverInterface, public webrtc::SetRemoteDescriptionObserverInterface
{
private:
    rust::Box<ArcasRustSetSessionDescriptionObserver> observer;

public:
    ArcasSetDescriptionObserver(rust::Box<ArcasRustSetSessionDescriptionObserver> observer);
    void OnSetLocalDescriptionComplete(webrtc::RTCError error);
    void OnSetRemoteDescriptionComplete(webrtc::RTCError error);
    // XXX: This implementation requires this be held by a single ref
    void AddRef() const {}
    rtc::RefCountReleaseStatus Release() const
    {
        return rtc::RefCountReleaseStatus::kDroppedLastRef;
    }
};