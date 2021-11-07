#pragma once
#include "rust/cxx.h"
#include "libwebrtc-sys/include/webrtc_api.h"

class ArcasSpatialLayer
{
private:
    std::unique_ptr<webrtc::SpatialLayer> spatial_layer;

public:
    ArcasSpatialLayer() : spatial_layer(std::make_unique<webrtc::SpatialLayer>())
    {
    }
    void set_width(int width) const
    {
        spatial_layer->width = width;
    }

    void set_height(int height) const
    {
        spatial_layer->height = height;
    }

    void set_max_framerate(float max_framerate) const
    {
        spatial_layer->maxFramerate = max_framerate;
    }

    void set_number_of_temporal_layers(uint8_t number_of_temporal_layers) const
    {
        spatial_layer->numberOfTemporalLayers = number_of_temporal_layers;
    }

    void set_max_bitrate(unsigned int max_bitrate) const
    {
        spatial_layer->maxBitrate = max_bitrate;
    }

    void set_target_bitrate(unsigned int target_bitrate) const
    {
        spatial_layer->targetBitrate = target_bitrate;
    }

    void set_min_bitrate(unsigned int min_bitrate) const
    {
        spatial_layer->minBitrate = min_bitrate;
    }

    void set_qp_max(unsigned int qp_max) const
    {
        spatial_layer->qpMax = qp_max;
    }

    void set_active(bool active) const
    {
        spatial_layer->active = active;
    }

    webrtc::SpatialLayer get_copy() const
    {
        return *spatial_layer;
    }
};

class ArcasVideoCodec
{
private:
    std::unique_ptr<webrtc::VideoCodec> codec_;

public:
    ArcasVideoCodec() : codec_(std::make_unique<webrtc::VideoCodec>()) {}
    void set_scalability_mode(rust::String scalability_mode) const
    {
        codec_->SetScalabilityMode(scalability_mode.c_str());
    }

    void set_codec_type(webrtc::VideoCodecType codec_type) const
    {
        codec_->codecType = codec_type;
    }

    void set_width(uint16_t width) const
    {
        codec_->width = width;
    }
    void set_height(uint16_t height) const
    {
        codec_->height = height;
    }
    void set_max_bitrate(uint32_t max_bitrate) const
    {
        codec_->maxBitrate = max_bitrate;
    }
    void set_min_bitrate(uint32_t min_bitrate) const
    {
        codec_->minBitrate = min_bitrate;
    }
    void set_start_bitrate(uint32_t start_bitrate) const
    {
        codec_->startBitrate = start_bitrate;
    }
    void set_max_framerate(uint32_t max_framerate) const
    {
        codec_->maxFramerate = max_framerate;
    }
    void set_active(bool active) const
    {
        codec_->active = active;
    }

    void set_qp_max(unsigned int qp_max) const
    {
        codec_->qpMax = qp_max;
    }

    void set_number_of_simulcast_streams(unsigned char number_of_simulcast_streams) const
    {
        codec_->numberOfSimulcastStreams = number_of_simulcast_streams;
    }

    void set_simulcast_stream_at(uint8_t index, const ArcasSpatialLayer &layer) const
    {
        if (index < webrtc::kMaxSimulcastStreams)
        {
            codec_->simulcastStream[index] = layer.get_copy();
        }
        else
        {
            RTC_LOG(LS_ERROR) << "simulcast stream index out of bounds";
        }
    }

    void set_spatial_layer_at(uint8_t index, const ArcasSpatialLayer &layer) const
    {
        if (index < webrtc::kMaxSpatialLayers)
        {
            codec_->spatialLayers[index] = layer.get_copy();
        }
        else
        {
            RTC_LOG(LS_ERROR) << "simulcast stream index out of bounds";
        }
    }

    void set_mode(webrtc::VideoCodecMode mode) const
    {
        codec_->mode = mode;
    }

    void set_expect_encode_from_texture(bool expect_encode_from_texture) const
    {
        codec_->expect_encode_from_texture = expect_encode_from_texture;
    }

    void set_buffer_pool_size(int buffer_pool_size) const
    {
        codec_->buffer_pool_size = absl::optional<int>(buffer_pool_size);
    }

    void set_timing_frame_trigger_thresholds(int64_t delay_ms, uint16_t outlier_ratio_percent) const
    {
        codec_->timing_frame_thresholds = {delay_ms, outlier_ratio_percent};
    }

    void set_legacy_conference_mode(bool legacy_conference_mode) const
    {
        codec_->legacy_conference_mode = legacy_conference_mode;
    }

    void vp8_set_codec_complexity(webrtc::VideoCodecComplexity complexity) const
    {
        codec_->VP8()->complexity = complexity;
    }

    void vp8_set_number_of_temporal_layers(uint8_t number_of_temporal_layers) const
    {
        codec_->VP8()->numberOfTemporalLayers = number_of_temporal_layers;
    }

    void vp8_set_denoising_on(bool denoising_on) const
    {
        codec_->VP8()->denoisingOn = denoising_on;
    }

    void vp8_set_automatic_resize_on(bool automatic_resize_on) const
    {
        codec_->VP8()->automaticResizeOn = automatic_resize_on;
    }

    void vp8_set_frame_dropping_on(bool frame_dropping_on) const
    {
        codec_->VP8()->frameDroppingOn = frame_dropping_on;
    }

    void vp8_set_key_frame_interval(int key_frame_interval) const
    {
        codec_->VP8()->keyFrameInterval = key_frame_interval;
    }

    void vp9_set_codec_complexity(webrtc::VideoCodecComplexity complexity) const
    {
        codec_->VP9()->complexity = complexity;
    }

    void vp9_set_number_of_temporal_layers(uint8_t number_of_temporal_layers) const
    {
        codec_->VP9()->numberOfTemporalLayers = number_of_temporal_layers;
    }

    void vp9_set_denoising_on(bool denoising_on) const
    {
        codec_->VP9()->denoisingOn = denoising_on;
    }

    void vp9_set_frame_dropping_on(bool frame_dropping_on) const
    {
        codec_->VP9()->frameDroppingOn = frame_dropping_on;
    }

    void vp9_set_key_frame_interval(int key_frame_interval) const
    {
        codec_->VP9()->keyFrameInterval = key_frame_interval;
    }

    void vp9_set_adaptive_qp_on(bool adaptive_qp_on) const
    {
        codec_->VP9()->adaptiveQpMode = adaptive_qp_on;
    }

    void vp9_set_automatic_resize_on(bool automatic_resize_on) const
    {
        codec_->VP9()->automaticResizeOn = automatic_resize_on;
    }

    void vp9_set_number_of_spatial_layers(uint8_t number_of_spatial_layers) const
    {
        codec_->VP9()->numberOfSpatialLayers = number_of_spatial_layers;
    }

    void vp9_set_flexible_mode(bool flexible_mode) const
    {
        codec_->VP9()->flexibleMode = flexible_mode;
    }

    void vp9_set_inter_layer_pred(webrtc::InterLayerPredMode inter_layer_pred) const
    {
        codec_->VP9()->interLayerPred = inter_layer_pred;
    }

    void h264_set_frame_dropping_on(bool frame_dropping_on) const
    {
        codec_->H264()->frameDroppingOn = frame_dropping_on;
    }

    void h264_set_key_frame_interval(int key_frame_interval) const
    {
        codec_->H264()->keyFrameInterval = key_frame_interval;
    }

    void h264_set_number_of_temporal_layers(uint8_t number_of_temporal_layers) const
    {
        codec_->H264()->numberOfTemporalLayers = number_of_temporal_layers;
    }

    webrtc::VideoCodec *as_ptr() const
    {
        return codec_.get();
    }

    webrtc::VideoCodec get_copy()
    {
        return *codec_;
    }
};

class ArcasCodecSpecificInfo
{

private:
    std::unique_ptr<webrtc::CodecSpecificInfo> api;

public:
    ArcasCodecSpecificInfo() : api(std::make_unique<webrtc::CodecSpecificInfo>()) {}
    ArcasCodecSpecificInfo(const webrtc::CodecSpecificInfo &api) : api(std::make_unique<webrtc::CodecSpecificInfo>(api)) {}

    void set_codec_type(webrtc::VideoCodecType type) const
    {
        api->codecType = type;
    }

    webrtc::VideoCodecType get_codec_type() const
    {
        return this->api->codecType;
    }

    void set_end_of_picture(bool end_of_picture) const
    {
        this->api->end_of_picture = end_of_picture;
    }

    const webrtc::CodecSpecificInfo &as_ref() const
    {
        return *api.get();
    }

    const webrtc::CodecSpecificInfo *as_ptr() const
    {
        return api.get();
    }

    const webrtc::CodecSpecificInfo get_copy() const
    {
        return *api.get();
    }
};

std::unique_ptr<ArcasCodecSpecificInfo> create_arcas_codec_specific_info();
std::shared_ptr<ArcasVideoCodec> create_arcas_video_codec();
std::shared_ptr<ArcasSpatialLayer> create_arcas_spatial_layer();
