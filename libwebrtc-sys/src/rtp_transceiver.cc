#include "rtp_transceiver.h"
#include "peer_connection.h"
#include <peer_connection_stats_callback.h>

ArcasRTPTransceiver::ArcasRTPTransceiver(webrtc::PeerConnectionInterface& associated_connection,
                                         rtc::scoped_refptr<webrtc::RtpTransceiverInterface> api)
: connection{associated_connection}
, api(api)
{
}

void ArcasRTPTransceiver::get_stats(rust::Box<ArcasRustRTCStatsCollectorCallback> cb) const
{
    auto cb_ = rtc::make_ref_counted<ArcasRTCStatsCollectorCallback>(std::move(cb), 2);
    connection.GetStats(api->sender(), cb_);
    connection.GetStats(api->receiver(), cb_);
}

std::unique_ptr<ArcasRTPVideoTransceiver> video_transceiver_from_base(const ArcasRTPTransceiver& transceiver)
{
    return std::make_unique<ArcasRTPVideoTransceiver>(transceiver.connection, transceiver.api);
}

std::unique_ptr<ArcasRTPAudioTransceiver> audio_transceiver_from_base(const ArcasRTPTransceiver& transceiver)
{
    return std::make_unique<ArcasRTPAudioTransceiver>(transceiver.connection, transceiver.api);
}
