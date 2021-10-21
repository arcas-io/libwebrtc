#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"

class ArcasMediaStream
{
private:
    rtc::scoped_refptr<webrtc::MediaStreamInterface> api;

public:
    ArcasMediaStream(rtc::scoped_refptr<webrtc::MediaStreamInterface> api);
};

std::unique_ptr<ArcasMediaStream> supbar();