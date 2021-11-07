#pragma once
#include "rust/cxx.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "libwebrtc-sys/include/video_codec.h"
#include "libwebrtc-sys/include/video_encoder.h"
#include "libwebrtc-sys/include/webrtc_api.h"

class ArcasVideoFrameTypesCollection
{
private:
    const std::vector<webrtc::VideoFrameType> types_;

public:
    ArcasVideoFrameTypesCollection(rust::Vec<webrtc::VideoFrameType> types) : types_(types.begin(), types.end())
    {
    }

    const std::vector<webrtc::VideoFrameType> *as_ptr() const
    {
        return &types_;
    }
};

class ArcasVideoEncoderSettings
{
private:
    const webrtc::VideoEncoder::Capabilities capabilities_;
    const webrtc::VideoEncoder::Settings settings_;

public:
    ArcasVideoEncoderSettings(
        bool loss_notification,
        int number_of_cores,
        size_t max_payload_size) : capabilities_(loss_notification), settings_(capabilities_, number_of_cores, max_payload_size)
    {
    }

    const webrtc::VideoEncoder::Settings &as_ref() const
    {
        return settings_;
    }

    const webrtc::VideoEncoder::Capabilities &capabilities_ref() const
    {
        return capabilities_;
    }
};

class ArcasSDPVideoFormatWrapper
{
private:
    webrtc::SdpVideoFormat api_;

public:
    ArcasSDPVideoFormatWrapper(webrtc::SdpVideoFormat format) : api_(format) {}

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

    webrtc::SdpVideoFormat get_copy() const
    {
        return api_;
    }
};

class ArcasVideoEncoderWrapper : public webrtc::EncodedImageCallback
{
private:
    std::unique_ptr<webrtc::VideoEncoder> video_encoder_;
    rust::Box<ArcasRustEncodedImageCallbackHandler> frame_handler_;

public:
    ArcasVideoEncoderWrapper(
        std::unique_ptr<webrtc::VideoEncoder> video_encoder,
        rust::Box<ArcasRustEncodedImageCallbackHandler> frame_handler);

    Result OnEncodedImage(
        const webrtc::EncodedImage &encoded_image,
        const webrtc::CodecSpecificInfo *codec_specific_info) override;

    void OnDroppedFrame(webrtc::EncodedImageCallback::DropReason reason) override;

    int init_encode(const ArcasVideoCodec &codec, const ArcasVideoEncoderSettings &settings) const
    {
        RTC_LOG(LS_INFO) << "DO the thing " << codec.as_ptr()->VP9()->numberOfSpatialLayers << "<< \n";
        return video_encoder_->InitEncode(codec.as_ptr(), settings.as_ref());
    }

    int32_t release() const
    {
        return video_encoder_->Release();
    }

    int32_t encode(const webrtc::VideoFrame &frame,
                   const ArcasVideoFrameTypesCollection &frame_types) const
    {
        return video_encoder_->Encode(frame, frame_types.as_ptr());
    }

    void set_rates(const ArcasVideoEncoderRateControlParameters &settings) const
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
    ArcasVideoEncoderFactoryWrapper(std::unique_ptr<webrtc::VideoEncoderFactory> factory) : video_encoding_factory_(std::move(factory)) {}

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

    std::unique_ptr<ArcasVideoEncoderWrapper> create_encoder(const ArcasSDPVideoFormatWrapper &format, rust::Box<ArcasRustEncodedImageCallbackHandler> frame_handler) const
    {
        auto copy = format.get_copy();
        return std::make_unique<ArcasVideoEncoderWrapper>(video_encoding_factory_->CreateVideoEncoder(copy), std::move(frame_handler));
    }
};

std::shared_ptr<ArcasVideoFrameTypesCollection> create_arcas_video_frame_types_collection(rust::Vec<webrtc::VideoFrameType> types);

std::shared_ptr<ArcasVideoEncoderSettings> create_arcas_video_encoder_settings(
    bool loss_notification,
    int number_of_cores,
    size_t max_payload_size);

std::unique_ptr<ArcasVideoEncoderFactoryWrapper> create_arcas_video_encoder_factory_from_builtin();
