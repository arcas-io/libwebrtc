#pragma once
#include "api/peer_connection_interface.h"
#include "libwebrtc-sys/include/api_internal.h"
#include "libwebrtc-sys/include/audio_device_module.h"
#include "libwebrtc-sys/include/audio_track.h"
#include "libwebrtc-sys/include/audio_track_source.h"
#include "libwebrtc-sys/include/peer_connection.h"
#include "libwebrtc-sys/include/rust_shared.h"
#include "libwebrtc-sys/include/video_decoder_factory.h"
#include "libwebrtc-sys/include/video_encoder_factory.h"
#include "libwebrtc-sys/include/video_track.h"
#include "libwebrtc-sys/include/video_track_source.h"

using ArcasCxxBundlePolicy = webrtc::PeerConnectionInterface::BundlePolicy;
using ArcasCxxRtcpMuxPolicy = webrtc::PeerConnectionInterface::RtcpMuxPolicy;

class ArcasAPIInternal;

class ArcasPeerConnectionFactory
{
private:
    // Held to ensure we don't need to keep references alive in rust.
    rtc::scoped_refptr<ArcasAPIInternal> internal_api;
    rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> api;

public:
    ArcasPeerConnectionFactory(rtc::scoped_refptr<ArcasAPIInternal> internal_api,
                               rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> api);

    /**
     * @pre observer is not null
     */
    std::shared_ptr<ArcasPeerConnection> create_peer_connection(
        std::unique_ptr<webrtc::PeerConnectionInterface::RTCConfiguration> config,
        ArcasPeerConnectionObserver* observer) const
    {
        webrtc::PeerConnectionDependencies deps(observer);
        auto result = api->CreatePeerConnectionOrError(*config, std::move(deps));

        if (!result.ok())
        {
            RTC_LOG(LS_ERROR) << "Error creating peer connection: " << result.error().message();
            return nullptr;
        }
        auto under_ptr = result.value().get();
        auto out = std::make_shared<ArcasPeerConnection>(api, std::move(result.MoveValue()));
        if (observer && under_ptr)
        {
            observer->observe(*under_ptr);
        }
        else
        {
            RTC_LOG(LS_ERROR)
                << "The peer connection observer passed into create a peer connection should not "
                   "be null, nor should the shared connection api which was just created.";
        }
        return out;
    }
    std::unique_ptr<ArcasVideoTrack>
    create_video_track(rust::String id, const ArcasVideoTrackSource& video_source) const
    {
        auto track = api->CreateVideoTrack(std::string(id.c_str()), video_source.ref());
        return std::make_unique<ArcasVideoTrack>(track);
    }

    std::unique_ptr<ArcasAudioTrack>
    create_audio_track(rust::String id, const ArcasAudioTrackSource& audio_source) const
    {
        auto track = api->CreateAudioTrack(id.c_str(), audio_source.GetSource().get());
        return std::make_unique<ArcasAudioTrack>(track);
    }
};

std::unique_ptr<webrtc::PeerConnectionInterface::RTCConfiguration>
create_rtc_configuration(ArcasPeerConnectionConfig config);
std::unique_ptr<ArcasPeerConnectionFactory> gen_unique_peer_connection_factory();
