#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "rust/cxx.h"

class ArcasVideoTrack
{
private:
    rtc::scoped_refptr<webrtc::VideoTrackInterface> api;

public:
    ArcasVideoTrack(rtc::scoped_refptr<webrtc::VideoTrackInterface> api) : api(api){};
    ~ArcasVideoTrack()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasVideoTrack";
    }
    webrtc::VideoTrackInterface::ContentHint content_hint() const
    {
        return api->content_hint();
    }

    rtc::scoped_refptr<webrtc::VideoTrackInterface> ref() const
    {
        return api;
    }

    rust::String id() const
    {
        return rust::String(api->id().c_str());
    }
};
