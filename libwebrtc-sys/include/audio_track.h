#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"

class ArcasAudioTrack
{
private:
    rtc::scoped_refptr<webrtc::AudioTrackInterface> api;

public:
};