#include "libwebrtc-sys/include/video_codec.h"

std::unique_ptr<ArcasCodecSpecificInfo> create_arcas_codec_specific_info()
{
    return std::make_unique<ArcasCodecSpecificInfo>();
}