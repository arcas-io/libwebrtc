#include "libwebrtc-sys/include/peerconnection_factory_config.h"

std::unique_ptr<ArcasPeerConnectionFactoryConfig> create_arcas_peerconnection_factory_config(
    std::unique_ptr<ArcasVideoEncoderFactory> video_encoder_factory,
    std::unique_ptr<ArcasVideoDecoderFactory> video_decoder_factory)
{
    return std::make_unique<ArcasPeerConnectionFactoryConfig>(std::move(video_encoder_factory),
                                                              std::move(video_decoder_factory));
}
