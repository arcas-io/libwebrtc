#include "libwebrtc-sys/src/pc/jsep_api.rs.h"

void ArcasJsepTransportControllerConfig::invoke_rtcp_handler(rtc::CopyOnWriteBuffer packet,
                                                             int64_t packet_time_us)
{
    if (_rtcp_handler)
    {
        _rtcp_handler.value()->invoke(packet, packet_time_us);
    }
}

void ArcasJsepTransportControllerConfig::invoke_dtls_handshake_error_handler(
    rtc::SSLHandshakeError error)
{
    if (_dtls_handshake_error_handler)
    {
        _dtls_handshake_error_handler.value()->invoke(error);
    }
}

bool ArcasJsepTransportControllerObserver::OnTransportChanged(
    const std::string& mid,
    webrtc::RtpTransportInternal* rtp_transport,
    rtc::scoped_refptr<webrtc::DtlsTransport> dtls_transport,
    webrtc::DataChannelTransportInterface* data_channel_transport)
{
    auto dtls = std::make_unique<ArcasDTLSTransport>(dtls_transport);
    return _observer->invoke(mid, rtp_transport, std::move(dtls), data_channel_transport);
}

std::unique_ptr<cricket::PortAllocator>
create_arcas_cxx_port_allocator(rtc::NetworkManager* network_manager)
{
    // new is used for explicit use of a constructor.
    return std::unique_ptr<cricket::BasicPortAllocator>(
        new cricket::BasicPortAllocator(network_manager));
}

std::unique_ptr<ArcasJsepTransportControllerConfig> create_arcas_jsep_transport_controller_config()
{
    return std::make_unique<ArcasJsepTransportControllerConfig>();
}

std::unique_ptr<ArcasJsepTransportController> create_arcas_jsep_transport_controller(
    rtc::Thread* network_thread,
    cricket::PortAllocator* port_allocator,
    webrtc::AsyncDnsResolverFactoryInterface* async_dns_resolver_factory,
    std::unique_ptr<ArcasJsepTransportControllerConfig> config)
{
    return std::make_unique<ArcasJsepTransportController>(network_thread,
                                                          port_allocator,
                                                          async_dns_resolver_factory,
                                                          std::move(config));
}

std::unique_ptr<ArcasRTCError>
ArcasJsepTransportController::add_remote_candidates(rust::String mid,
                                                    rust::Vec<ArcasCandidateWrapper> candidates)
{
    std::vector<cricket::Candidate> candidates_list;

    for (auto& candidate : candidates)
    {
        candidates_list.push_back(candidate.ptr->get_candidate());
    }
    auto err = _transport_controller->AddRemoteCandidates(mid.c_str(), candidates_list);
    return std::make_unique<ArcasRTCError>(err);
}


std::unique_ptr<ArcasRTCError>
ArcasJsepTransportController::remove_remote_candidates(rust::Vec<ArcasCandidateWrapper> candidates)
{
    std::vector<cricket::Candidate> candidates_list;

    for (auto& candidate : candidates)
    {
        candidates_list.push_back(candidate.ptr->get_candidate());
    }
    auto err = _transport_controller->RemoveRemoteCandidates(candidates_list);
    return std::make_unique<ArcasRTCError>(err);
}
