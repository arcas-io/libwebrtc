#pragma once
#include "libwebrtc-sys/include/alias.h"
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "rust/cxx.h"

class ArcasVideoEncoderSelector : public webrtc::VideoEncoderFactory::EncoderSelectorInterface
{
private:
    rust::Vec<ArcasRustVideoEncoderSelector> api;

public:
    ArcasVideoEncoderSelector(rust::Vec<ArcasRustVideoEncoderSelector> api);

    // Informs the encoder selector about which encoder that is currently being
    // used.
    void OnCurrentEncoder(const webrtc::SdpVideoFormat &format) override;

    // Called every time the available bitrate is updated. Should return a
    // non-empty if an encoder switch should be performed.
    absl::optional<webrtc::SdpVideoFormat> OnAvailableBitrate(
        const webrtc::DataRate &rate) override;

    // Called if the currently used encoder reports itself as broken. Should
    // return a non-empty if an encoder switch should be performed.
    absl::optional<webrtc::SdpVideoFormat> OnEncoderBroken() override;
};

class ArcasVideoEncoderFactory : public webrtc::VideoEncoderFactory
{
private:
    rust::Box<ArcasRustVideoEncoderFactory> api;

public:
    ArcasVideoEncoderFactory(rust::Box<ArcasRustVideoEncoderFactory> api) : api(std::move(api)){};

    // Returns a list of supported video formats in order of preference, to use
    // for signaling etc.
    std::vector<webrtc::SdpVideoFormat> GetSupportedFormats() const override;

    // Returns a list of supported video formats in order of preference, that can
    // also be tagged with additional information to allow the VideoEncoderFactory
    // to separate between different implementations when CreateVideoEncoder is
    // called.
    std::vector<webrtc::SdpVideoFormat> GetImplementations() const override;

    // Returns information about how this format will be encoded. The specified
    // format must be one of the supported formats by this factory.

    // TODO(magjed): Try to get rid of this method. Since is_hardware_accelerated
    // is unused, only factories producing internal source encoders (in itself a
    // deprecated feature) needs to override this method.
    CodecInfo QueryVideoEncoder(const webrtc::SdpVideoFormat &format) const override;

    // Query whether the specifed format is supported or not and if it will be
    // power efficient, which is currently interpreted as if there is support for
    // hardware acceleration.
    // See https://w3c.github.io/webrtc-svc/#scalabilitymodes* for a specification
    // of valid values for `scalability_mode`.
    // NOTE: QueryCodecSupport is currently an experimental feature that is
    // subject to change without notice.
    webrtc::VideoEncoderFactory::CodecSupport QueryCodecSupport(
        const webrtc::SdpVideoFormat &format,
        absl::optional<std::string> scalability_mode) const override;

    // Creates a VideoEncoder for the specified format.
    std::unique_ptr<webrtc::VideoEncoder> CreateVideoEncoder(
        const webrtc::SdpVideoFormat &format) override;

    std::unique_ptr<EncoderSelectorInterface> GetEncoderSelector() const override;
};

std::unique_ptr<ArcasVideoEncoderFactory> create_arcas_video_encoder_factory(rust::Box<ArcasRustVideoEncoderFactory> api);

ArcasVideoEncodingErrCode get_arcas_video_encoding_err_codes();