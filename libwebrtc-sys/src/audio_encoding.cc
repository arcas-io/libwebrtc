#include "audio_encoding.h"
#include "audio_encoder_factory.h"
#include "libwebrtc-sys/src/audio_encoding.rs.h"
#include "rtc_base/ref_counted_object.h"
#include "rtc_buffer.h"
#include "rust/cxx.h"

ArcasAudioEncoder::ArcasAudioEncoder(rust::Box<ArcasRustAudioEncoder> api)
: api(std::move(api))
{
}
int ArcasAudioEncoder::SampleRateHz() const
{
    return api->sample_rate_hz();
}
size_t ArcasAudioEncoder::NumChannels() const
{
    return api->num_channels();
}
size_t ArcasAudioEncoder::Num10MsFramesInNextPacket() const
{
    return api->num_10ms_frames_in_next_packet();
}
size_t ArcasAudioEncoder::Max10MsFramesInAPacket() const
{
    return api->max_10ms_frames_in_a_packet();
}

int ArcasAudioEncoder::GetTargetBitrate() const
{
    return api->get_target_bitrate();
}

absl::optional<std::pair<webrtc::TimeDelta, webrtc::TimeDelta>> ArcasAudioEncoder::GetFrameLengthRange() const
{
    return absl::optional<std::pair<webrtc::TimeDelta, webrtc::TimeDelta>>(
        std::make_pair<webrtc::TimeDelta, webrtc::TimeDelta>(webrtc::TimeDelta::Millis(5), webrtc::TimeDelta::Millis(60)));
}

void ArcasAudioEncoder::Reset() {}

EncodedInfo ArcasAudioEncoder::Encode(uint32_t rtp_timestamp, rtc::ArrayView<const int16_t> audio_data, rtc::Buffer* encoded)
{
    return EncodeImpl(rtp_timestamp, audio_data, encoded);
}

EncodedInfo ArcasAudioEncoder::EncodeImpl(uint32_t rtp_timestamp, rtc::ArrayView<const int16_t> audio_data, rtc::Buffer* encoded)
{
    auto encode_buffer = std::make_unique<BufferUint8>(encoded);
    auto ffi_result = api->encode_impl(rtp_timestamp, audio_data.data(), audio_data.size(), std::move(encode_buffer));
    auto result = EncodedInfo();

    {
        result.encoded_bytes = ffi_result.encoded_bytes;
        result.encoded_timestamp = ffi_result.encoded_timestamp;
        result.encoder_type = ffi_result.encoder_type;
        result.payload_type = ffi_result.payload_type;
        result.send_even_if_empty = ffi_result.send_even_if_empty;
        result.speech = ffi_result.speech;
    }

    return result;
}

// AudioEncoderFactory
webrtc::SdpAudioFormat from_arcas_sdp_audio_format(ArcasSdpAudioFormat const& format)
{
    std::map<std::string, std::string> parameters;
    auto size = format.parameters.size();
    for (size_t idx = 0; idx < size; idx += 2)
    {
        if (idx + 1 >= size)
        {
            break;
        }
        auto& l = format.parameters[idx];
        auto& r = format.parameters[idx + 1];
        std::string key{l.data(), l.size()};
        std::string val{r.data(), r.size()};
        parameters.emplace(key, val);
    }
    std::string name{format.name.data(), format.name.size()};
    return webrtc::SdpAudioFormat(name, format.clockrate_hz, format.num_channels, std::move(parameters));
}

ArcasSdpAudioFormat from_webrtc_sdp_audio_format(const webrtc::SdpAudioFormat& format)
{
    ArcasSdpAudioFormat result;
    result.name = rust::String(format.name.c_str());
    result.clockrate_hz = format.clockrate_hz;
    result.num_channels = format.num_channels;
    for (auto entry : format.parameters)
    {
        result.parameters.push_back(std::move(rust::String(entry.first.c_str())));
        result.parameters.push_back(std::move(rust::String(entry.second.c_str())));
    }
    return result;
}

webrtc::AudioCodecInfo from_arcas_audio_codec_info(const ArcasAudioCodecInfo& info)
{
    return webrtc::AudioCodecInfo(info.sample_rate, info.num_channels, info.default_bitrate_bps, info.min_bitrate_bps, info.max_bitrate_bps);
}

ArcasAudioCodecInfo from_webrtc_audio_codec_info(const webrtc::AudioCodecInfo& info)
{
    return ArcasAudioCodecInfo{.sample_rate = info.sample_rate_hz,
                               .num_channels = info.num_channels,
                               .default_bitrate_bps = info.default_bitrate_bps,
                               .min_bitrate_bps = info.min_bitrate_bps,
                               .max_bitrate_bps = info.max_bitrate_bps,
                               .allow_comfort_noise = info.allow_comfort_noise,
                               .supports_network_adaptation = info.supports_network_adaption};
}


std::unique_ptr<ArcasAudioEncoder> create_audio_encoder(rust::Box<ArcasRustAudioEncoder> api)
{
    auto result = std::make_unique<ArcasAudioEncoder>(std::move(api));
    return result;
}
