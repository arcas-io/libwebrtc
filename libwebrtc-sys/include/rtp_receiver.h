#pragma once
#include "api/rtp_receiver_interface.h"

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

std::unique_ptr<ArcasRTPReceiver> gen_unique_rtp_receiver();
std::unique_ptr<ArcasRTPAudioReceiver> gen_unique_rtp_audio_receiver();
std::unique_ptr<ArcasRTPVideoReceiver> gen_unique_rtp_video_receiver();
