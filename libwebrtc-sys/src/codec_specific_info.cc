#include "libwebrtc-sys/include/codec_specific_info.h"

std::unique_ptr<ArcasCodecSpecificInfo> create_arcas_codec_specific_info()
{
    return std::make_unique<ArcasCodecSpecificInfo>();
}
