#pragma once
#include "api/media_stream_interface.h"
#include "rust/cxx.h"

class ArcasAudioTrack
{
private:
    rtc::scoped_refptr<webrtc::AudioTrackInterface> api;

public:
    ArcasAudioTrack(rtc::scoped_refptr<webrtc::AudioTrackInterface> api)
    : api(api)
    {
    }

    rtc::scoped_refptr<webrtc::AudioTrackInterface> ref()
    {
        return api;
    }

    rust::String id()
    {
        return rust::String(api->id().c_str());
    }
};

std::unique_ptr<ArcasAudioTrack> gen_unique_audio_track();