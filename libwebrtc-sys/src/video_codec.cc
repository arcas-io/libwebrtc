#include "libwebrtc-sys/include/video_codec.h"

std::shared_ptr<ArcasVideoCodec> create_arcas_video_codec()
{
    return std::make_shared<ArcasVideoCodec>();
}

std::shared_ptr<ArcasSpatialLayer> create_arcas_spatial_layer()
{
    return std::make_shared<ArcasSpatialLayer>();
}

std::unique_ptr<ArcasVideoCodec> create_arcas_video_codec_from_cxx(const webrtc::VideoCodec* inst)
{
    return std::make_unique<ArcasVideoCodec>(inst);
}
