#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"

class ArcasRTPTransceiver
{
private:
    rtc::scoped_refptr<webrtc::RtpTransceiverInterface> api;

public:
    ArcasRTPTransceiver(rtc::scoped_refptr<webrtc::RtpTransceiverInterface> api);
};