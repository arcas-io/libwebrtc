#include "libwebrtc-sys/include/video_codec.h"

std::unique_ptr<ArcasCodecSpecificInfo> create_arcas_codec_specific_info()
{
    return std::make_unique<ArcasCodecSpecificInfo>();
}

std::shared_ptr<ArcasVideoCodec> create_arcas_video_codec()
{
    return std::make_shared<ArcasVideoCodec>();
}

std::shared_ptr<ArcasSpatialLayer> create_arcas_spatial_layer()
{
    return std::make_shared<ArcasSpatialLayer>();
}
