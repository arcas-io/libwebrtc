#pragma once
#include "api/media_stream_interface.h"

class ArcasMediaStreamTrack
{

    rtc::scoped_refptr<MediaStreamTrackInterface> ref()
    {
        return api;
    }

private:
    rtc::scoped_refptr<MediaStreamTrackInterface> api;
};
