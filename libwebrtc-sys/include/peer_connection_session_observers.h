#pragma once
#include "api/peer_connection_interface.h"
#include "rust/cxx.h"
#include "rust_shared.h"
#include "session_description.h"

class ArcasCreateSessionDescriptionObserver : public webrtc::CreateSessionDescriptionObserver, public rtc::RefCountedBase
{
private:
    rust::Box<ArcasRustCreateSessionDescriptionObserver> observer;

public:
    ArcasCreateSessionDescriptionObserver(rust::Box<ArcasRustCreateSessionDescriptionObserver> observer);

    ~ArcasCreateSessionDescriptionObserver()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasCreateSessionDescriptionObserver";
    }

    void AddRef() const override
    {
        rtc::RefCountedBase::AddRef();
    }
    rtc::RefCountReleaseStatus Release() const override
    {
        return rtc::RefCountedBase::Release();
    }

    void OnSuccess(webrtc::SessionDescriptionInterface* desc) override;
    void OnFailure(webrtc::RTCError error) override;
};

class ArcasSetDescriptionObserver : public webrtc::SetLocalDescriptionObserverInterface,
                                    public webrtc::SetRemoteDescriptionObserverInterface,
                                    public rtc::RefCountedBase
{
private:
    rust::Box<ArcasRustSetSessionDescriptionObserver> observer;

public:
    ArcasSetDescriptionObserver(rust::Box<ArcasRustSetSessionDescriptionObserver> observer);
    ~ArcasSetDescriptionObserver()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasSetDescriptionObserver";
    }
    void OnSetLocalDescriptionComplete(webrtc::RTCError error) override;
    void OnSetRemoteDescriptionComplete(webrtc::RTCError error) override;
    // XXX: This implementation requires this be held by a single ref
    void AddRef() const override
    {
        rtc::RefCountedBase::AddRef();
    }
    rtc::RefCountReleaseStatus Release() const override
    {
        return rtc::RefCountedBase::Release();
    }
};