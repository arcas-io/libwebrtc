#pragma once
#include <api/audio_codecs/audio_encoder.h>
#include <api/audio_codecs/audio_format.h>

#include "rust/cxx.h"
#include "rust_shared.h"

using WebRTCAudioEncoder = webrtc::AudioEncoder;
using EncodedInfo = webrtc::AudioEncoder::EncodedInfo;
using ArcasCxxAudioCodecType = webrtc::AudioEncoder::CodecType;

class ArcasAudioEncoder : public WebRTCAudioEncoder
{
private:
    rust::Box<ArcasRustAudioEncoder> api;

public:
    ArcasAudioEncoder(rust::Box<ArcasRustAudioEncoder>);
    int SampleRateHz() const override;
    size_t NumChannels() const override;
    size_t Num10MsFramesInNextPacket() const override;
    size_t Max10MsFramesInAPacket() const override;
    void Reset() override;
    absl::optional<std::pair<webrtc::TimeDelta, webrtc::TimeDelta>> GetFrameLengthRange() const override;
    int GetTargetBitrate() const override;
    EncodedInfo Encode(uint32_t rtp_timestamp, rtc::ArrayView<const int16_t> audio, rtc::Buffer* encoded);

protected:
    EncodedInfo EncodeImpl(uint32_t rtp_timestamp, rtc::ArrayView<const int16_t> audio, rtc::Buffer* encoded) override;
};

std::unique_ptr<ArcasAudioEncoder> create_audio_encoder(rust::Box<ArcasRustAudioEncoder>);
webrtc::SdpAudioFormat from_arcas_sdp_audio_format(ArcasSdpAudioFormat const& format);
webrtc::AudioCodecInfo from_arcas_audio_codec_info(ArcasAudioCodecInfo const& info);
ArcasSdpAudioFormat from_webrtc_sdp_audio_format(webrtc::SdpAudioFormat const& format);
