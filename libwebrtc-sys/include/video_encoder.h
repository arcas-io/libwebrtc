#pragma once
#include "alias.h"
#include "encoded_image_factory.h"
#include "rust/cxx.h"
#include "rust_shared.h"
#include "video_codec.h"
#include "video_encoder_rate_control_parameters.h"

using WebRTCVideoEncoder = webrtc::VideoEncoder;

class ArcasVideoCodecSettings
{
private:
    webrtc::VideoEncoder::Settings api;

public:
    webrtc::VideoEncoder::Settings get()
    {
        return api;
    }
};

class ArcasVideoEncoder : public WebRTCVideoEncoder
{
private:
    rust::Box<ArcasRustVideoEncoder> api;

public:
    ArcasVideoEncoder(rust::Box<ArcasRustVideoEncoder> api);
    ~ArcasVideoEncoder();

    void SetFecControllerOverride(webrtc::FecControllerOverride* fec_controller_override) override
    {
        // TODO: Implement bindings.
    }
    // Initialize the encoder with the information from the codecSettings
    //
    // Input:
    //          - codec_settings    : Codec settings
    //          - settings          : Settings affecting the encoding itself.
    // Input for deprecated version:
    //          - number_of_cores   : Number of cores available for the encoder
    //          - max_payload_size  : The maximum size each payload is allowed
    //                                to have. Usually MTU - overhead.
    //
    // Return value                  : Set bit rate if OK
    //                                 <0 - Errors:
    //                                  WEBRTC_VIDEO_CODEC_ERR_PARAMETER
    //                                  WEBRTC_VIDEO_CODEC_ERR_SIZE
    //                                  WEBRTC_VIDEO_CODEC_MEMORY
    //                                  WEBRTC_VIDEO_CODEC_ERROR
    // TODO(bugs.webrtc.org/10720): After updating downstream projects and posting
    // an announcement to discuss-webrtc, remove the three-parameters variant
    // and make the two-parameters variant pure-virtual.
    int InitEncode(const webrtc::VideoCodec* codec_settings, int number_of_cores, size_t max_payload_size) override;
    // Register an encode complete callback object.
    //
    // Input:
    //          - callback         : Callback object which handles encoded images.
    //
    // Return value                : WEBRTC_VIDEO_CODEC_OK if OK, < 0 otherwise.
    int32_t RegisterEncodeCompleteCallback(webrtc::EncodedImageCallback* callback) override;

    // Free encoder memory.
    // Return value                : WEBRTC_VIDEO_CODEC_OK if OK, < 0 otherwise.
    int32_t Release() override;

    // Encode an image (as a part of a video stream). The encoded image
    // will be returned to the user through the encode complete callback.
    //
    // Input:
    //          - frame             : Image to be encoded
    //          - frame_types       : Frame type to be generated by the encoder.
    //
    // Return value                 : WEBRTC_VIDEO_CODEC_OK if OK
    //                                <0 - Errors:
    //                                  WEBRTC_VIDEO_CODEC_ERR_PARAMETER
    //                                  WEBRTC_VIDEO_CODEC_MEMORY
    //                                  WEBRTC_VIDEO_CODEC_ERROR
    int32_t Encode(const webrtc::VideoFrame& frame, const std::vector<webrtc::VideoFrameType>* frame_types) override;

    // Sets rate control parameters: bitrate, framerate, etc. These settings are
    // instantaneous (i.e. not moving averages) and should apply from now until
    // the next call to SetRates().
    void SetRates(const RateControlParameters& parameters) override;

    // Inform the encoder when the packet loss rate changes.
    //
    // Input:   - packet_loss_rate  : The packet loss rate (0.0 to 1.0).
    void OnPacketLossRateUpdate(float packet_loss_rate) override;

    // Inform the encoder when the round trip time changes.
    //
    // Input:   - rtt_ms            : The new RTT, in milliseconds.
    void OnRttUpdate(int64_t rtt_ms) override;

    // Called when a loss notification is received.
    void OnLossNotification(const LossNotification& loss_notification) override;

    // Returns meta-data about the encoder, such as implementation name.
    // The output of this method may change during runtime. For instance if a
    // hardware encoder fails, it may fall back to doing software encoding using
    // an implementation with different characteristics.
    EncoderInfo GetEncoderInfo() const override;
};

std::unique_ptr<ArcasCxxVideoBitrateAllocation> create_video_bitrate_allocation();
std::shared_ptr<ArcasVideoEncoderRateControlParameters>
create_arcas_video_encoder_rate_control_parameters(const ArcasCxxVideoBitrateAllocation& bitrate, double framerate_fps);
