#include "libwebrtc-sys/include/sdp_video_format.h"
#include "libwebrtc-sys/src/shared_bridge.rs.h"

rust::Vec<ArcasRustDict> sdp_video_format_get_parameters(const webrtc::SdpVideoFormat &format)
{
    rust::Vec<ArcasRustDict> list;

    for (auto param : format.parameters)
    {
        ArcasRustDict dict;
        dict.key = rust::String(param.first.c_str());
        dict.value = rust::String(param.second.c_str());
        list.push_back(dict);
    }
    return list;
}

const std::string &sdp_video_format_get_name(const webrtc::SdpVideoFormat &format)
{
    return format.name;
}

rust::String sdp_video_format_to_string(const webrtc::SdpVideoFormat &format)
{
    return rust::String(format.ToString().c_str());
}

std::unique_ptr<webrtc::SdpVideoFormat> create_sdp_video_format(ArcasSdpVideoFormatInit init)
{

    std::map<std::string, std::string> cxx_parameters;

    for (auto item : init.parameters)
    {
        cxx_parameters.insert(std::make_pair(std::string(item.key.c_str()), std::string(item.value.c_str())));
    }

    return std::make_unique<webrtc::SdpVideoFormat>(std::string(init.name.c_str()), cxx_parameters);
}

std::unique_ptr<std::vector<webrtc::SdpVideoFormat>> create_sdp_video_format_list(ArcasSdpVideoFormatVecInit list)
{
    auto output = std::make_unique<std::vector<webrtc::SdpVideoFormat>>();

    for (auto rust_sdp_video_format : list.list)
    {
        std::map<std::string, std::string> cxx_parameters;

        for (auto item : rust_sdp_video_format.parameters)
        {
            cxx_parameters.insert(std::make_pair(std::string(item.key.c_str()), std::string(item.value.c_str())));
        }
        output->push_back(webrtc::SdpVideoFormat(std::string(rust_sdp_video_format.name.c_str()), cxx_parameters));
    }

    return output;
}
