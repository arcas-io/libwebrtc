#pragma once

#include "unimp.h"
#include <api/peer_connection_interface.h>

namespace
{
struct test_webrtc_peer_connection final : public webrtc::PeerConnectionInterface
{
    using StreamCollectionInterface = webrtc::StreamCollectionInterface;
    using MediaStreamInterface = webrtc::MediaStreamInterface;
    using MediaStreamTrackInterface = webrtc::MediaStreamTrackInterface;
    using RtpSenderInterface = webrtc::RtpSenderInterface;
    using RtpReceiverInterface = webrtc::RtpReceiverInterface;
    using RtpTransceiverInterface = webrtc::RtpTransceiverInterface;
    template<class T>
    using RTCErrorOr = webrtc::RTCErrorOr<T>;
    using RtpTransceiverInit = webrtc::RtpTransceiverInit;
    using RTCError = webrtc::RTCError;
    using StatsObserver = webrtc::StatsObserver;
    using RTCStatsCollectorCallback = webrtc::RTCStatsCollectorCallback;
    rtc::scoped_refptr<StreamCollectionInterface> local_streams() UNIMP
        virtual rtc::scoped_refptr<StreamCollectionInterface> remote_streams() UNIMP
        virtual bool AddStream(MediaStreamInterface* stream) UNIMP
        virtual void RemoveStream(MediaStreamInterface* stream) UNIMP
        virtual RTCErrorOr<rtc::scoped_refptr<RtpSenderInterface>> AddTrack(
            rtc::scoped_refptr<MediaStreamTrackInterface> track,
            const std::vector<std::string>& stream_ids) UNIMP
        virtual bool RemoveTrack(RtpSenderInterface* sender) UNIMP virtual webrtc::
            RTCErrorOr<rtc::scoped_refptr<webrtc::RtpTransceiverInterface>> AddTransceiver(
                rtc::scoped_refptr<MediaStreamTrackInterface> track) UNIMP
        virtual RTCErrorOr<rtc::scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
            rtc::scoped_refptr<MediaStreamTrackInterface> track,
            const RtpTransceiverInit& init) UNIMP
        virtual RTCErrorOr<rtc::scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
            cricket::MediaType media_type) UNIMP
        virtual RTCErrorOr<rtc::scoped_refptr<RtpTransceiverInterface>> AddTransceiver(
            cricket::MediaType media_type, const RtpTransceiverInit& init) UNIMP
        virtual rtc::scoped_refptr<RtpSenderInterface> CreateSender(
            const std::string& kind, const std::string& stream_id) UNIMP

        // If Plan B semantics are specified, gets all RtpSenders, created either
        // through AddStream, AddTrack, or CreateSender. All senders of a specific
        // media type share the same media description.
        //
        // If Unified Plan semantics are specified, gets the RtpSender for each
        // RtpTransceiver.
        virtual std::vector<rtc::scoped_refptr<RtpSenderInterface>> GetSenders() const UNIMP

        // If Plan B semantics are specified, gets all RtpReceivers created when a
        // remote description is applied. All receivers of a specific media type share
        // the same media description. It is also possible to have a media description
        // with no associated RtpReceivers, if the directional attribute does not
        // indicate that the remote peer is sending any media.
        //
        // If Unified Plan semantics are specified, gets the RtpReceiver for each
        // RtpTransceiver.
        virtual std::vector<rtc::scoped_refptr<RtpReceiverInterface>> GetReceivers() const UNIMP
        virtual std::vector<rtc::scoped_refptr<RtpTransceiverInterface>> GetTransceivers()
            const UNIMP virtual bool GetStats(StatsObserver* observer,
                                              MediaStreamTrackInterface* track,// Optional
                                              StatsOutputLevel level) UNIMP
        virtual void GetStats(RTCStatsCollectorCallback* callback) UNIMP std::vector<std::pair<
            rtc::scoped_refptr<RtpSenderInterface>,
            rtc::scoped_refptr<RTCStatsCollectorCallback>>> senders_awaiting_stats_callbacks_;
    std::vector<std::pair<rtc::scoped_refptr<RtpReceiverInterface>,
                          rtc::scoped_refptr<RTCStatsCollectorCallback>>>
        recvers_awaiting_stats_callbacks_;
    virtual void GetStats(rtc::scoped_refptr<RtpSenderInterface> selector,
                          rtc::scoped_refptr<RTCStatsCollectorCallback> callback)
    {
        senders_awaiting_stats_callbacks_.emplace_back(selector, callback);
    }
    virtual void GetStats(rtc::scoped_refptr<RtpReceiverInterface> selector,
                          rtc::scoped_refptr<RTCStatsCollectorCallback> callback)
    {
        recvers_awaiting_stats_callbacks_.emplace_back(selector, callback);
    }
    using RtcEventLogOutput = webrtc::RtcEventLogOutput;
    virtual bool StartRtcEventLog(std::unique_ptr<RtcEventLogOutput> output,
                                  int64_t output_period_ms) UNIMP
        virtual bool StartRtcEventLog(std::unique_ptr<RtcEventLogOutput> output) UNIMP

        // Stops logging the RtcEventLog.
        virtual void StopRtcEventLog() UNIMP virtual void Close() UNIMP virtual IceConnectionState
        ice_connection_state() UNIMP

        // Returns an aggregated state of all ICE transports.
        virtual IceConnectionState standardized_ice_connection_state() UNIMP

        // Returns an aggregated state of all ICE and DTLS transports.
        virtual PeerConnectionState peer_connection_state() UNIMP

        virtual IceGatheringState ice_gathering_state() UNIMP virtual void AddRef() const UNIMP
        using RefCountReleaseStatus = rtc::RefCountReleaseStatus;
    virtual RefCountReleaseStatus Release() const UNIMP
        using SessionDescriptionInterface = webrtc::SessionDescriptionInterface;
    virtual const SessionDescriptionInterface* local_description() const UNIMP
        virtual const SessionDescriptionInterface* remote_description() const UNIMP
        virtual const SessionDescriptionInterface* current_local_description() const UNIMP
        virtual const SessionDescriptionInterface* current_remote_description() const UNIMP
        using DtlsTransportInterface = webrtc::DtlsTransportInterface;
    using SctpTransportInterface = webrtc::SctpTransportInterface;
    virtual rtc::scoped_refptr<DtlsTransportInterface>
    LookupDtlsTransportByMid(const std::string& mid) UNIMP
        virtual rtc::scoped_refptr<SctpTransportInterface> GetSctpTransport() const UNIMP

        // Returns the current SignalingState.
        virtual SignalingState signaling_state() UNIMP
        using IceCandidateInterface = webrtc::IceCandidateInterface;
    virtual bool AddIceCandidate(const IceCandidateInterface* candidate) UNIMP
        virtual void AddIceCandidate(std::unique_ptr<IceCandidateInterface> candidate,
                                     std::function<void(RTCError)> callback)
    {
    }
    virtual bool RemoveIceCandidates(const std::vector<cricket::Candidate>& candidates) UNIMP
        using BitrateSettings = webrtc::BitrateSettings;
    virtual RTCError SetBitrate(const BitrateSettings& bitrate) UNIMP
        virtual const SessionDescriptionInterface* pending_local_description() const UNIMP
        virtual const SessionDescriptionInterface* pending_remote_description() const UNIMP
        virtual void RestartIce() UNIMP
        using SetSessionDescriptionObserver = webrtc::SetSessionDescriptionObserver;
    virtual void SetRemoteDescription(SetSessionDescriptionObserver* observer,
                                      SessionDescriptionInterface* desc)
    {
    }
    virtual bool ShouldFireNegotiationNeededEvent(uint32_t event_id)
    {
        return true;
    }
    using CreateSessionDescriptionObserver = webrtc::CreateSessionDescriptionObserver;
    virtual void CreateOffer(CreateSessionDescriptionObserver* observer,
                             const RTCOfferAnswerOptions& options) UNIMP
        virtual void CreateAnswer(CreateSessionDescriptionObserver* observer,
                                  const RTCOfferAnswerOptions& options) UNIMP

        virtual PeerConnectionInterface::RTCConfiguration GetConfiguration() UNIMP
        using SetLocalDescriptionObserverInterface = webrtc::SetLocalDescriptionObserverInterface;
    virtual void SetLocalDescription(SetSessionDescriptionObserver* observer,
                                     SessionDescriptionInterface* desc) UNIMP
        using SetRemoteDescriptionObserverInterface = webrtc::SetRemoteDescriptionObserverInterface;
    virtual void
    SetRemoteDescription(std::unique_ptr<SessionDescriptionInterface> desc,
                         rtc::scoped_refptr<SetRemoteDescriptionObserverInterface> observer) UNIMP
};
}//namespace
