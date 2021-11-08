#pragma once
#include "video_track_source_internal.h"

class ArcasVideoTrackSource
{
private:
    rtc::scoped_refptr<ArcasVideoTrackSourceInternal> api;

public:
    ArcasVideoTrackSource(rtc::scoped_refptr<ArcasVideoTrackSourceInternal> api) : api(api){};
    ~ArcasVideoTrackSource()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasVideoTrackSource";
    }

    rtc::scoped_refptr<ArcasVideoTrackSourceInternal> ref() const
    {
        return api;
    }

    void push_i420_data(int32_t width,
                        int32_t height,
                        int32_t stride_y,
                        int32_t stride_u,
                        int32_t stride_v,
                        const uint8_t *data) const
    {
        api->push_i420_data(width, height, stride_y, stride_u, stride_v, data);
    }

    std::unique_ptr<ArcasVideoTrackSource> clone() const
    {
        return std::make_unique<ArcasVideoTrackSource>(ref());
    }
};

std::unique_ptr<ArcasVideoTrackSource> create_arcas_video_track_source();