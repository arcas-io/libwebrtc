#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"

class ArcasRTPReceiver
{
private:
    rtc::scoped_refptr<webrtc::RtpReceiverInterface> api;

public:
    ArcasRTPReceiver(rtc::scoped_refptr<webrtc::RtpReceiverInterface> api);
};