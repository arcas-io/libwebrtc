#pragma once
#include "api/ref_counted_base.h"
#include "api/video/video_frame_buffer.h"
#include "rtc_base/logging.h"

class ArcasVideoFrameBufferEmpty : public rtc::RefCountedBase, public webrtc::VideoFrameBuffer
{
public:
    ArcasVideoFrameBufferEmpty() {}

    virtual void AddRef() const override
    {
        rtc::RefCountedBase::AddRef();
    }
    virtual rtc::RefCountReleaseStatus Release() const override
    {
        return rtc::RefCountedBase::Release();
    }

    webrtc::VideoFrameBuffer::Type type() const override
    {
        return webrtc::VideoFrameBuffer::Type::kNative;
    }

    // The resolution of the frame in pixels. For formats where some planes are
    // subsampled, this is the highest-resolution plane.
    int width() const override
    {
        return 720;
    };

    int height() const override
    {
        return 360;
    }

    uint32_t size() const
    {
        return 100;
    }

    const uint8_t* data() const
    {
        return nullptr;
    }

    // Returns a memory-backed frame buffer in I420 format. If the pixel data is
    // in another format, a conversion will take place. All implementations must
    // provide a fallback to I420 for compatibility with e.g. the internal WebRTC
    // software encoders.
    virtual rtc::scoped_refptr<webrtc::I420BufferInterface> ToI420() override
    {
        RTC_LOG(LS_ERROR) << "Not implemented ToI420 \n";
        return nullptr;
    };

    // GetI420() methods should return I420 buffer if conversion is trivial, i.e
    // no change for binary data is needed. Otherwise these methods should return
    // nullptr. One example of buffer with that property is
    // WebrtcVideoFrameAdapter in Chrome - it's I420 buffer backed by a shared
    // memory buffer. Therefore it must have type kNative. Yet, ToI420()
    // doesn't affect binary data at all. Another example is any I420A buffer.
    // TODO(https://crbug.com/webrtc/12021): Make this method non-virtual and
    // behave as the other GetXXX methods below.
    const webrtc::I420BufferInterface* GetI420()
    {
        RTC_LOG(LS_ERROR) << "Not implemented GetI420 \n";
        return nullptr;
    };

    // A format specific scale function. Default implementation works by
    // converting to I420. But more efficient implementations may override it,
    // especially for kNative.
    // First, the image is cropped to `crop_width` and `crop_height` and then
    // scaled to `scaled_width` and `scaled_height`.
    rtc::scoped_refptr<VideoFrameBuffer>
    CropAndScale(int offset_x, int offset_y, int crop_width, int crop_height, int scaled_width, int scaled_height) override
    {
        RTC_LOG(LS_ERROR) << "Not implemented Crop & Scale \n";
        return nullptr;
    }

    // From a kNative frame, returns a VideoFrameBuffer with a pixel format in
    // the list of types that is in the main memory with a pixel perfect
    // conversion for encoding with a software encoder. Returns nullptr if the
    // frame type is not supported, mapping is not possible, or if the kNative
    // frame has not implemented this method. Only callable if type() is kNative.
    rtc::scoped_refptr<VideoFrameBuffer> GetMappedFrameBuffer(rtc::ArrayView<Type> types) override
    {
        RTC_LOG(LS_ERROR) << "Not implemented Get Mapped Frame Buffer \n";
        return nullptr;
    }
};
