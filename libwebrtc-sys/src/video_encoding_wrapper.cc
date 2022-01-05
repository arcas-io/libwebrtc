#include "libwebrtc-sys/include/video_encoding_wrapper.h"
#include "api/video_codecs/builtin_video_encoder_factory.h"
#include "libwebrtc-sys/src/shared_bridge.rs.h"
#include "libwebrtc-sys/src/video_encoder_factory_wrapper.rs.h"
#include "libwebrtc-sys/src/video_encoding.rs.h"
#include "libwebrtc-sys/src/video_frame_buffer.rs.h"
#include <iostream>

rust::Vec<ArcasRustDict> ArcasSDPVideoFormatWrapper::get_parameters() const
{
    rust::Vec<ArcasRustDict> output;
    for (auto param : api_.parameters)
    {
        ArcasRustDict output_item{rust::String(param.first.c_str()),
                                  rust::String(param.second.c_str())};
        output.push_back(output_item);
    }
    return output;
}

std::shared_ptr<ArcasVideoEncoderSettings> create_arcas_video_encoder_settings(
    bool loss_notification, int number_of_cores, size_t max_payload_size)
{
    return std::make_shared<ArcasVideoEncoderSettings>(loss_notification,
                                                       number_of_cores,
                                                       max_payload_size);
}

ArcasVideoEncoderWrapper::ArcasVideoEncoderWrapper(
    std::unique_ptr<webrtc::VideoEncoder>           video_encoder,
    rust::Box<ArcasRustEncodedImageCallbackHandler> frame_handler)
: video_encoder_(std::move(video_encoder))
, frame_handler_(std::move(frame_handler))
{
    if (this->video_encoder_->RegisterEncodeCompleteCallback(this) != 0)
    {
        RTC_LOG(LS_ERROR) << "Failed to register encode complete callback";
    }
}

void ArcasVideoEncoderWrapper::OnDroppedFrame(webrtc::EncodedImageCallback::DropReason reason)
{
    frame_handler_->trigger_dropped(reason);
}

webrtc::EncodedImageCallback::Result
ArcasVideoEncoderWrapper::OnEncodedImage(const webrtc::EncodedImage&      encoded_image,
                                         const webrtc::CodecSpecificInfo* codec_specific_info)
{
    auto current_encoded_image = std::make_unique<webrtc::EncodedImage>(encoded_image);
    auto current_codec_specific_info =
        std::make_unique<ArcasCodecSpecificInfo>(*codec_specific_info);

    frame_handler_->trigger_encoded_image(std::move(current_encoded_image),
                                          std::move(current_codec_specific_info));

    return webrtc::EncodedImageCallback::Result(webrtc::EncodedImageCallback::Result::OK);
}

void ArcasVideoEncoderWrapper::on_loss_notification(ArcasVideoEncoderLossNotification loss) const
{
    webrtc::VideoEncoder::LossNotification cxx_loss_notification;
    cxx_loss_notification.timestamp_of_last_decodable = loss.timestamp_of_last_decodable;
    cxx_loss_notification.timestamp_of_last_received  = loss.timestamp_of_last_received;

    if (loss.dependencies_of_last_received_decodable.size() > 0)
    {
        cxx_loss_notification.dependencies_of_last_received_decodable.emplace(
            loss.dependencies_of_last_received_decodable[0]);
    }

    if (loss.last_received_decodable.size() > 0)
    {
        cxx_loss_notification.last_received_decodable.emplace(loss.last_received_decodable[0]);
    }

    this->video_encoder_->OnLossNotification(cxx_loss_notification);
}

ArcasVideoEncoderInfo ArcasVideoEncoderWrapper::get_encoder_info() const
{
    auto                             info = video_encoder_->GetEncoderInfo();
    ArcasVideoEncoderScalingSettings scaling_settings;

    // XXX: we're unable to convert kOff into the struct here. We relay zero values in those cases.
    scaling_settings.min_pixels = info.scaling_settings.min_pixels_per_frame;

    if (info.scaling_settings.thresholds.has_value())
    {
        scaling_settings.thresholds.push_back(
            ArcasVideoEncoderQpThresholds{info.scaling_settings.thresholds.value().low,
                                          info.scaling_settings.thresholds.value().high});
    }

    rust::Vec<ArcasVideoEncoderInfoFPSAllocation> fps_allocation;

    // for (auto i = 0; i < webrtc::kMaxTemporalStreams; i++)
    // {
    //     ArcasVideoEncoderInfoFPSAllocation rust_fps_allocation;
    //     rust_fps_allocation.allocation.reserve(webrtc::kMaxSpatialLayers);

    //     for (auto j = 0; j < webrtc::kMaxSpatialLayers; j++)
    //     {
    //         rust_fps_allocation[i][j] = info.fps_allocation[i][j];
    //     }
    //     fps_allocation.push_back(rust_fps_allocation);
    // }

    rust::Vec<ArcasVideoEncoderResolutionBitrateLimits> resolution_bitrate_limits;

    for (auto resolution_bitrate_limit : info.resolution_bitrate_limits)
    {
        ArcasVideoEncoderResolutionBitrateLimits rust_resolution_bitrate_limit;
        rust_resolution_bitrate_limit.frame_size_pixels =
            resolution_bitrate_limit.frame_size_pixels;
        rust_resolution_bitrate_limit.max_bitrate_bps = resolution_bitrate_limit.max_bitrate_bps;
        rust_resolution_bitrate_limit.min_bitrate_bps = resolution_bitrate_limit.min_bitrate_bps;
        rust_resolution_bitrate_limit.min_start_bitrate_bps =
            resolution_bitrate_limit.min_start_bitrate_bps;
        resolution_bitrate_limits.push_back(rust_resolution_bitrate_limit);
    }

    rust::Vec<webrtc::VideoFrameBuffer::Type> preferred_pixel_formats;

    for (auto pixel_format : info.preferred_pixel_formats)
    {
        preferred_pixel_formats.push_back(pixel_format);
    }

    rust::Vec<uint8_t> is_qp_trusted;
    if (info.is_qp_trusted.has_value())
    {
        is_qp_trusted.push_back(info.is_qp_trusted.value() ? 1 : 0);
    }

    auto output = ArcasVideoEncoderInfo{
        .scaling_settings                        = scaling_settings,
        .requested_resolution_alignment          = info.requested_resolution_alignment,
        .apply_alignment_to_all_simulcast_layers = info.apply_alignment_to_all_simulcast_layers,
        .supports_native_handle                  = info.supports_native_handle,
        .implementation_name                     = rust::String(info.implementation_name.c_str()),
        .has_trusted_rate_controller             = info.has_trusted_rate_controller,
        .is_hardware_accelerated                 = info.is_hardware_accelerated,
        .has_internal_source                     = info.has_internal_source,
        .fps_allocation                          = fps_allocation,
        .supports_simulcast                      = info.supports_simulcast,
        .preferred_pixel_formats                 = preferred_pixel_formats,
        .is_qp_trusted                           = is_qp_trusted,
    };
    return output;
}

std::unique_ptr<ArcasVideoEncoderFactoryWrapper> create_arcas_video_encoder_factory_from_builtin()
{
    auto factory = webrtc::CreateBuiltinVideoEncoderFactory();
    return std::make_unique<ArcasVideoEncoderFactoryWrapper>(std::move(factory));
}

std::unique_ptr<ArcasCxxVideoEncoderEncoderInfo>
get_video_encoder_encoder_info(const webrtc::VideoEncoder& encoder)
{
    return std::make_unique<ArcasCxxVideoEncoderEncoderInfo>(encoder.GetEncoderInfo());
}
