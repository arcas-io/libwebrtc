#pragma once
#include "api/video/video_frame.h"
#include "video_codec.h"
#include "video_frame_buffer_encoded.h"
#include "video_track_source_internal.h"

class ArcasVideoTrackSource
{
private:
    rtc::scoped_refptr<ArcasVideoTrackSourceInternal> api;

public:
    ArcasVideoTrackSource(rtc::scoped_refptr<ArcasVideoTrackSourceInternal> api)
    : api(api){};
    ~ArcasVideoTrackSource()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasVideoTrackSource";
    }

    rtc::scoped_refptr<ArcasVideoTrackSourceInternal> ref() const
    {
        return api;
    }

    void push_frame(const webrtc::VideoFrame& frame) const
    {
        api->push_frame(frame);
    }

    std::unique_ptr<ArcasVideoTrackSource> cxx_clone() const
    {
        return std::make_unique<ArcasVideoTrackSource>(ref());
    }
};

std::unique_ptr<ArcasVideoTrackSource> create_arcas_video_track_source();
std::unique_ptr<ArcasVideoFrameEncodedImageData> extract_arcas_video_frame_to_raw_frame_buffer(const webrtc::VideoFrame& frame);
