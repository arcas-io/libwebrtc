#include "iostream"
#include "rust/cxx.h"
#include "libwebrtc-sys/include/peer_connection_factory.h"
#include "libwebrtc-sys/include/api_internal.h"
#include "libwebrtc-sys/include/peer_connection_observer.h"
#include "libwebrtc-sys/src/lib.rs.h"

ArcasPeerConnectionFactory::ArcasPeerConnectionFactory(
    rtc::scoped_refptr<ArcasAPIInternal> internal_api,
    rtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> api) : internal_api(internal_api), api(api)
{
    internal_api->AddRef();
};

std::unique_ptr<webrtc::PeerConnectionInterface::RTCConfiguration> create_rtc_configuration(ArcasPeerConnectionConfig config)
{
    auto rtc = std::make_unique<webrtc::PeerConnectionInterface::RTCConfiguration>();
    webrtc::PeerConnectionInterface::IceServers servers;

    rtc->sdp_semantics = config.sdp_semantics;
    rtc->servers = servers;

    for (auto server_config : config.ice_servers)
    {
        webrtc::PeerConnectionInterface::IceServer rtc_ice_server;
        std::vector<std::string> rtc_urls;

        for (auto url : server_config.urls)
        {
            auto rtc_url = std::string(url.c_str());
            rtc_urls.push_back(rtc_url);
        }

        rtc_ice_server.urls = rtc_urls;
        rtc_ice_server.username = std::string(server_config.username.c_str());
        rtc_ice_server.password = std::string(server_config.password.c_str());
        servers.push_back(rtc_ice_server);
    }
    RTC_LOG(LS_VERBOSE) << "RTC LOG WITH " << servers.size() << " URLS";

    return rtc;
}

std::unique_ptr<ArcasPeerConnectionObserver> create_peer_connection_observer(rust::Box<ArcasRustPeerConnectionObserver> rust_box)
{
    return std::make_unique<ArcasPeerConnectionObserver>(std::move(rust_box));
}
