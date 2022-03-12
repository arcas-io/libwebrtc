#include <libwebrtc-sys/src/pc/jsep_api.rs.h>
#include <pc/jsep_api.h>

#include <async_dns_resolver_factory.h>
#include <ice_transport.h>
#include <rtc_base/ssl_stream_adapter.h>
#include <session_description.rs.h>

#include <tests/mock/erasure.h>
#include <tests/mock/stub_macros.h>

#include <catch2/catch.hpp>
#include <cxx.h>
#include <fmt/core.h>

using namespace std::literals;


TEST_CASE("ctor", "[ArcasJsepTransportController]")
{
    auto cfg = std::make_unique<ArcasJsepTransportControllerConfig>();
    cfg->set_observer(make_jsep_obs([](auto, auto, auto&&, auto) { return true; }));
    cfg->bypass_rtcp_handler([](auto&, auto) {});
    cfg->set_ice_transport_factory(std::make_unique<webrtc::DefaultIceTransportFactory>());
    cfg->bypass_dtls_handshake_error_handler([](auto&&) {});
    auto net_thr = rtc::Thread::Create();
    CHECK(net_thr->Start());
    ArcasJsepTransportController ajtc{net_thr.get(), {}, {}, std::move(cfg)};
}
