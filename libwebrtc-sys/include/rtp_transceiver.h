#pragma once
#include "libwebrtc-sys/include/webrtc_api.h"
#include "libwebrtc-sys/include/rtp_parameters.h"
#include "libwebrtc-sys/include/rtp_receiver.h"
#include "libwebrtc-sys/include/rtp_sender.h"
#include "libwebrtc-sys/include/error.h"
#include "rust/cxx.h"

class ArcasRTPTransceiver;
class ArcasRTPVideoTransceiver;
class ArcasRTPAudioTransceiver;

std::unique_ptr<ArcasRTPVideoTransceiver> video_transceiver_from_base(const ArcasRTPTransceiver&);
std::unique_ptr<ArcasRTPAudioTransceiver> audio_transceiver_from_base(const ArcasRTPTransceiver&);
class ArcasRTPTransceiver
{
    friend std::unique_ptr<ArcasRTPVideoTransceiver> video_transceiver_from_base(const ArcasRTPTransceiver&);
    friend std::unique_ptr<ArcasRTPAudioTransceiver> audio_transceiver_from_base(const ArcasRTPTransceiver&);
protected:
    rtc::scoped_refptr<webrtc::RtpTransceiverInterface> api;

public:
    ArcasRTPTransceiver(rtc::scoped_refptr<webrtc::RtpTransceiverInterface> api);
    ~ArcasRTPTransceiver()
    {
        RTC_LOG(LS_VERBOSE) << "~ArcasRTPTransceiver";
    }

    // void set_track(std::shared_ptr<ArcasVideoTrack>)

    rust::String mid() const
    {
        auto out = api->mid();
        if (out.has_value())
        {
            return rust::String(out.value().c_str());
        }
        return rust::String();
    }

    cricket::MediaType media_type() const
    {
        return api->media_type();
    }

    bool stopped() const
    {
        return api->stopped();
    }

    bool stopping() const
    {
        return api->stopping();
    }

    webrtc::RtpTransceiverDirection direction() const
    {
        return api->direction();
    }

    std::unique_ptr<ArcasRTCError> stop() const
    {
        return std::make_unique<ArcasRTCError>(api->StopStandard());
    }

    std::unique_ptr<std::vector<ArcasRTPHeaderExtensionCapability>> header_extensions_to_offer() const
    {
        auto list = api->HeaderExtensionsToOffer();
        auto out = std::make_unique<std::vector<ArcasRTPHeaderExtensionCapability>>();

        for (auto item : list)
        {
            ArcasRTPHeaderExtensionCapability cap(item);
            out->push_back(cap);
        }
        return out;
    }

    std::unique_ptr<std::vector<ArcasRTPHeaderExtensionCapability>> header_extensions_to_negotiated() const
    {
        auto list = api->HeaderExtensionsNegotiated();
        auto out = std::make_unique<std::vector<ArcasRTPHeaderExtensionCapability>>();

        for (auto item : list)
        {
            ArcasRTPHeaderExtensionCapability cap(item);
            out->push_back(cap);
        }
        return out;
    }

    std::unique_ptr<std::vector<ArcasRTPCodecCapability>> codec_preferences() const
    {
        auto list = api->codec_preferences();
        auto out = std::make_unique<std::vector<ArcasRTPCodecCapability>>();

        for (auto item : list)
        {
            ArcasRTPCodecCapability cap(item);
            out->push_back(cap);
        }
        return out;
    }

    std::unique_ptr<ArcasRTCError> set_direction(webrtc::RtpTransceiverDirection direction) const {
        return std::make_unique<ArcasRTCError>(api->SetDirectionWithError(direction));
    }

    std::unique_ptr<ArcasRTCError> set_codec_preferences(std::unique_ptr<std::vector<ArcasRTPCodecCapability>> codecs) const
    {
        std::vector<webrtc::RtpCodecCapability> list;

        for (auto item : *codecs)
        {
            list.push_back(item.get());
        }

        rtc::ArrayView<webrtc::RtpCodecCapability> view(list);
        return std::make_unique<ArcasRTCError>(api->SetCodecPreferences(view));
    }

    std::unique_ptr<ArcasRTCError> set_offerred_rtp_header_extensions(std::unique_ptr<std::vector<ArcasRTPHeaderExtensionCapability>> extensions) const
    {
        std::vector<webrtc::RtpHeaderExtensionCapability> list;

        for (auto item : *extensions)
        {
            list.push_back(item.get());
        }

        rtc::ArrayView<webrtc::RtpHeaderExtensionCapability> view(list);

        return std::make_unique<ArcasRTCError>(api->SetOfferedRtpHeaderExtensions(view));
    }
};

class ArcasRTPVideoTransceiver : public ArcasRTPTransceiver
{
public:
    ArcasRTPVideoTransceiver(rtc::scoped_refptr<webrtc::RtpTransceiverInterface> api) : ArcasRTPTransceiver(api) {}

    std::unique_ptr<ArcasRTPVideoSender> get_sender() const
    {
        return std::make_unique<ArcasRTPVideoSender>(api->sender());
    }

    std::unique_ptr<ArcasRTPVideoReceiver> get_receiver() const
    {
        return std::make_unique<ArcasRTPVideoReceiver>(api->receiver());
    }

    std::unique_ptr<ArcasRTPVideoTransceiver> clone() const {
        return std::make_unique<ArcasRTPVideoTransceiver>(api);
    }
};

class ArcasRTPAudioTransceiver : public ArcasRTPTransceiver
{
public:
    ArcasRTPAudioTransceiver(rtc::scoped_refptr<webrtc::RtpTransceiverInterface> api) : ArcasRTPTransceiver(api) {}
    std::unique_ptr<ArcasRTPAudioSender> get_sender() const
    {
        return std::make_unique<ArcasRTPAudioSender>(api->sender());
    }

    std::unique_ptr<ArcasRTPAudioReceiver> get_receiver() const
    {
        return std::make_unique<ArcasRTPAudioReceiver>(api->receiver());
    }

    std::unique_ptr<ArcasRTPAudioTransceiver> clone() const {
        return std::make_unique<ArcasRTPAudioTransceiver>(api);
    }
};

