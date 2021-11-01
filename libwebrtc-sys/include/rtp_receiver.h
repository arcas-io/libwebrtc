#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"

class ArcasRTPReceiver
{
private:
    rtc::scoped_refptr<webrtc::RtpReceiverInterface> api;

public:
    ArcasRTPReceiver(rtc::scoped_refptr<webrtc::RtpReceiverInterface> api) : api(api){};
};

class ArcasRTPVideoReceiver : public ArcasRTPReceiver
{
public:
    ArcasRTPVideoReceiver(rtc::scoped_refptr<webrtc::RtpReceiverInterface> api) : ArcasRTPReceiver(api){};
};

class ArcasRTPAudioReceiver : public ArcasRTPReceiver
{
public:
    ArcasRTPAudioReceiver(rtc::scoped_refptr<webrtc::RtpReceiverInterface> api) : ArcasRTPReceiver(api){};
};