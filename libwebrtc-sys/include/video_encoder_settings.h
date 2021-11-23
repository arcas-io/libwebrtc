#pragma once
#include "rust/cxx.h"
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
