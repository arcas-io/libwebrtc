#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/video_track_source.h"
#include "libwebrtc-sys/include/video_track_source_internal.h"

std::unique_ptr<ArcasVideoTrackSource> create_arcas_video_track_source()
{
    auto ref = rtc::make_ref_counted<ArcasVideoTrackSourceInternal>();
    return std::make_unique<ArcasVideoTrackSource>(ref);
}
