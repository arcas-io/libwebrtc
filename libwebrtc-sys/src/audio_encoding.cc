#include "rust/cxx.h"
#include "libwebrtc-sys/include/audio_encoding.h"
#include "libwebrtc-sys/include/rtc_buffer.h"
#include "libwebrtc-sys/include/audio_encoder_factory.h"
#include "libwebrtc-sys/src/audio_encoding.rs.h"
#include "rtc_base/ref_counted_object.h"

ArcasAudioEncoder::ArcasAudioEncoder(rust::Box<ArcasRustAudioEncoder> api) : api(std::move(api)) {}
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
    return absl::optional<std::pair<webrtc::TimeDelta, webrtc::TimeDelta>>();
}

EncodedInfo ArcasAudioEncoder::EncodeImpl(
    uint32_t rtp_timestamp,
    rtc::ArrayView<const int16_t> audio_data,
    rtc::Buffer *encoded)
{
    auto encode_buffer = std::make_unique<BufferUint8>(BufferUint8(encoded));
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
const webrtc::SdpAudioFormat from_arcas_sdp_audio_format(ArcasSdpAudioFormat &format)
{
    std::map<std::string, std::string> parameters;
    auto size = format.parameters.size();
    for (size_t idx = 0; idx < size; idx += 2)
    {
        if (idx + 1 >= size)
        {
            break;
        }
        parameters.insert({std::string(format.parameters[idx].c_str()),
                           std::string(format.parameters[idx + 1].c_str())});
    }
    auto name = std::string(format.name.c_str());
    return webrtc::SdpAudioFormat(
        name,
        format.clockrate_hz,
        format.num_channels,
        std::move(parameters));
}

ArcasSdpAudioFormat from_webrtc_sdp_audio_format(const webrtc::SdpAudioFormat &format)
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

webrtc::AudioCodecInfo from_arcas_audio_codec_info(const ArcasAudioCodecInfo &info)
{
    return webrtc::AudioCodecInfo(
        info.sample_rate,
        info.num_channels,
        info.default_bitrate_bps,
        info.min_bitrate_bps,
        info.max_bitrate_bps);
}

ArcasAudioCodecInfo from_webrtc_audio_codec_info(const webrtc::AudioCodecInfo &info)
{
    return ArcasAudioCodecInfo{
        .sample_rate = info.sample_rate_hz,
        .num_channels = info.num_channels,
        .default_bitrate_bps = info.default_bitrate_bps,
        .min_bitrate_bps = info.min_bitrate_bps,
        .max_bitrate_bps = info.max_bitrate_bps,
        .allow_comfort_noise = info.allow_comfort_noise,
        .supports_network_adaptation = info.supports_network_adaption};
}

std::vector<webrtc::AudioCodecSpec> ArcasAudioEncoderFactory::GetSupportedEncoders()
{
    std::vector<webrtc::AudioCodecSpec> result;
    auto ffi_result = api->get_supported_formats();
    for (auto spec : ffi_result)
    {
        result.push_back(webrtc::AudioCodecSpec{
            .format = from_arcas_sdp_audio_format(spec.format),
            .info = from_arcas_audio_codec_info(spec.info)});
    }
    return result;
}

absl::optional<webrtc::AudioCodecInfo> ArcasAudioEncoderFactory::QueryAudioEncoder(const webrtc::SdpAudioFormat &format)
{
    auto fmt = from_webrtc_sdp_audio_format(format);
    auto result = api->query_audio_encoder(fmt);
    if (result.sample_rate == 0 && result.num_channels == 0)
    {
        return absl::optional<webrtc::AudioCodecInfo>();
    }
    return absl::optional<webrtc::AudioCodecInfo>(from_arcas_audio_codec_info(result));
}

std::unique_ptr<WebRTCAudioEncoder> ArcasAudioEncoderFactory::MakeAudioEncoder(
    int payload_type,
    const webrtc::SdpAudioFormat &format,
    absl::optional<webrtc::AudioCodecPairId> codec_pair_id)
{
    auto fmt = from_webrtc_sdp_audio_format(format);
    return api->make_audio_encoder(payload_type, fmt);
}

ArcasAudioEncoderFactory::ArcasAudioEncoderFactory(rust::Box<ArcasRustAudioEncoderFactory> api): api(std::move(api)) {}

std::unique_ptr<WebRTCAudioEncoder> create_audio_encoder(rust::Box<ArcasRustAudioEncoder> api)
{
    auto result = std::make_unique<ArcasAudioEncoder>(std::move(api));
    return result;
}

rtc::scoped_refptr<ArcasAudioEncoderFactory> create_audio_encoder_factory(rust::Box<ArcasRustAudioEncoderFactory> api)
{
    return rtc::make_ref_counted<ArcasAudioEncoderFactory>(std::move(api));
}