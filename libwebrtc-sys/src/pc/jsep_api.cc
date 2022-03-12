#include "libwebrtc-sys/src/pc/jsep_api.rs.h"

void ArcasJsepTransportControllerConfig::invoke_rtcp_handler(rtc::CopyOnWriteBuffer packet, int64_t packet_time_us)
{
    if (_rtcp_handler)
    {
        _rtcp_handler.value()->invoke(packet, packet_time_us);
    }
}

void ArcasJsepTransportControllerConfig::invoke_dtls_handshake_error_handler(rtc::SSLHandshakeError error)
{
    if (_dtls_handshake_error_handler)
    {
        _dtls_handshake_error_handler.value()->invoke(error);
    }
}

bool ArcasJsepTransportControllerObserver::OnTransportChanged(const std::string& mid,
                                                              webrtc::RtpTransportInternal* rtp_transport,
                                                              rtc::scoped_refptr<webrtc::DtlsTransport> dtls_transport,
                                                              webrtc::DataChannelTransportInterface* data_channel_transport)
{
    auto dtls = std::make_unique<ArcasDTLSTransport>(dtls_transport);
    auto rust_mid = rust::String{mid.data(), mid.size()};
    return _observer->invoke(rust_mid, rtp_transport, std::move(dtls), data_channel_transport);
}

std::unique_ptr<cricket::PortAllocator> create_arcas_cxx_port_allocator(rtc::NetworkManager* network_manager)
{
    // new is used for explicit use of a constructor.
    return std::unique_ptr<cricket::BasicPortAllocator>(new cricket::BasicPortAllocator(network_manager));
}

std::unique_ptr<ArcasJsepTransportControllerConfig> create_arcas_jsep_transport_controller_config()
{
    return std::make_unique<ArcasJsepTransportControllerConfig>();
}

std::unique_ptr<ArcasJsepTransportController>
create_arcas_jsep_transport_controller(rtc::Thread* network_thread,
                                       std::unique_ptr<cricket::PortAllocator> port_allocator,
                                       webrtc::AsyncDnsResolverFactoryInterface* async_dns_resolver_factory,
                                       std::unique_ptr<ArcasJsepTransportControllerConfig> config)
{
    return std::make_unique<ArcasJsepTransportController>(network_thread, std::move(port_allocator), async_dns_resolver_factory, std::move(config));
}

std::unique_ptr<ArcasRTCError> ArcasJsepTransportController::add_remote_candidates(rust::String mid, rust::Vec<ArcasCandidateWrapper> candidates)
{
    //Convert the list
    std::vector<cricket::Candidate> candidates_list(candidates.size());
    auto get = [](auto& c) { return c.ptr->get_candidate(); };
    std::transform(candidates.begin(), candidates.end(), candidates_list.begin(), get);

    return add_remote_candidates({mid.data(), mid.size()}, candidates_list);
}

std::unique_ptr<ArcasRTCError> ArcasJsepTransportController::add_remote_candidates(std::string transport_name,
                                                                                   std::vector<cricket::Candidate> candidates)
{
    webrtc::RTCError err;//The default ctor // Creates a "no error" error.
    auto do_add = [&]() { err = _transport_controller->AddRemoteCandidates(transport_name, candidates); };
    if (_network_thread->IsCurrent())
    {
        do_add();
    }
    else
    {
        _network_thread->Invoke<void>(RTC_FROM_HERE, do_add);
    }
    return std::make_unique<ArcasRTCError>(err);
}

rust::Vec<rtc::SSLRole> ArcasJsepTransportController::get_dtls_role(rust::String mid) const
{
    auto role = _transport_controller->GetDtlsRole(mid.c_str());
    rust::Vec<rtc::SSLRole> out;
    if (role.has_value())
    {
        out.push_back(role.value());
    }
    return out;
}


std::unique_ptr<ArcasRTCError> ArcasJsepTransportController::remove_remote_candidates(rust::Vec<ArcasCandidateWrapper> candidates)
{
    std::vector<cricket::Candidate> candidates_list;

    for (auto& candidate : candidates) { candidates_list.push_back(candidate.ptr->get_candidate()); }
    auto err = _transport_controller->RemoveRemoteCandidates(candidates_list);
    return std::make_unique<ArcasRTCError>(err);
}

std::unique_ptr<ArcasJsepTransportControllerObserver> to_cxx(rust::Box<ArcasRustJsepTransportControllerObserver> r)
{
    return std::make_unique<ArcasJsepTransportControllerObserver>(std::move(r));
}
