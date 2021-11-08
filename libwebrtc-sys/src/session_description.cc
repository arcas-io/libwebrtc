#include "libwebrtc-sys/include/session_description.h"
#include "libwebrtc-sys/src/lib.rs.h"

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

ArcasCreateSessionDescriptionResult create_arcas_session_description(webrtc::SdpType type, rust::String sdp)
{
    webrtc::SdpParseError error;
    ArcasCreateSessionDescriptionResult result;

    auto api = webrtc::CreateSessionDescription(type, sdp.c_str(), &error);
    if (error.line.size() > 0)
    {
        result.ok = false;
        result.error.line = rust::String(error.line.c_str());
        result.error.description = rust::String(error.description.c_str());
        return result;
    }
    else
    {
        result.ok = true;
        result.session = std::make_unique<ArcasSessionDescription>(std::move(api));
        return result;
    }
}