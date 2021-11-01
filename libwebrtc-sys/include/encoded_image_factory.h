#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"

// Opaque to rust not to C++
class ArcasOpaqueEncodedImageBuffer
{
public:
    ArcasOpaqueEncodedImageBuffer(
        rtc::scoped_refptr<webrtc::EncodedImageBuffer> api) : api(api)
    {
    }
    rtc::scoped_refptr<webrtc::EncodedImageBuffer> api;

    rtc::scoped_refptr<webrtc::EncodedImageBuffer> ref()
    {
        return api;
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
    std::shared_ptr<ArcasOpaqueEncodedImageBuffer> create_encoded_image_buffer(const uint8_t *data, size_t size) const
    {
        auto api = webrtc::EncodedImageBuffer::Create(data, size);
        return std::make_shared<ArcasOpaqueEncodedImageBuffer>(api);
    }
    std::shared_ptr<ArcasOpaqueEncodedImageBuffer> create_empty_encoded_image_buffer() const
    {
        auto api = webrtc::EncodedImageBuffer::Create();
        return std::make_shared<ArcasOpaqueEncodedImageBuffer>(api);
    }

    std::unique_ptr<webrtc::EncodedImage> set_encoded_image_buffer(
        std::unique_ptr<webrtc::EncodedImage> image,
        std::shared_ptr<ArcasOpaqueEncodedImageBuffer> buffer) const
    {
        image->SetEncodedData(buffer->ref());
        return image;
    }

    std::unique_ptr<webrtc::EncodedImage> create_encoded_image() const
    {
        return std::make_unique<webrtc::EncodedImage>();
    }
};

std::unique_ptr<ArcasEncodedImageFactory> create_arcas_encoded_image_factory();
