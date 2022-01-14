#include <rtp_transceiver.h>

#include <tests/mock/api/rtp_transceiver_interface.h>
#include <tests/mock/peer_connection_interface.h>

#include <peer_connection_stats_callback.h>

#include <catch2/catch.hpp>

struct ArcasRustRTCStatsCollectorCallback
{
};

namespace
{
std::vector<std::pair<ArcasRustRTCStatsCollectorCallback*,
                      rtc::scoped_refptr<const webrtc::RTCStatsReport>>>
    stats_delivered;
}


TEST_CASE("ctor", "[ArcasRTPVideoTransceiver]")
{
    test_webrtc_peer_connection pc;
    CHECK_NOTHROW(ArcasRTPVideoTransceiver{pc, nullptr});
}

TEST_CASE("get_stats() : enqueues a callback for sender and receiver", "[ArcasRTPVideoTransceiver]")
{
    stats_delivered.clear();
    test_webrtc_peer_connection pc;
    rtc::scoped_refptr<webrtc::RtpTransceiverInterface> api{
        new webrtc::test_webrtc_peer_rtp_transceiver{}};
    ArcasRTPVideoTransceiver tsrv{pc, api};
    ArcasRustRTCStatsCollectorCallback cb;
    tsrv.get_stats(rust::Box{cb});
    CHECK(pc.senders_awaiting_stats_callbacks_.size() == 1);
    CHECK(pc.recvers_awaiting_stats_callbacks_.size() == 1);
}

void ArcasRTCStatsCollectorCallback::OnStatsDelivered(
    rtc::scoped_refptr<const webrtc::RTCStatsReport> const& report)
{
    stats_delivered.emplace_back(cb.into_raw(), report);
}

//How is alloc noexcept? Is it catching bad_alloc, or doing fun malloc stuff?
template<>
ArcasRustRTCStatsCollectorCallback*
rust::cxxbridge1::Box<ArcasRustRTCStatsCollectorCallback>::allocation::alloc() noexcept
{
    return nullptr;
}
template<>
void rust::cxxbridge1::Box<ArcasRustRTCStatsCollectorCallback>::allocation::dealloc(
    ArcasRustRTCStatsCollectorCallback*) noexcept
{
}
template<>
void rust::cxxbridge1::Box<ArcasRustRTCStatsCollectorCallback>::drop() noexcept
{
}
