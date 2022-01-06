#pragma once
#include "libwebrtc-sys/include/reactive_video_encoder_wrapper.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "libwebrtc-sys/include/video_codec.h"
#include "libwebrtc-sys/include/video_encoder.h"
#include "libwebrtc-sys/include/video_frame.h"
#include "rust/cxx.h"

template<>
struct rust::IsRelocatable<ArcasRustEncodedImageCallbackHandler> : std::true_type
{
};

class ArcasSDPVideoFormatWrapper
{
private:
    webrtc::SdpVideoFormat api_;

public:
    ArcasSDPVideoFormatWrapper(webrtc::SdpVideoFormat format)
    : api_(format)
    {
    }

    rust::String get_name() const
    {
        return rust::String(api_.name.c_str());
    }

    rust::Vec<ArcasRustDict> get_parameters() const;

    rust::String to_string() const
    {
        return rust::String(api_.ToString().c_str());
    }

    std::unique_ptr<ArcasSDPVideoFormatWrapper> clone() const
    {
        return std::make_unique<ArcasSDPVideoFormatWrapper>(api_);
    }

    // Helper for use with encoder get_supported_formats.
    std::unique_ptr<std::vector<webrtc::SdpVideoFormat>> cxx_format_list() const
    {
        auto output = std::make_unique<std::vector<webrtc::SdpVideoFormat>>();
        output->push_back(this->get_copy());
        return output;
    }

    webrtc::SdpVideoFormat get_copy() const
    {
        return api_;
    }
};

class ArcasVideoEncoderWrapper : public webrtc::EncodedImageCallback
{
private:
    std::unique_ptr<webrtc::VideoEncoder>           video_encoder_;
    rust::Box<ArcasRustEncodedImageCallbackHandler> frame_handler_;

public:
    ArcasVideoEncoderWrapper(std::unique_ptr<webrtc::VideoEncoder>           video_encoder,
                             rust::Box<ArcasRustEncodedImageCallbackHandler> frame_handler);

    Result OnEncodedImage(const webrtc::EncodedImage&      encoded_image,
                          const webrtc::CodecSpecificInfo* codec_specific_info) override;

    void OnDroppedFrame(webrtc::EncodedImageCallback::DropReason reason) override;

    int init_encode(const ArcasVideoCodec& codec, const ArcasVideoEncoderSettings& settings) const
    {
        return video_encoder_->InitEncode(codec.as_ptr(), settings.as_ref());
    }

    int cxx_init_encode(const ArcasCxxVideoCodec* codec,
                        int32_t                   number_of_cores,
                        size_t                    max_payload_size) const
    {
        webrtc::VideoEncoder::Settings settings(webrtc::VideoEncoder::Capabilities(true),
                                                number_of_cores,
                                                max_payload_size);
        return video_encoder_->InitEncode(codec, settings);
    }

    int32_t release() const
    {
        return video_encoder_->Release();
    }

    int32_t cxx_encode(const webrtc::VideoFrame&                  frame,
                       const std::vector<webrtc::VideoFrameType>* frame_types) const
    {
        return video_encoder_->Encode(frame, frame_types);
    }

    int32_t encode(const webrtc::VideoFrame&             frame,
                   const ArcasVideoFrameTypesCollection& frame_types) const
    {
        return video_encoder_->Encode(frame, frame_types.as_ptr());
    }

    void cxx_set_rates(const webrtc::VideoEncoder::RateControlParameters& parameters) const
    {
        video_encoder_->SetRates(parameters);
    }

    void set_rates(const ArcasVideoEncoderRateControlParameters& settings) const
    {
        video_encoder_->SetRates(settings.as_ref());
    }

    void on_packet_loss_rate_update(float packet_loss_rate) const
    {
        video_encoder_->OnPacketLossRateUpdate(packet_loss_rate);
    }

    void on_rtt_update(int64_t rtt) const
    {
        video_encoder_->OnRttUpdate(rtt);
    }

    void on_loss_notification(ArcasVideoEncoderLossNotification loss) const;

    ArcasVideoEncoderInfo get_encoder_info() const;
};

class ArcasVideoEncoderFactoryWrapper
{
private:
    std::unique_ptr<webrtc::VideoEncoderFactory> video_encoding_factory_;

public:
    ArcasVideoEncoderFactoryWrapper(std::unique_ptr<webrtc::VideoEncoderFactory> factory)
    : video_encoding_factory_(std::move(factory))
    {
    }

    std::unique_ptr<std::vector<ArcasSDPVideoFormatWrapper>> get_supported_formats() const
    {
        auto formats = video_encoding_factory_->GetSupportedFormats();
        auto output = std::make_unique<std::vector<ArcasSDPVideoFormatWrapper>>();
        for (auto format : formats)
        {
            ArcasSDPVideoFormatWrapper rust_output(format);
            output->push_back(rust_output);
        }
        return output;
    }

    std::unique_ptr<std::vector<webrtc::SdpVideoFormat>> cxx_get_supported_formats() const
    {
        auto formats = video_encoding_factory_->GetSupportedFormats();
        auto output = std::make_unique<std::vector<webrtc::SdpVideoFormat>>();
        for (auto format : formats) { output->push_back(format); }
        return output;
    }

    std::unique_ptr<ArcasVideoEncoderWrapper>
    create_encoder(const ArcasSDPVideoFormatWrapper&               format,
                   rust::Box<ArcasRustEncodedImageCallbackHandler> frame_handler) const
    {
        auto copy = format.get_copy();
        return std::make_unique<ArcasVideoEncoderWrapper>(
            video_encoding_factory_->CreateVideoEncoder(copy),
            std::move(frame_handler));
    }

    std::unique_ptr<ArcasReactiveVideoEncoderWrapper>
    create_encoder_reactive(const webrtc::SdpVideoFormat& format) const
    {
        return std::make_unique<ArcasReactiveVideoEncoderWrapper>(
            video_encoding_factory_->CreateVideoEncoder(format));
    }
};

std::shared_ptr<ArcasVideoEncoderSettings> create_arcas_video_encoder_settings(
    bool loss_notification, int number_of_cores, size_t max_payload_size);

std::unique_ptr<ArcasVideoEncoderFactoryWrapper> create_arcas_video_encoder_factory_from_builtin();
std::unique_ptr<ArcasCxxVideoEncoderEncoderInfo>
get_video_encoder_encoder_info(const webrtc::VideoEncoder& encoder);

std::unique_ptr<ArcasVideoEncoderWrapper>   gen_unique_video_encoder_wrapper();
std::unique_ptr<ArcasSDPVideoFormatWrapper> gen_unique_sdp_video_format_wrapper();
std::unique_ptr<std::vector<ArcasSDPVideoFormatWrapper>>
gen_unique_vector_sdp_video_format_wrapper();
