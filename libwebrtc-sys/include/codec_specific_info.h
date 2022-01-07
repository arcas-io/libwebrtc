#pragma once
#include "api/video_codecs/video_codec.h"
#include "libwebrtc-sys/include/alias.h"
#include "modules/video_coding/include/video_codec_interface.h"
#include "rtc_base/logging.h"

class ArcasCodecSpecificInfo
{
private:
    std::unique_ptr<webrtc::CodecSpecificInfo> api;

public:
    ArcasCodecSpecificInfo()
    : api(std::make_unique<webrtc::CodecSpecificInfo>())
    {
    }
    ArcasCodecSpecificInfo(const webrtc::CodecSpecificInfo& api)
    : api(std::make_unique<webrtc::CodecSpecificInfo>(api))
    {
    }

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

    const webrtc::CodecSpecificInfo& as_ref() const
    {
        return *api.get();
    }

    const webrtc::CodecSpecificInfo* as_ptr() const
    {
        return api.get();
    }

    const webrtc::CodecSpecificInfo get_copy() const
    {
        return *api.get();
    }
};

std::unique_ptr<ArcasCodecSpecificInfo> create_arcas_codec_specific_info();
