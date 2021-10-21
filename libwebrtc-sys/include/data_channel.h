#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"

class ArcasDataChannel
{
private:
    rtc::scoped_refptr<webrtc::DataChannelInterface> api;

public:
    ArcasDataChannel(rtc::scoped_refptr<webrtc::DataChannelInterface> api);
};

std::unique_ptr<ArcasDataChannel> nutbar();