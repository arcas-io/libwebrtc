#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"

class ArcasCodecSpecificInfo
{

private:
    webrtc::CodecSpecificInfo api;

public:
    ArcasCodecSpecificInfo(){};

    const webrtc::CodecSpecificInfo *ref() const
    {
        return &this->api;
    }

    void set_codec_type(webrtc::VideoCodecType type)
    {
        this->api.codecType = type;
    }

    webrtc::VideoCodecType get_codec_type() const
    {
        return this->api.codecType;
    }

    void set_end_of_picture(bool end_of_picture)
    {
        this->api.end_of_picture = end_of_picture;
    }
};

std::unique_ptr<ArcasCodecSpecificInfo> create_arcas_codec_specific_info();