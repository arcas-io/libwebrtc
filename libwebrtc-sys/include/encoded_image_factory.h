#pragma once
#include "api/video/color_space.h"
#include "api/video/encoded_image.h"
#include "api/video/video_frame.h"

// Opaque to rust not to C++
class ArcasOpaqueEncodedImageBuffer
{
private:
    rtc::scoped_refptr<webrtc::EncodedImageBuffer> api_;

public:
    ArcasOpaqueEncodedImageBuffer(rtc::scoped_refptr<webrtc::EncodedImageBuffer> api)
    : api_(api)
    {
    }

    rtc::scoped_refptr<webrtc::EncodedImageBuffer> ref() const
    {
        return api_;
    }

    const rtc::scoped_refptr<webrtc::EncodedImageBuffer>& current_ref() const
    {
        return api_;
    }
};

/**
 * @brief Helper class to work with encoded images.
 *
 * This is needed primarily as a convience wrapper for the rust <> C++ ffi.
 */
class ArcasEncodedImageFactory
{
public:
    std::shared_ptr<ArcasOpaqueEncodedImageBuffer> create_encoded_image_buffer(const uint8_t* data,
                                                                               size_t size) const
    {
        auto api = webrtc::EncodedImageBuffer::Create(data, size);
        return std::make_shared<ArcasOpaqueEncodedImageBuffer>(api);
    }
    std::shared_ptr<ArcasOpaqueEncodedImageBuffer> create_empty_encoded_image_buffer() const
    {
        auto api = webrtc::EncodedImageBuffer::Create();
        return std::make_shared<ArcasOpaqueEncodedImageBuffer>(api);
    }

    std::unique_ptr<webrtc::EncodedImage>
    set_encoded_image_buffer(const webrtc::VideoFrame&             video_frame,
                             std::unique_ptr<webrtc::EncodedImage> image,
                             const ArcasOpaqueEncodedImageBuffer&  buffer) const
    {
        const webrtc::ColorSpace kColorSpace(webrtc::ColorSpace::PrimaryID::kBT709,
                                             webrtc::ColorSpace::TransferID::kBT709,
                                             webrtc::ColorSpace::MatrixID::kBT709,
                                             webrtc::ColorSpace::RangeID::kFull);
        image->_frameType = webrtc::VideoFrameType::kVideoFrameKey;
        image->SetColorSpace(kColorSpace);
        image->SetTimestamp(video_frame.timestamp());
        image->SetEncodedData(buffer.ref());
        return image;
    }

    std::unique_ptr<webrtc::EncodedImage> create_encoded_image() const
    {
        return std::make_unique<webrtc::EncodedImage>();
    }
};

std::unique_ptr<ArcasEncodedImageFactory> create_arcas_encoded_image_factory();
