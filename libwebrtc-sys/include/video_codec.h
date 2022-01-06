#pragma once
#include "api/video_codecs/spatial_layer.h"
#include "api/video_codecs/video_codec.h"
#include "libwebrtc-sys/include/codec_specific_info.h"
#include "libwebrtc-sys/include/video_frame_buffer_encoded.h"
#include "modules/video_coding/include/video_codec_interface.h"
#include "rtc_base/logging.h"
#include "rust/cxx.h"

class ArcasSpatialLayerInternal
{
public:
    webrtc::SpatialLayer value;
    ArcasSpatialLayerInternal()
    : value()
    {
    }
    ArcasSpatialLayerInternal(const webrtc::SpatialLayer value)
    : value(value)
    {
    }
    ArcasSpatialLayerInternal(const webrtc::SpatialLayer* value)
    : value(*value)
    {
    }
};

class ArcasSpatialLayer
{
private:
    std::unique_ptr<ArcasSpatialLayerInternal> spatial_layer;

public:
    ArcasSpatialLayer(const webrtc::SpatialLayer& layer)
    : spatial_layer(std::make_unique<ArcasSpatialLayerInternal>(layer))
    {
    }
    ArcasSpatialLayer()
    : spatial_layer(std::make_unique<ArcasSpatialLayerInternal>())
    {
    }

    int get_width() const
    {
        return spatial_layer->value.width;
    }

    int get_height() const
    {
        return spatial_layer->value.height;
    }

    float get_max_framerate() const
    {
        return spatial_layer->value.maxFramerate;
    }

    uint8_t get_number_of_temporal_layers() const
    {
        return spatial_layer->value.numberOfTemporalLayers;
    }

    unsigned int get_max_bitrate() const
    {
        return spatial_layer->value.maxBitrate;
    }

    unsigned int get_target_bitrate() const
    {
        return spatial_layer->value.targetBitrate;
    }

    unsigned int get_min_bitrate() const
    {
        return spatial_layer->value.minBitrate;
    }

    unsigned int get_qp_max() const
    {
        return spatial_layer->value.qpMax;
    }

    bool get_active() const
    {
        return spatial_layer->value.active;
    }

    void set_width(int width) const
    {
        spatial_layer->value.width = width;
    }

    void set_height(int height) const
    {
        spatial_layer->value.height = height;
    }

    void set_max_framerate(float max_framerate) const
    {
        spatial_layer->value.maxFramerate = max_framerate;
    }

    void set_number_of_temporal_layers(uint8_t number_of_temporal_layers) const
    {
        spatial_layer->value.numberOfTemporalLayers = number_of_temporal_layers;
    }

    void set_max_bitrate(unsigned int max_bitrate) const
    {
        spatial_layer->value.maxBitrate = max_bitrate;
    }

    void set_target_bitrate(unsigned int target_bitrate) const
    {
        spatial_layer->value.targetBitrate = target_bitrate;
    }

    void set_min_bitrate(unsigned int min_bitrate) const
    {
        spatial_layer->value.minBitrate = min_bitrate;
    }

    void set_qp_max(unsigned int qp_max) const
    {
        spatial_layer->value.qpMax = qp_max;
    }

    void set_active(bool active) const
    {
        spatial_layer->value.active = active;
    }

    webrtc::SpatialLayer get_copy() const
    {
        return spatial_layer->value;
    }
};

class ArcasVideoCodecInternal
{
public:
    webrtc::VideoCodec codec;
    ArcasVideoCodecInternal()
    : codec()
    {
    }
    ArcasVideoCodecInternal(const webrtc::VideoCodec& codec)
    : codec(codec)
    {
    }
    ArcasVideoCodecInternal(const webrtc::VideoCodec* codec)
    : codec(*codec)
    {
    }

    const webrtc::VideoCodec* as_ptr() const
    {
        return &codec;
    }

    webrtc::VideoCodec get_copy()
    {
        return codec;
    }
};

class ArcasVideoCodec
{
private:
    std::unique_ptr<ArcasVideoCodecInternal> codec_;

public:
    ArcasVideoCodec()
    : codec_(std::make_unique<ArcasVideoCodecInternal>())
    {
    }
    ArcasVideoCodec(const webrtc::VideoCodec& codec)
    : codec_(std::make_unique<ArcasVideoCodecInternal>(codec))
    {
    }
    ArcasVideoCodec(const webrtc::VideoCodec* codec)
    : codec_(std::make_unique<ArcasVideoCodecInternal>(codec))
    {
    }

    rust::String get_scalability_mode() const
    {
        return rust::String(std::string(codec_->codec.ScalabilityMode()).c_str());
    }

    webrtc::VideoCodecType get_codec_type() const
    {
        return codec_->codec.codecType;
    }

    int get_width() const
    {
        return codec_->codec.width;
    }

    int get_height() const
    {
        return codec_->codec.height;
    }

    uint32_t get_max_bitrate() const
    {
        return codec_->codec.maxBitrate;
    }

    uint32_t get_min_bitrate() const
    {
        return codec_->codec.minBitrate;
    }

    uint32_t get_start_bitrate() const
    {
        return codec_->codec.startBitrate;
    }

    uint32_t get_max_framerate() const
    {
        return codec_->codec.maxFramerate;
    }

    bool get_active() const
    {
        return codec_->codec.active;
    }

    unsigned int get_qp_max() const
    {
        return codec_->codec.qpMax;
    }

    unsigned char get_number_of_simulcast_streams() const
    {
        return codec_->codec.numberOfSimulcastStreams;
    }

    void set_scalability_mode(rust::String scalability_mode) const
    {
        codec_->codec.SetScalabilityMode(scalability_mode.c_str());
    }

    void set_codec_type(webrtc::VideoCodecType codec_type) const
    {
        codec_->codec.codecType = codec_type;
    }

    void set_width(uint16_t width) const
    {
        codec_->codec.width = width;
    }
    void set_height(uint16_t height) const
    {
        codec_->codec.height = height;
    }
    void set_max_bitrate(uint32_t max_bitrate) const
    {
        codec_->codec.maxBitrate = max_bitrate;
    }
    void set_min_bitrate(uint32_t min_bitrate) const
    {
        codec_->codec.minBitrate = min_bitrate;
    }
    void set_start_bitrate(uint32_t start_bitrate) const
    {
        codec_->codec.startBitrate = start_bitrate;
    }
    void set_max_framerate(uint32_t max_framerate) const
    {
        codec_->codec.maxFramerate = max_framerate;
    }
    void set_active(bool active) const
    {
        codec_->codec.active = active;
    }

    void set_qp_max(unsigned int qp_max) const
    {
        codec_->codec.qpMax = qp_max;
    }

    void set_number_of_simulcast_streams(unsigned char number_of_simulcast_streams) const
    {
        codec_->codec.numberOfSimulcastStreams = number_of_simulcast_streams;
    }

    void set_simulcast_stream_at(uint8_t index, const ArcasSpatialLayer& layer) const
    {
        if (index < webrtc::kMaxSimulcastStreams)
        {
            codec_->codec.simulcastStream[index] = layer.get_copy();
        }
        else
        {
            RTC_LOG(LS_ERROR) << "simulcast stream index out of bounds";
        }
    }

    void set_spatial_layer_at(uint8_t index, const ArcasSpatialLayer& layer) const
    {
        if (index < webrtc::kMaxSpatialLayers)
        {
            codec_->codec.spatialLayers[index] = layer.get_copy();
        }
        else
        {
            RTC_LOG(LS_ERROR) << "simulcast stream index out of bounds";
        }
    }

    void set_mode(webrtc::VideoCodecMode mode) const
    {
        codec_->codec.mode = mode;
    }

    void set_expect_encode_from_texture(bool expect_encode_from_texture) const
    {
        codec_->codec.expect_encode_from_texture = expect_encode_from_texture;
    }

    void set_buffer_pool_size(int buffer_pool_size) const
    {
        codec_->codec.buffer_pool_size = absl::optional<int>(buffer_pool_size);
    }

    void set_timing_frame_trigger_thresholds(int64_t delay_ms, uint16_t outlier_ratio_percent) const
    {
        codec_->codec.timing_frame_thresholds = {delay_ms, outlier_ratio_percent};
    }

    void set_legacy_conference_mode(bool legacy_conference_mode) const
    {
        codec_->codec.legacy_conference_mode = legacy_conference_mode;
    }

    void vp8_set_codec_complexity(webrtc::VideoCodecComplexity complexity) const
    {
        codec_->codec.VP8()->complexity = complexity;
    }

    void vp8_set_number_of_temporal_layers(uint8_t number_of_temporal_layers) const
    {
        codec_->codec.VP8()->numberOfTemporalLayers = number_of_temporal_layers;
    }

    void vp8_set_denoising_on(bool denoising_on) const
    {
        codec_->codec.VP8()->denoisingOn = denoising_on;
    }

    void vp8_set_automatic_resize_on(bool automatic_resize_on) const
    {
        codec_->codec.VP8()->automaticResizeOn = automatic_resize_on;
    }

    void vp8_set_frame_dropping_on(bool frame_dropping_on) const
    {
        codec_->codec.VP8()->frameDroppingOn = frame_dropping_on;
    }

    void vp8_set_key_frame_interval(int key_frame_interval) const
    {
        codec_->codec.VP8()->keyFrameInterval = key_frame_interval;
    }

    void vp9_set_codec_complexity(webrtc::VideoCodecComplexity complexity) const
    {
        codec_->codec.VP9()->complexity = complexity;
    }

    void vp9_set_number_of_temporal_layers(uint8_t number_of_temporal_layers) const
    {
        codec_->codec.VP9()->numberOfTemporalLayers = number_of_temporal_layers;
    }

    void vp9_set_denoising_on(bool denoising_on) const
    {
        codec_->codec.VP9()->denoisingOn = denoising_on;
    }

    void vp9_set_frame_dropping_on(bool frame_dropping_on) const
    {
        codec_->codec.VP9()->frameDroppingOn = frame_dropping_on;
    }

    void vp9_set_key_frame_interval(int key_frame_interval) const
    {
        codec_->codec.VP9()->keyFrameInterval = key_frame_interval;
    }

    void vp9_set_adaptive_qp_on(bool adaptive_qp_on) const
    {
        codec_->codec.VP9()->adaptiveQpMode = adaptive_qp_on;
    }

    void vp9_set_automatic_resize_on(bool automatic_resize_on) const
    {
        codec_->codec.VP9()->automaticResizeOn = automatic_resize_on;
    }

    void vp9_set_number_of_spatial_layers(uint8_t number_of_spatial_layers) const
    {
        codec_->codec.VP9()->numberOfSpatialLayers = number_of_spatial_layers;
    }

    void vp9_set_flexible_mode(bool flexible_mode) const
    {
        codec_->codec.VP9()->flexibleMode = flexible_mode;
    }

    void vp9_set_inter_layer_pred(webrtc::InterLayerPredMode inter_layer_pred) const
    {
        codec_->codec.VP9()->interLayerPred = inter_layer_pred;
    }

    void h264_set_frame_dropping_on(bool frame_dropping_on) const
    {
        codec_->codec.H264()->frameDroppingOn = frame_dropping_on;
    }

    void h264_set_key_frame_interval(int key_frame_interval) const
    {
        codec_->codec.H264()->keyFrameInterval = key_frame_interval;
    }

    void h264_set_number_of_temporal_layers(uint8_t number_of_temporal_layers) const
    {
        codec_->codec.H264()->numberOfTemporalLayers = number_of_temporal_layers;
    }

    const webrtc::VideoCodec* as_ptr() const
    {
        return codec_->as_ptr();
    }

    webrtc::VideoCodec get_copy()
    {
        return codec_->get_copy();
    }

    std::unique_ptr<ArcasVideoCodec> cxx_clone() const
    {
        return std::make_unique<ArcasVideoCodec>(this->as_ptr());
    }

    std::unique_ptr<std::vector<ArcasSpatialLayer>> spatial_layers() const
    {
        auto result = std::make_unique<std::vector<ArcasSpatialLayer>>();
        for (auto& layer : codec_->codec.spatialLayers) { result->push_back(layer); }
        return result;
    }

    std::unique_ptr<std::vector<ArcasSpatialLayer>> simulcast_streams() const
    {
        auto result = std::make_unique<std::vector<ArcasSpatialLayer>>();
        for (auto& layer : codec_->codec.simulcastStream) { result->push_back(layer); }
        return result;
    }

    // Generate a deterministic id such that two codecs with same parameters
    // have same identifier.
    //
    // XXX: This is an approximation there are many variables and we only need a
    // certain degree of uniqueness.
    rust::String id()
    {
        std::stringstream output;
        auto type_output = CodecTypeToPayloadString(codec_->codec.codecType);
        auto width = codec_->codec.width;
        auto height = codec_->codec.height;
        auto max_bitrate = codec_->codec.maxBitrate;
        auto min_bitrate = codec_->codec.minBitrate;
        auto start_bitrate = codec_->codec.startBitrate;
        auto max_framerate = codec_->codec.maxFramerate;
        auto simulcast_streams = codec_->codec.numberOfSimulcastStreams;
        auto mode =
            codec_->codec.mode == webrtc::VideoCodecMode::kRealtimeVideo ? "realtime" : "screen";
        auto keyframe_interval = 0;
        auto number_of_temporal_layers = 0;

        switch (codec_->codec.codecType)
        {
        case webrtc::VideoCodecType::kVideoCodecVP8:
            keyframe_interval = codec_->codec.VP8()->keyFrameInterval;
            number_of_temporal_layers = codec_->codec.VP8()->numberOfTemporalLayers;
            break;
        case webrtc::VideoCodecType::kVideoCodecVP9:
            keyframe_interval = codec_->codec.VP9()->keyFrameInterval;
            number_of_temporal_layers = codec_->codec.VP9()->numberOfTemporalLayers;
            break;
        case webrtc::VideoCodecType::kVideoCodecH264:
            keyframe_interval = codec_->codec.H264()->keyFrameInterval;
            number_of_temporal_layers = codec_->codec.H264()->numberOfTemporalLayers;
            break;
        default:
            // we only care about the above codecs.
            break;
        }

        output << "type=" << type_output << "&width=" << width << "&height=" << height
               << "&max_bitrate=" << max_bitrate << "&min_bitrate=" << min_bitrate
               << "&start_bitrate=" << start_bitrate << "&max_framerate=" << max_framerate
               << "&simulcast_streams=" << simulcast_streams << "&mode=" << mode
               << "&keyframe_interval=" << keyframe_interval
               << "&number_of_temporal_layers=" << number_of_temporal_layers;

        return rust::String(output.str().c_str());
    }
};

std::unique_ptr<ArcasVideoCodec> create_arcas_video_codec_from_cxx(const webrtc::VideoCodec* codec);
std::shared_ptr<ArcasVideoCodec> create_arcas_video_codec();
std::shared_ptr<ArcasSpatialLayer> create_arcas_spatial_layer();
std::unique_ptr<std::vector<ArcasSpatialLayer>> gen_unique_vector_spatial_layers();
std::shared_ptr<ArcasVideoCodec> gen_shared_video_codec();
