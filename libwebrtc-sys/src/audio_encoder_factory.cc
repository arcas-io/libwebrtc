#include <audio_encoder_factory.h>
#include <libwebrtc-sys/src/audio_encoding.rs.h>

ArcasAudioEncoderFactory::ArcasAudioEncoderFactory(rust::Box<ArcasRustAudioEncoderFactory> fac)
: api{std::move(fac)}
{
}

std::vector<webrtc::AudioCodecSpec> ArcasAudioEncoderFactory::GetSupportedEncoders()
{
    std::vector<webrtc::AudioCodecSpec> result;
    auto ffi_result = api->get_supported_formats();
    for (auto spec : ffi_result)
    {
        result.push_back(webrtc::AudioCodecSpec{.format = from_arcas_sdp_audio_format(spec.format), .info = from_arcas_audio_codec_info(spec.info)});
    }
    return result;
}

absl::optional<webrtc::AudioCodecInfo> ArcasAudioEncoderFactory::QueryAudioEncoder(const webrtc::SdpAudioFormat& format)
{
    auto fmt = from_webrtc_sdp_audio_format(format);
    auto result = api->query_audio_encoder(fmt);
    if (result.sample_rate == 0 && result.num_channels == 0)
    {
        return absl::optional<webrtc::AudioCodecInfo>();
    }
    return absl::optional<webrtc::AudioCodecInfo>(from_arcas_audio_codec_info(result));
}

std::unique_ptr<WebRTCAudioEncoder> ArcasAudioEncoderFactory::MakeAudioEncoder(int payload_type,
                                                                               const webrtc::SdpAudioFormat& format,
                                                                               absl::optional<webrtc::AudioCodecPairId> codec_pair_id)
{
    auto fmt = from_webrtc_sdp_audio_format(format);
    return api->make_audio_encoder(payload_type, fmt);
}

rtc::scoped_refptr<ArcasAudioEncoderFactory> create_audio_encoder_factory(rust::Box<ArcasRustAudioEncoderFactory> api)
{
    return rtc::make_ref_counted<ArcasAudioEncoderFactory>(std::move(api));
}
