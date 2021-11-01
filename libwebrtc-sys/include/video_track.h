#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"

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
    webrtc::VideoTrackInterface::ContentHint content_hint()
    {
        return api->content_hint();
    }

    rtc::scoped_refptr<webrtc::VideoTrackInterface> ref()
    {
        return api;
    }
};