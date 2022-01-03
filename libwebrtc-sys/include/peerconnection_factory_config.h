#pragma once
#include "libwebrtc-sys/include/video_decoder_factory.h"
#include "libwebrtc-sys/include/video_encoder_factory.h"
#include "audio_encoder_factory.h"
#include "rust_shared.h"
#include "rust/cxx.h"

struct ArcasPeerConnectionFactoryConfig
{
    ArcasPeerConnectionFactoryConfig(
        std::unique_ptr<ArcasVideoEncoderFactory> video_encoder_factory,
        std::unique_ptr<ArcasVideoDecoderFactory> video_decoder_factory
    ):
        video_encoder_factory(std::move(video_encoder_factory)),
        video_decoder_factory(std::move(video_decoder_factory)) {
        }

    ArcasPeerConnectionFactoryConfig() {}

    std::unique_ptr<ArcasVideoEncoderFactory> video_encoder_factory;
    std::unique_ptr<ArcasVideoDecoderFactory> video_decoder_factory;
    absl::optional<rtc::scoped_refptr<ArcasAudioEncoderFactory>> audio_encoder_factory;

    void set_video_encoder_factory(rust::Box<ArcasRustVideoEncoderFactory>);
    // void set_video_encoder_factory();
    void set_video_decoder_factory(rust::Box<ArcasRustVideoDecoderFactory>);
    void set_audio_encoder_factory(rust::Box<ArcasRustAudioEncoderFactory>);
};


std::unique_ptr<ArcasPeerConnectionFactoryConfig> create_arcas_peerconnection_factory_config(
    std::unique_ptr<ArcasVideoEncoderFactory> video_encoder_factory,
    std::unique_ptr<ArcasVideoDecoderFactory> video_decoder_factory);
