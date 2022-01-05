#pragma once
#include "api/video/video_frame_buffer.h"
#include "libwebrtc-sys/include/codec_specific_info.h"
#include "libwebrtc-sys/include/video_codec.h"
#include "rtc_base/ref_counted_object.h"

class ArcasVideoFrameEncodedImageDataInternal : public rtc::RefCountedBase,
                                                public webrtc::VideoFrameBuffer
{
private:
    webrtc::EncodedImage      _encodedImage;
    webrtc::CodecSpecificInfo _codecSpecificInfo;

public:
    ArcasVideoFrameEncodedImageDataInternal(const webrtc::EncodedImage&      encodedImage,
                                            const webrtc::CodecSpecificInfo& codecSpecificInfo)
    : _encodedImage(encodedImage)
    , _codecSpecificInfo(codecSpecificInfo)
    {
    }

    ~ArcasVideoFrameEncodedImageDataInternal()
    {
        RTC_LOG(LS_INFO) << "~ArcasVideoFrameEncodedImageDataInternal";
    }

    webrtc::EncodedImage encoded_image() const
    {
        return _encodedImage;
    }

    webrtc::CodecSpecificInfo codec_specific_info() const
    {
        return _codecSpecificInfo;
    }

    const webrtc::EncodedImage& encoded_image_ref() const
    {
        return _encodedImage;
    }

    const webrtc::CodecSpecificInfo& codec_specific_info_ref() const
    {
        return _codecSpecificInfo;
    }

    std::unique_ptr<ArcasCodecSpecificInfo> arcas_codec_specific_info() const
    {
        return std::make_unique<ArcasCodecSpecificInfo>(_codecSpecificInfo);
    }

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
        return _encodedImage._encodedWidth;
    };
    int height() const override
    {
        return _encodedImage._encodedHeight;
    }

    const uint32_t size() const
    {
        return _encodedImage.size();
    }

    const uint8_t* data() const
    {
        return _encodedImage.data();
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
    rtc::scoped_refptr<VideoFrameBuffer> CropAndScale(int offset_x,
                                                      int offset_y,
                                                      int crop_width,
                                                      int crop_height,
                                                      int scaled_width,
                                                      int scaled_height) override
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

class ArcasVideoFrameEncodedImageData
{
private:
    rtc::scoped_refptr<webrtc::VideoFrameBuffer> api_;

public:
    // NOTE: This is a horrible hack/cast here we use this ptr to ArcasVideoFrameEncodedImageDataInternal supertype.
    ArcasVideoFrameEncodedImageData(rtc::scoped_refptr<webrtc::VideoFrameBuffer> api)
    : api_(api)
    {
    }
    ~ArcasVideoFrameEncodedImageData()
    {
        RTC_LOG(LS_INFO) << "~ArcasVideoFrameEncodedImageData";
    }

    int width() const
    {
        return api_->width();
    };
    int height() const
    {
        return api_->height();
    }

    uint32_t size() const
    {
        return static_cast<ArcasVideoFrameEncodedImageDataInternal*>(api_.get())->size();
    }

    const uint8_t* data() const
    {
        return static_cast<ArcasVideoFrameEncodedImageDataInternal*>(api_.get())->data();
    }

    const webrtc::EncodedImage& encoded_image_ref() const
    {
        auto ptr = static_cast<ArcasVideoFrameEncodedImageDataInternal*>(api_.get());
        return ptr->encoded_image_ref();
    }

    const webrtc::CodecSpecificInfo& codec_specific_info_ref() const
    {
        auto ptr = static_cast<ArcasVideoFrameEncodedImageDataInternal*>(api_.get());
        return ptr->codec_specific_info_ref();
    }

    std::unique_ptr<ArcasCodecSpecificInfo> arcas_codec_specific_info() const
    {
        auto ptr = static_cast<ArcasVideoFrameEncodedImageDataInternal*>(api_.get());
        return std::move(ptr->arcas_codec_specific_info());
    }

    const rtc::scoped_refptr<webrtc::VideoFrameBuffer>& current_ref() const
    {
        return api_;
    }

    rtc::scoped_refptr<webrtc::VideoFrameBuffer> ref() const
    {
        return api_;
    }

    rtc::scoped_refptr<webrtc::VideoFrameBuffer> internal_ref() const
    {
        return api_;
    }

    std::unique_ptr<ArcasVideoFrameEncodedImageData> clone() const
    {
        return std::unique_ptr<ArcasVideoFrameEncodedImageData>(
            new ArcasVideoFrameEncodedImageData(api_));
    }
};

class ArcasVideoFrameRawImageData
{
private:
    rtc::scoped_refptr<webrtc::VideoFrameBuffer> api_;

public:
    ArcasVideoFrameRawImageData(rtc::scoped_refptr<webrtc::VideoFrameBuffer> api)
    : api_(api)
    {
    }
    ~ArcasVideoFrameRawImageData()
    {
        RTC_LOG(LS_INFO) << "~ArcasVideoFrameEncodedImageData";
    }

    int width() const
    {
        return api_->width();
    };
    int height() const
    {
        return api_->height();
    }

    const rtc::scoped_refptr<webrtc::VideoFrameBuffer>& current_ref() const
    {
        return api_;
    }

    rtc::scoped_refptr<webrtc::VideoFrameBuffer> ref() const
    {
        return api_;
    }
};

std::unique_ptr<ArcasVideoFrameEncodedImageData>
create_arcas_video_frame_buffer_from_encoded_image(const webrtc::EncodedImage&   encodedImage,
                                                   const ArcasCodecSpecificInfo& codec_info);
std::unique_ptr<ArcasVideoFrameRawImageData>
create_arcas_video_frame_buffer_from_I420(int32_t width, int32_t height, const uint8_t* data);
