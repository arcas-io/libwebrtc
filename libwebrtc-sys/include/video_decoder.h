#pragma once

#include "api/video_codecs/video_decoder.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "rust/cxx.h"

class ArcasVideoDecoder: public webrtc::VideoDecoder {
    public:
    ArcasVideoDecoder(rust::Box<ArcasRustVideoDecoder> api): api(std::move(api)) {}

    virtual bool Configure(const webrtc::VideoDecoder::Settings&);

    virtual int32_t Decode(
        const webrtc::EncodedImage&,
        bool,
        int64_t
    );

    virtual int32_t RegisterDecodeCompleteCallback(
        webrtc::DecodedImageCallback*
    );

    virtual int32_t Release();

    int GetNumFramesReceived() const;

    private:
    rust::Box<ArcasRustVideoDecoder> api;
};
