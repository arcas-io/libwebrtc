#include "libwebrtc-sys/include/video_track_source.h"

std::shared_ptr<ArcasVideoTrackSource> create_arcas_video_track_source()
{
    return std::make_shared<ArcasVideoTrackSource>();
}

void push_i420_to_video_track_source(std::shared_ptr<ArcasVideoTrackSource> source,
                                     int32_t width,
                                     int32_t height,
                                     int32_t stride_y,
                                     int32_t stride_u,
                                     int32_t stride_v,
                                     uint8_t *data)
{
    source->push_i420_data(width, height, stride_y, stride_u, stride_v, data);
}
