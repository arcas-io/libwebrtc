#pragma once
#include "libwebrtc-sys/include/video_decoder_factory.h"
#include "libwebrtc-sys/include/video_encoder_factory.h"

struct ArcasPeerConnectionFactoryConfig
{
    ArcasPeerConnectionFactoryConfig(
        std::unique_ptr<ArcasVideoEncoderFactory> video_encoder_factory,
        std::unique_ptr<ArcasVideoDecoderFactory> video_decoder_factory) : video_encoder_factory(std::move(video_encoder_factory)),
                                                                           video_decoder_factory(std::move(video_decoder_factory)) {}

    std::unique_ptr<ArcasVideoEncoderFactory> video_encoder_factory;
    std::unique_ptr<ArcasVideoDecoderFactory> video_decoder_factory;
};


std::unique_ptr<ArcasPeerConnectionFactoryConfig> create_arcas_peerconnection_factory_config(
    std::unique_ptr<ArcasVideoEncoderFactory> video_encoder_factory,
    std::unique_ptr<ArcasVideoDecoderFactory> video_decoder_factory);
