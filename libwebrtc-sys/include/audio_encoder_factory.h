#pragma once
#include "rust/cxx.h"
#include "rust_shared.h"
#include "api/audio_codecs/audio_encoder_factory.h"
#include "api/scoped_refptr.h"

using AudioEncoderFactory = webrtc::AudioEncoderFactory;

class ArcasAudioEncoderFactory : public AudioEncoderFactory
{
public:
    ArcasAudioEncoderFactory(rust::Box<ArcasRustAudioEncoderFactory>);
    std::vector<webrtc::AudioCodecSpec> GetSupportedEncoders() override;
    absl::optional<webrtc::AudioCodecInfo> QueryAudioEncoder(const webrtc::SdpAudioFormat &) override;
    std::unique_ptr<webrtc::AudioEncoder> MakeAudioEncoder(
        int payload_type,
        const webrtc::SdpAudioFormat &,
        absl::optional<webrtc::AudioCodecPairId> codec_pair_id) override;

private:
    rust::Box<ArcasRustAudioEncoderFactory> api;
};

rtc::scoped_refptr<ArcasAudioEncoderFactory> create_audio_encoder_factory(rust::Box<ArcasRustAudioEncoderFactory>);