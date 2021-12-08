#include "libwebrtc-sys/src/lib.rs.h"

bool ArcasVideoDecoder::Configure(const webrtc::VideoDecoder::Settings& settings) {
    return true;
}

int32_t ArcasVideoDecoder::Decode(
    const webrtc::EncodedImage& image,
    bool missing_frames,
    int64_t render_times_ms
) {
    return api->decode(image, missing_frames, render_times_ms);
}

int32_t ArcasVideoDecoder::RegisterDecodeCompleteCallback(
    webrtc::DecodedImageCallback* cb
) {
    // TODO: support this callback
    /* api->register_decode_complete_callback(cb); */
    return 0;
}

int32_t ArcasVideoDecoder::Release() {
    return api->release();
}

int ArcasVideoDecoder::GetNumFramesReceived() const {
    return api->get_num_frames_received();
}
