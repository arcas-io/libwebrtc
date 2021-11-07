#include "libwebrtc-sys/include/video_encoder_factory.h"
#include "libwebrtc-sys/src/lib.rs.h"

std::vector<webrtc::SdpVideoFormat> ArcasVideoEncoderFactory::GetSupportedFormats() const
{
    auto result = api->get_supported_formats();
    return *result;
};

std::vector<webrtc::SdpVideoFormat> ArcasVideoEncoderFactory::GetImplementations() const
{
    auto result = api->get_implementations();
    return *result;
}

webrtc::VideoEncoderFactory::CodecInfo ArcasVideoEncoderFactory::QueryVideoEncoder(const webrtc::SdpVideoFormat &format) const
{
    auto out = api->query_video_encoder(format);
    return webrtc::VideoEncoderFactory::CodecInfo{
        .has_internal_source = out.has_internal_source};
}

webrtc::VideoEncoderFactory::CodecSupport ArcasVideoEncoderFactory::QueryCodecSupport(
    const webrtc::SdpVideoFormat &format,
    absl::optional<std::string> scalability_mode) const
{
    rust::Vec<rust::String> rust_mode;

    if (scalability_mode.has_value())
    {
        rust_mode.push_back(scalability_mode.value().c_str());
    }

    auto out = api->query_codec_support(format, rust_mode);
    return webrtc::VideoEncoderFactory::CodecSupport{
        .is_supported = out.is_supported,
        .is_power_efficient = out.is_power_efficient,
    };
}

// Creates a VideoEncoder for the specified format.
std::unique_ptr<webrtc::VideoEncoder> ArcasVideoEncoderFactory::CreateVideoEncoder(
    const webrtc::SdpVideoFormat &format)
{
    auto proxy = api->create_video_encoder(format);
    return std::make_unique<ArcasVideoEncoder>(std::move(proxy));
}

std::unique_ptr<webrtc::VideoEncoderFactory::EncoderSelectorInterface> ArcasVideoEncoderFactory::GetEncoderSelector() const
{
    auto proxy = api->get_encoder_selector();

    if (proxy.size() > 0)
    {
        return std::make_unique<ArcasVideoEncoderSelector>(std::move(proxy));
    }
    else
    {
        return nullptr;
    }
}

ArcasVideoEncoderSelector::ArcasVideoEncoderSelector(rust::Vec<ArcasRustVideoEncoderSelector> api) : api(std::move(api)) {}

void ArcasVideoEncoderSelector::OnCurrentEncoder(const webrtc::SdpVideoFormat &format)
{
    api[0].on_current_encoder(format);
};

absl::optional<webrtc::SdpVideoFormat> ArcasVideoEncoderSelector::OnAvailableBitrate(
    const webrtc::DataRate &rate)
{

    absl::optional<webrtc::SdpVideoFormat> result;
    auto rust_result = api[0].on_available_bitrate(rate);

    if (rust_result->size() > 0)
    {
        result.emplace(rust_result->at(0));
    }

    return result;
};

absl::optional<webrtc::SdpVideoFormat> ArcasVideoEncoderSelector::OnEncoderBroken()
{

    absl::optional<webrtc::SdpVideoFormat> result;
    auto rust_result = api[0].on_encoder_broken();

    if (rust_result->size() > 0)
    {
        result.emplace(rust_result->at(0));
    }

    return result;
};

std::unique_ptr<ArcasVideoEncoderFactory> create_arcas_video_encoder_factory(rust::Box<ArcasRustVideoEncoderFactory> api)
{
    return std::make_unique<ArcasVideoEncoderFactory>(std::move(api));
}

ArcasVideoEncodingErrCode get_arcas_video_encoding_err_codes()
{
    return ArcasVideoEncodingErrCode{
        .VIDEO_CODEC_OK_REQUEST_KEYFRAME = WEBRTC_VIDEO_CODEC_OK_REQUEST_KEYFRAME,
        .VIDEO_CODEC_NO_OUTPUT = WEBRTC_VIDEO_CODEC_NO_OUTPUT,
        .VIDEO_CODEC_OK = WEBRTC_VIDEO_CODEC_OK,
        .VIDEO_CODEC_ERROR = WEBRTC_VIDEO_CODEC_ERROR,
        .VIDEO_CODEC_MEMORY = WEBRTC_VIDEO_CODEC_MEMORY,
        .VIDEO_CODEC_ERR_PARAMETER = WEBRTC_VIDEO_CODEC_ERR_PARAMETER,
        .VIDEO_CODEC_UNINITIALIZED = WEBRTC_VIDEO_CODEC_UNINITIALIZED,
        .VIDEO_CODEC_FALLBACK_SOFTWARE = WEBRTC_VIDEO_CODEC_FALLBACK_SOFTWARE,
        .VIDEO_CODEC_TARGET_BITRATE_OVERSHOOT = WEBRTC_VIDEO_CODEC_TARGET_BITRATE_OVERSHOOT,
        .VIDEO_CODEC_ERR_SIMULCAST_PARAMETERS_NOT_SUPPORTED = WEBRTC_VIDEO_CODEC_ERR_SIMULCAST_PARAMETERS_NOT_SUPPORTED,
        .VIDEO_CODEC_ENCODER_FAILURE = WEBRTC_VIDEO_CODEC_ENCODER_FAILURE,
    };
}
