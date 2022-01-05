#pragma once
#include "api/media_stream_interface.h"
#include "rust/cxx.h"

class ArcasMediaStream
{
private:
    rtc::scoped_refptr<webrtc::MediaStreamInterface> api;

public:
    ArcasMediaStream(rtc::scoped_refptr<webrtc::MediaStreamInterface> api);

    rust::String id()
    {
        return rust::String(api->id().c_str());
    }

    rust::Vec<rust::String> GetAudioTracks()
    {
        rust::Vec<rust::String> vec;

        for (auto track : api->GetAudioTracks())
        {
            vec.push_back(rust::String(track->id().c_str()));
        }

        return vec;
    }

    rust::Vec<rust::String> GetVideoTracks()
    {
        rust::Vec<rust::String> vec;

        for (auto track : api->GetVideoTracks())
        {
            vec.push_back(rust::String(track->id().c_str()));
        }

        return vec;
    }
};

std::unique_ptr<ArcasMediaStream> gen_unique_media_stream();
