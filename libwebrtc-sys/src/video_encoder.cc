#include "video_encoder.h"
#include "libwebrtc-sys/src/shared_bridge.rs.h"
#include "libwebrtc-sys/src/video_encoding.rs.h"

ArcasVideoEncoder::ArcasVideoEncoder(rust::Box<ArcasRustVideoEncoder> rust_encoder)
: api{std::move(rust_encoder)}
{
}
ArcasVideoEncoder::~ArcasVideoEncoder() noexcept {}

int32_t ArcasVideoEncoder::InitEncode(const webrtc::VideoCodec* codec_settings, int number_of_cores, size_t max_payload_size)
{
    return api->init_encode(codec_settings, number_of_cores, max_payload_size);
};

// Register an encode complete callback object.
//
// Input:
//          - callback         : Callback object which handles encoded images.
//
// Return value                : WEBRTC_VIDEO_CODEC_OK if OK, < 0 otherwise.
int32_t ArcasVideoEncoder::RegisterEncodeCompleteCallback(webrtc::EncodedImageCallback* callback)
{
    auto rust = std::make_unique<ArcasEncodedImageCallback>(callback);
    return api->register_encode_complete_callback(std::move(rust));
};

// Free encoder memory.
// Return value                : WEBRTC_VIDEO_CODEC_OK if OK, < 0 otherwise.
int32_t ArcasVideoEncoder::Release()
{
    return api->release();
};

// Encode an image (as a part of a video stream). The encoded image
// will be returned to the user through the encode complete callback.
//
// Input:
//          - frame             : Image to be encoded
//          - frame_types       : Frame type to be generated by the encoder.
//
// Return value                 : WEBRTC_VIDEO_CODEC_OK if OK
//                                <0 - Errors:
//                                  WEBRTC_VIDEO_CODEC_ERR_PARAMETER
//                                  WEBRTC_VIDEO_CODEC_MEMORY
//                                  WEBRTC_VIDEO_CODEC_ERROR
int32_t ArcasVideoEncoder::Encode(const webrtc::VideoFrame& frame, const std::vector<webrtc::VideoFrameType>* frame_types)
{
    return api->encode(frame, frame_types);
};

// Sets rate control parameters: bitrate, framerate, etc. These settings are
// instantaneous (i.e. not moving averages) and should apply from now until
// the next call to SetRates().
void ArcasVideoEncoder::SetRates(const RateControlParameters& parameters)
{
    auto rust_param = std::make_unique<ArcasVideoEncoderRateControlParameters>(parameters);
    api->set_rates(std::move(rust_param));
};

// Inform the encoder when the packet loss rate changes.
//
// Input:   - packet_loss_rate  : The packet loss rate (0.0 to 1.0).
void ArcasVideoEncoder::OnPacketLossRateUpdate(float packet_loss_rate)
{
    api->on_packet_loss_rate_update(packet_loss_rate);
};

// Inform the encoder when the round trip time changes.
//
// Input:   - rtt_ms            : The new RTT, in milliseconds.
void ArcasVideoEncoder::OnRttUpdate(int64_t rtt_ms)
{
    api->on_rtt_update(rtt_ms);
};

// Called when a loss notification is received.
void ArcasVideoEncoder::OnLossNotification(const LossNotification& loss_notification)
{
    // We can't pass back an optional (or a bool in a vec) so we use a vector of u8.
    rust::Vec<uint8_t> rust_deps;
    rust::Vec<uint8_t> rust_decodable;

    if (loss_notification.dependencies_of_last_received_decodable.has_value())
    {
        rust_deps.push_back(loss_notification.dependencies_of_last_received_decodable.value() ? 1 : 0);
    }

    if (loss_notification.last_received_decodable.has_value())
    {
        rust_decodable.push_back(loss_notification.last_received_decodable.value() ? 1 : 0);
    }

    ArcasVideoEncoderLossNotification rust_loss{
        loss_notification.timestamp_of_last_decodable,
        loss_notification.timestamp_of_last_received,
        rust_deps,
        rust_decodable,
    };
    api->on_loss_notification(rust_loss);
};

// Returns meta-data about the encoder, such as implementation name.
// The output of this method may change during runtime. For instance if a
// hardware encoder fails, it may fall back to doing software encoding using
// an implementation with different characteristics.
webrtc::VideoEncoder::EncoderInfo ArcasVideoEncoder::GetEncoderInfo() const
{
    webrtc::VideoEncoder::EncoderInfo info;
    auto rust_info = api->get_encoder_info();

    if (rust_info.scaling_settings.kOff)
    {
        info.scaling_settings = webrtc::VideoEncoder::ScalingSettings(webrtc::VideoEncoder::ScalingSettings::kOff);
    }
    else
    {
        webrtc::VideoEncoder::ScalingSettings scale_settings(rust_info.scaling_settings.low,
                                                             rust_info.scaling_settings.high,
                                                             rust_info.scaling_settings.min_pixels);
        info.scaling_settings = scale_settings;
    }

    info.requested_resolution_alignment = rust_info.requested_resolution_alignment;
    info.apply_alignment_to_all_simulcast_layers = rust_info.apply_alignment_to_all_simulcast_layers;
    info.supports_native_handle = rust_info.supports_native_handle;
    info.implementation_name = std::string(rust_info.implementation_name.c_str());
    info.has_trusted_rate_controller = rust_info.has_trusted_rate_controller;
    info.is_hardware_accelerated = rust_info.is_hardware_accelerated;
    //     info.has_internal_source = rust_info.has_internal_source;

    // FPS allocation code is turned off for now. The below implementation crashes.
    //
    // if (info.fps_allocation->size() > webrtc::kMaxTemporalStreams)
    // {
    //     RTC_LOG(LS_ERROR) << "FPS allocation vector is too big";
    // }
    // else
    // {

    //     for (auto i = 0; i < rust_info.fps_allocation.size(); i++)
    //     {
    //         auto allocation_size = rust_info.fps_allocation[i].allocation.size();
    //         if (allocation_size > webrtc::kMaxSpatialLayers)
    //         {
    //             RTC_LOG(LS_ERROR) << "FPS allocation vector is too big";
    //             continue;
    //         }
    //         for (auto j = 0; j < allocation_size; j++)
    //         {
    //             auto cxx_fps_allocation = &info.fps_allocation[i];
    //             cxx_fps_allocation[j].push_back(rust_info.fps_allocation[i].allocation[j]);
    //         };
    //     }
    // }

    std::vector<webrtc::VideoEncoder::ResolutionBitrateLimits> resolution_bitrate_limits;
    for (auto& limit : rust_info.resolution_bitrate_limits)
    {
        webrtc::VideoEncoder::ResolutionBitrateLimits new_limit(limit.frame_size_pixels,
                                                                limit.min_start_bitrate_bps,
                                                                limit.min_bitrate_bps,
                                                                limit.max_bitrate_bps);
        resolution_bitrate_limits.push_back(new_limit);
    }

    info.resolution_bitrate_limits = resolution_bitrate_limits;
    info.supports_simulcast = rust_info.supports_simulcast;

    absl::InlinedVector<webrtc::VideoFrameBuffer::Type, webrtc::kMaxPreferredPixelFormats> preferred_pixel_formats;

    for (auto pixel_format : rust_info.preferred_pixel_formats)
    {
        preferred_pixel_formats.push_back(static_cast<webrtc::VideoFrameBuffer::Type>(pixel_format));
    }

    info.preferred_pixel_formats = preferred_pixel_formats;

    if (rust_info.is_qp_trusted.size() > 0)
    {
        info.is_qp_trusted.emplace(true);
    }

    return info;
};

std::shared_ptr<ArcasVideoEncoderRateControlParameters>
create_arcas_video_encoder_rate_control_parameters(const ArcasCxxVideoBitrateAllocation& bitrate, double framerate_fps)
{
    return std::make_shared<ArcasVideoEncoderRateControlParameters>(bitrate, framerate_fps);
}

std::unique_ptr<ArcasCxxVideoBitrateAllocation> create_video_bitrate_allocation()
{
    return std::make_unique<ArcasCxxVideoBitrateAllocation>();
}
