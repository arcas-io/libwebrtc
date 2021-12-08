#include "libwebrtc-sys/src/lib.rs.h"
#include "libwebrtc-sys/include/video_decoder_factory.h"

std::vector<webrtc::SdpVideoFormat> ArcasVideoDecoderFactory::GetSupportedFormats() const {
    auto result = api->get_supported_formats();
    return *result;
}

webrtc::VideoDecoderFactory::CodecSupport ArcasVideoDecoderFactory::QueryCodecSupport(
        const webrtc::SdpVideoFormat& format,
        bool reference_scaling)  const 
{
    auto out = api->query_codec_support(format, reference_scaling);
    return webrtc::VideoDecoderFactory::CodecSupport {
        .is_supported = out.is_supported,
        .is_power_efficient = out.is_power_efficient,
    };
}

std::unique_ptr<webrtc::VideoDecoder> ArcasVideoDecoderFactory::CreateVideoDecoder(
    const webrtc::SdpVideoFormat& format
) {
    auto proxy = api->create_video_decoder(format);
    return std::make_unique<ArcasVideoDecoder>(std::move(proxy));
}

std::unique_ptr<ArcasVideoDecoderFactory> create_arcas_video_decoder_factory(rust::Box<ArcasRustVideoDecoderFactory> proxy) {
    return std::make_unique<ArcasVideoDecoderFactory>(std::move(proxy));
}
