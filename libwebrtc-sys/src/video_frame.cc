#include "video_frame.h"
#include "libwebrtc-sys/src/video_frame.rs.h"

std::unique_ptr<ArcasVideoFrameFactory> create_arcas_video_frame_factory()
{
    return std::make_unique<ArcasVideoFrameFactory>();
}

std::shared_ptr<ArcasVideoFrameTypesCollection> create_arcas_video_frame_types_collection(rust::Vec<webrtc::VideoFrameType> types)
{
    return std::make_shared<ArcasVideoFrameTypesCollection>(types);
}
