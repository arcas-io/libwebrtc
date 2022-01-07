#pragma once
#include "libwebrtc-sys/include/video_frame_buffer_encoded.h"

class ArcasVideoFrameInternal : public webrtc::VideoFrame
{
private:
    std::unique_ptr<ArcasVideoFrameEncodedImageData> buffer_;

public:
    ArcasVideoFrameInternal(std::unique_ptr<ArcasVideoFrameEncodedImageData> buffer,
                            webrtc::VideoRotation rotation,
                            int64_t timestamp_us)
    : webrtc::VideoFrame(buffer->ref(), rotation, timestamp_us)
    , buffer_(std::move(buffer))
    {
    }

    ~ArcasVideoFrameInternal()
    {
        RTC_LOG(LS_INFO) << "~ArcasVideoFrameInternal";
    }
};
