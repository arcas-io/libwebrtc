#pragma once
#include "libwebrtc-sys/include/video_codec.h"
#include "libwebrtc-sys/include/video_frame.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "libwebrtc-sys/include/video_encoder_rate_control_parameters.h"
#include "libwebrtc-sys/include/encoded_image_callback.h"

class ArcasReactiveVideoEncoderWrapper
{
private:
    std::unique_ptr<webrtc::VideoEncoder> video_encoder_;

public:
    ArcasReactiveVideoEncoderWrapper(
        std::unique_ptr<webrtc::VideoEncoder> video_encoder) : video_encoder_(std::move(video_encoder)){};

    int register_encode_complete_callback(const ArcasEncodedImageCallback *callback) const
    {
        return video_encoder_->RegisterEncodeCompleteCallback(callback->get());
    }

    int init_encode(const ArcasCxxVideoCodec *codec, int32_t number_of_cores, size_t max_payload_size) const
    {
        webrtc::VideoEncoder::Settings settings(webrtc::VideoEncoder::Capabilities(true), number_of_cores, max_payload_size);
        return video_encoder_->InitEncode(codec, settings);
    }

    int32_t release() const
    {
        return video_encoder_->Release();
    }

    int32_t encode(const webrtc::VideoFrame &frame,
                   const std::vector<webrtc::VideoFrameType> *frame_types) const
    {
        return video_encoder_->Encode(frame, frame_types);
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
