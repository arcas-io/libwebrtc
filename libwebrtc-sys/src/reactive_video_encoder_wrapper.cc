#include "libwebrtc-sys/include/reactive_video_encoder_wrapper.h"
#include "libwebrtc-sys/src/shared_bridge.rs.h"
#include "libwebrtc-sys/src/video_encoding.rs.h"

void ArcasReactiveVideoEncoderWrapper::on_loss_notification(
    ArcasVideoEncoderLossNotification loss) const
{
    webrtc::VideoEncoder::LossNotification notification;

    if (loss.last_received_decodable.size() == 1)
    {
        notification.last_received_decodable.emplace(loss.last_received_decodable[0]);
    }

    if (loss.dependencies_of_last_received_decodable.size() == 1)
    {
        notification.dependencies_of_last_received_decodable.emplace(
            loss.dependencies_of_last_received_decodable[0]);
    }

    notification.timestamp_of_last_decodable = loss.timestamp_of_last_decodable;
    notification.timestamp_of_last_received  = loss.timestamp_of_last_received;

    video_encoder_->OnLossNotification(notification);
}

ArcasVideoEncoderInfo ArcasReactiveVideoEncoderWrapper::get_encoder_info() const
{
    ArcasVideoEncoderScalingSettings scaling_settings;
    auto                             info = video_encoder_->GetEncoderInfo();

    // XXX: we're unable to convert kOff into the struct here. We relay zero values in those cases.
    scaling_settings.min_pixels = info.scaling_settings.min_pixels_per_frame;

    if (info.scaling_settings.thresholds.has_value())
    {
        scaling_settings.thresholds.push_back(
            ArcasVideoEncoderQpThresholds{info.scaling_settings.thresholds.value().low,
                                          info.scaling_settings.thresholds.value().high});
    }

    rust::Vec<ArcasVideoEncoderInfoFPSAllocation> fps_allocation;
    fps_allocation.reserve(webrtc::kMaxTemporalStreams);

    for (auto i = 0; i < webrtc::kMaxTemporalStreams; i++)
    {
        ArcasVideoEncoderInfoFPSAllocation rust_fps_allocation;
        rust_fps_allocation.allocation.reserve(webrtc::kMaxSpatialLayers);

        for (auto j = 0; j < webrtc::kMaxSpatialLayers; j++)
        {
            rust_fps_allocation.allocation.push_back(info.fps_allocation[i][j]);
        }
        fps_allocation.push_back(rust_fps_allocation);
    }

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

    rust::Vec<ArcasCxxVideoFrameBufferType> preferred_pixel_formats;

    for (auto pixel_format : info.preferred_pixel_formats)
    {
        preferred_pixel_formats.push_back(pixel_format);
    }

    rust::Vec<uint8_t> is_qp_trusted;
    if (info.is_qp_trusted.has_value())
    {
        is_qp_trusted.push_back(info.is_qp_trusted.value() ? 1 : 0);
    }

    return ArcasVideoEncoderInfo{
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
}
