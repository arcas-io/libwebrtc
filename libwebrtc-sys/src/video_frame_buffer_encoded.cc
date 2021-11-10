#include "libwebrtc-sys/include/video_frame_buffer_encoded.h"
#include "api/video/i420_buffer.h"

std::unique_ptr<ArcasVideoFrameEncodedImageData> create_arcas_video_frame_buffer_from_encoded_image(const webrtc::EncodedImage &encodedImage, const ArcasCodecSpecificInfo &codec_info)
{
    auto info = codec_info.get_copy();
    return std::unique_ptr<ArcasVideoFrameEncodedImageData>(
        new ArcasVideoFrameEncodedImageData(
            new ArcasVideoFrameEncodedImageDataInternal(
                encodedImage, info)));
}

std::unique_ptr<ArcasVideoFrameRawImageData> create_arcas_video_frame_buffer_from_I420(
    int32_t width,
    int32_t height,
    const uint8_t *data)
{
    auto stride_y = width;
    auto stride_u = width / 2;
    auto stride_v = width / 2;
    int y_offset = 0, u_offset = stride_y * height;
    int v_offset = u_offset + stride_u * (height / 2);
    auto frame_buffer = webrtc::I420Buffer::Copy(
        width,
        height,
        data + y_offset,
        stride_y,
        data + u_offset,
        stride_u,
        data + v_offset,
        stride_v);

    return std::make_unique<ArcasVideoFrameRawImageData>(frame_buffer);
}
