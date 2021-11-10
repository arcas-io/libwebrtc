#include "libwebrtc-sys/include/video_frame.h"

std::unique_ptr<ArcasVideoFrameFactory> create_arcas_video_frame_factory()
{
    return std::make_unique<ArcasVideoFrameFactory>();
}
