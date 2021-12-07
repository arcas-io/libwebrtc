#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "libwebrtc-sys/include/video_codec.h"

class ArcasEncodedImageCallback
{
private:
    // very unsafe but fast. We rely on the caller to keep the callback alive.
    webrtc::EncodedImageCallback *api;

public:
    ArcasEncodedImageCallback(webrtc::EncodedImageCallback *api) : api(api)
    {
        RTC_LOG(LS_INFO) << "ArcasEncodedImageCallback::ArcasEncodedImageCallback ptr=" << api;
    }
    ArcasEncodedImageCallbackResult on_encoded_image(const webrtc::EncodedImage &image, const ArcasCodecSpecificInfo *codec_specific_info) const;
    void on_dropped_frame(webrtc::EncodedImageCallback::DropReason reason)
    {
        api->OnDroppedFrame(reason);
    }

    webrtc::EncodedImageCallback *get() const
    {
        return api;
    }
};
