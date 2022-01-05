#pragma once

#include "api/video_codecs/video_decoder.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "rust/cxx.h"

class ArcasDecodedImageCallback
{
private:
    webrtc::DecodedImageCallback* cb;

public:
    ArcasDecodedImageCallback()
    : cb(nullptr)
    {
    }

    void SetCallback(webrtc::DecodedImageCallback* cb);

    int32_t decoded(webrtc::VideoFrame&) const;
};

int32_t decoded_image_callback_on_decoded(ArcasDecodedImageCallback&, webrtc::VideoFrame&);

class ArcasVideoDecoder : public webrtc::VideoDecoder
{
public:
    ArcasVideoDecoder(rust::Box<ArcasRustVideoDecoder> api)
    : api(std::move(api))
    {
    }

    virtual bool Configure(const webrtc::VideoDecoder::Settings&);

    virtual int32_t Decode(const webrtc::EncodedImage&, bool, int64_t);

    virtual int32_t RegisterDecodeCompleteCallback(webrtc::DecodedImageCallback*);

    virtual int32_t Release();

    int GetNumFramesReceived() const;

private:
    rust::Box<ArcasRustVideoDecoder> api;
    ArcasDecodedImageCallback        cb;
};
