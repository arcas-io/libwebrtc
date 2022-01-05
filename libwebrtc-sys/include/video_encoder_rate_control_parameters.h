#pragma once
#include "api/video_codecs/video_encoder.h"
#include "libwebrtc-sys/include/alias.h"

class ArcasVideoEncoderRateControlParameters
{
private:
    webrtc::VideoEncoder::RateControlParameters api;

public:
    ArcasVideoEncoderRateControlParameters(const webrtc::VideoEncoder::RateControlParameters& api)
    : api(api)
    {
    }
    ArcasVideoEncoderRateControlParameters(const ArcasCxxVideoBitrateAllocation& bitrate,
                                           double                                framerate_fps)
    : api(webrtc::VideoEncoder::RateControlParameters(bitrate, framerate_fps))
    {
    }
    ArcasVideoEncoderRateControlParameters(const ArcasCxxVideoBitrateAllocation& bitrate,
                                           double                                framerate_fps,
                                           std::unique_ptr<webrtc::DataRate>     data_rate)
    : api(webrtc::VideoEncoder::RateControlParameters(bitrate, framerate_fps, *data_rate))
    {
    }

    const webrtc::VideoBitrateAllocation& get_bitrate() const
    {
        return api.bitrate;
    }

    const webrtc::VideoBitrateAllocation& get_target_bitrate() const
    {
        return api.target_bitrate;
    }

    double get_framerate_fps() const
    {
        return api.framerate_fps;
    }

    int64_t get_bytes_per_second() const
    {
        return api.bandwidth_allocation.bps();
    }

    const webrtc::VideoEncoder::RateControlParameters& as_ref() const
    {
        return api;
    }
};
