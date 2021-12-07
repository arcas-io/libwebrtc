#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/color_space.h"
#include "libwebrtc-sys/include/video_track_source.h"
#include "libwebrtc-sys/include/encoded_image_factory.h"
#include "libwebrtc-sys/include/video_frame_buffer_empty.h"

class ArcasVideoFrameFactory
{
private:
    std::unique_ptr<webrtc::VideoFrame::Builder> builder;

public:
    ArcasVideoFrameFactory() : builder(std::make_unique<webrtc::VideoFrame::Builder>()) {}
    void set_raw_video_frame_buffer(const ArcasVideoFrameRawImageData &buffer) const
    {
        builder->set_video_frame_buffer(buffer.current_ref());
    }

    void set_empty_video_frame_buffer() const
    {
        auto empty = rtc::make_ref_counted<ArcasVideoFrameBufferEmpty>();
        builder->set_video_frame_buffer(empty);
    }

    void set_encoded_video_frame_buffer(const ArcasVideoFrameEncodedImageData &buffer) const
    {
        builder->set_video_frame_buffer(buffer.current_ref());
    }

    void set_timestamp_ms(uint64_t timestamp_ms) const
    {
        // XXX: We may not want to implicitly conver the int types here from signed to unsigned.
        builder->set_timestamp_ms(timestamp_ms);
    }

    void set_timestamp_rtp(uint32_t timestamp_ms) const
    {
        // XXX: We may not want to implicitly conver the int types here from signed to unsigned.
        builder->set_timestamp_rtp(timestamp_ms);
    }

    void set_ntp_time_ms(int64_t ntp_time_ms) const
    {
        builder->set_ntp_time_ms(ntp_time_ms);
    }

    void set_color_space(const ArcasColorSpace &color_space) const
    {
        // takes a pointer but makes a copy of the contents...
        builder->set_color_space(color_space.as_ptr());
    }

    std::unique_ptr<webrtc::VideoFrame> build() const
    {
        return std::make_unique<webrtc::VideoFrame>(builder->build());
    }
};

std::unique_ptr<ArcasVideoFrameFactory> create_arcas_video_frame_factory();
