#pragma once
#include "api/video_codecs/sdp_video_format.h"
#include "api/video_codecs/video_decoder.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "rust/cxx.h"

class ArcasVideoDecoderFactory : public webrtc::VideoDecoderFactory
{
public:
    ArcasVideoDecoderFactory(rust::Box<ArcasRustVideoDecoderFactory> api) : api(std::move(api)) {}
    // Returns a list of supported video formats in order of preference, to use
    // for signaling etc.
    virtual std::vector<webrtc::SdpVideoFormat> GetSupportedFormats() const;

    // Query whether the specifed format is supported or not and if it will be
    // power efficient, which is currently interpreted as if there is support for
    // hardware acceleration.
    // The parameter `reference_scaling` is used to query support for prediction
    // across spatial layers. An example where support for reference scaling is
    // needed is if the video stream is produced with a scalability mode that has
    // a dependency between the spatial layers. See
    // https://w3c.github.io/webrtc-svc/#scalabilitymodes* for a specification of
    // different scalabilty modes. NOTE: QueryCodecSupport is currently an
    // experimental feature that is subject to change without notice.
    virtual webrtc::VideoDecoderFactory::CodecSupport QueryCodecSupport(const webrtc::SdpVideoFormat &format, bool reference_scaling) const;

    // Creates a VideoDecoder for the spArcasRustVideoDecoderecified format.
    virtual std::unique_ptr<webrtc::VideoDecoder> CreateVideoDecoder(
        const webrtc::SdpVideoFormat &format);

private:
    rust::Box<ArcasRustVideoDecoderFactory> api;
};

std::unique_ptr<ArcasVideoDecoderFactory> create_arcas_video_decoder_factory(rust::Box<ArcasRustVideoDecoderFactory> proxy);

template <>
struct rust::IsRelocatable<ArcasRustVideoDecoder> : std::true_type
{
};
