#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"

class ArcasMediaStreamTrack
{

    rtc::scoped_refptr<MediaStreamTrackInterface> ref()
    {
        return api;
    }

private:
    rtc::scoped_refptr<MediaStreamTrackInterface> api;
};