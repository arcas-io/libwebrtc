#include "libwebrtc-sys/include/peerconnection_factory_config.h"

std::unique_ptr<ArcasPeerConnectionFactoryConfig> create_arcas_peerconnection_factory_config(
    std::unique_ptr<ArcasVideoEncoderFactory> video_encoder_factory,
    std::unique_ptr<ArcasVideoDecoderFactory> video_decoder_factory)
{
    return std::make_unique<ArcasPeerConnectionFactoryConfig>(std::move(video_encoder_factory),
                                                              std::move(video_decoder_factory));
}

void ArcasPeerConnectionFactoryConfig::set_video_encoder_factory(rust::Box<ArcasRustVideoEncoderFactory> api) {
    video_encoder_factory = create_arcas_video_encoder_factory(std::move(api));
}

void ArcasPeerConnectionFactoryConfig::set_video_decoder_factory(rust::Box<ArcasRustVideoDecoderFactory> api) {
    video_decoder_factory = create_arcas_video_decoder_factory(std::move(api));
}

void ArcasPeerConnectionFactoryConfig::set_audio_encoder_factory(rust::Box<ArcasRustAudioEncoderFactory> api) {
    audio_encoder_factory = absl::optional<rtc::scoped_refptr<ArcasAudioEncoderFactory>>(create_audio_encoder_factory(std::move(api)));
}