#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/video_track.h"

class ArcasRTPSender
{
protected:
    rtc::scoped_refptr<webrtc::RtpSenderInterface> api;

public:
    ArcasRTPSender(rtc::scoped_refptr<webrtc::RtpSenderInterface> api) : api(api){};
};

class ArcasRTPVideoSender : public ArcasRTPSender
{

public:
    ArcasRTPVideoSender(rtc::scoped_refptr<webrtc::RtpSenderInterface> api) : ArcasRTPSender(api){};

    bool set_track(const ArcasVideoTrack &track) const
    {
        return api->SetTrack(track.ref());
    }
};

class ArcasRTPAudioSender : public ArcasRTPSender
{

public:
    ArcasRTPAudioSender(rtc::scoped_refptr<webrtc::RtpSenderInterface> api) : ArcasRTPSender(api){};
};