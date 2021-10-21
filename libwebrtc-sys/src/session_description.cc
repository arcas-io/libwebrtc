#include "libwebrtc-sys/include/session_description.h"

ArcasSessionDescription::ArcasSessionDescription(
    std::unique_ptr<webrtc::SessionDescriptionInterface> api) : api(std::move(api)){};

rust::String ArcasSessionDescription::to_string() const
{
    std::string out;
    api->ToString(&out);
    return rust::String(out.c_str());
}

webrtc::SdpType ArcasSessionDescription::get_type() const
{
    return api->GetType();
}

std::unique_ptr<ArcasSessionDescription> ArcasSessionDescription::clone() const
{
    auto clone = api->Clone();
    return std::make_unique<ArcasSessionDescription>(std::move(clone));
}

std::unique_ptr<webrtc::SessionDescriptionInterface> ArcasSessionDescription::clone_sdp() const
{
    return api->Clone();
}