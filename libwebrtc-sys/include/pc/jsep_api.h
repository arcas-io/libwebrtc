#pragma once
#include "api/ice_transport_interface.h"
#include "api/peer_connection_interface.h"
#include "api/rtc_event_log/rtc_event_log.h"
#include "api/transport/sctp_transport_factory_interface.h"
#include "libwebrtc-sys/include/candidate.h"
#include "libwebrtc-sys/include/error.h"
#include "libwebrtc-sys/include/ice_candidate.h"
#include "libwebrtc-sys/include/p2p/ice_transport_internal.h"
#include "libwebrtc-sys/include/peer_connection_factory.h"
#include "libwebrtc-sys/include/rtc_base/base.h"
#include "libwebrtc-sys/include/rtc_base/certificates.h"
#include "libwebrtc-sys/include/session_description.h"
#include "p2p/base/default_ice_transport_factory.h"
#include "p2p/client/basic_port_allocator.h"
#include "pc/jsep_transport_controller.h"
#include "rust/cxx.h"
#include <pc/srtp_transport.h>
#include <rtti.h>

struct ArcasRustJsepRTCPHandler;
struct ArcasRustDTLSHandshakeErrorHandler;
struct ArcasRustJsepTransportControllerObserver;
struct ArcasCandidateWrapper;

class ArcasJsepTransportControllerObserver : public webrtc::JsepTransportController::Observer
{
private:
    rust::Box<ArcasRustJsepTransportControllerObserver> _observer;

public:
    ArcasJsepTransportControllerObserver(rust::Box<ArcasRustJsepTransportControllerObserver> observer)
    : _observer(std::move(observer))
    {
    }

    bool OnTransportChanged(const std::string& mid,
                            webrtc::RtpTransportInternal* rtp_transport,
                            rtc::scoped_refptr<webrtc::DtlsTransport> dtls_transport,
                            webrtc::DataChannelTransportInterface* data_channel_transport) override;
};

class ArcasDTLSTransport
{
private:
    rtc::scoped_refptr<webrtc::DtlsTransport> _dtls_transport;

public:
    ArcasDTLSTransport(rtc::scoped_refptr<webrtc::DtlsTransport> dtls_transport)
    : _dtls_transport(std::move(dtls_transport))
    {
    }

    // todo: more methods to manipulate the changed state of the transport
};

class ArcasJsepTransportControllerConfig
{
private:
    webrtc::JsepTransportController::Config _config;
    std::unique_ptr<webrtc::SctpTransportFactoryInterface> _sctp_factory;
    std::unique_ptr<webrtc::IceTransportFactory> _ice_transport_factory;
    std::unique_ptr<webrtc::JsepTransportController::Observer> _transport_observer;
    std::unique_ptr<webrtc::RtcEventLog> _event_log;
    absl::optional<rust::Box<ArcasRustJsepRTCPHandler>> _rtcp_handler;
    absl::optional<rust::Box<ArcasRustDTLSHandshakeErrorHandler>> _dtls_handshake_error_handler;
    // bool active_reset_srtp_params_;
    // std::unique_ptr<webrtc::RtcEventLog> event_log_;

public:
    ArcasJsepTransportControllerConfig() {}

    void set_redetermine_role_on_ice_restart(bool restart)
    {
        _config.redetermine_role_on_ice_restart = restart;
    }

    void set_bundle_policy(webrtc::PeerConnectionInterface::BundlePolicy policy)
    {
        _config.bundle_policy = policy;
    }

    void set_rtcp_mux_policy(webrtc::PeerConnectionInterface::RtcpMuxPolicy policy)
    {
        _config.rtcp_mux_policy = policy;
    }

    void set_ice_transport_factory(std::unique_ptr<webrtc::IceTransportFactory> factory)
    {
        _ice_transport_factory = std::move(factory);
        _config.ice_transport_factory = _ice_transport_factory.get();
    }

    void set_sctp_transport_factory(std::unique_ptr<webrtc::SctpTransportFactoryInterface> factory)
    {
        _sctp_factory = std::move(factory);
        _config.sctp_factory = _sctp_factory.get();
    }

    void set_rtcp_handler(rust::Box<ArcasRustJsepRTCPHandler> handler)
    {
        _rtcp_handler = std::move(handler);
        _config.rtcp_handler = [this](rtc::CopyOnWriteBuffer packet, int64_t packet_time_us) { this->invoke_rtcp_handler(packet, packet_time_us); };
    }
    void bypass_rtcp_handler(decltype(_config.rtcp_handler) handler)
    {
        _config.rtcp_handler = handler;
    }

    void set_dtls_handshake_error_handler(rust::Box<ArcasRustDTLSHandshakeErrorHandler> handler)
    {
        _dtls_handshake_error_handler = std::move(handler);
        _config.on_dtls_handshake_error_ = [this](rtc::SSLHandshakeError error) { this->invoke_dtls_handshake_error_handler(error); };
    }
    void bypass_dtls_handshake_error_handler(decltype(_config.on_dtls_handshake_error_) handler)
    {
        _config.on_dtls_handshake_error_ = handler;
    }

    void set_observer(std::unique_ptr<webrtc::JsepTransportController::Observer> observer)
    {
        _transport_observer = std::move(observer);
        _config.transport_observer = _transport_observer.get();
    }
    void set_transport_observer(std::unique_ptr<ArcasJsepTransportControllerObserver> handler)
    {
        set_observer(std::move(handler));
    }


    void invoke_rtcp_handler(rtc::CopyOnWriteBuffer packet, int64_t packet_time_us);
    void invoke_dtls_handshake_error_handler(rtc::SSLHandshakeError error);

    void set_active_reset_srtp_params(bool active)
    {
        _config.active_reset_srtp_params = active;
    }

    void set_event_log(std::unique_ptr<webrtc::RtcEventLog> event_log)
    {
        _event_log = std::move(event_log);
        _config.event_log = _event_log.get();
    }

    webrtc::JsepTransportController::Config get_config()
    {
        assert(this);
        return _config;
    }
};

class ArcasJsepTransportController
{
private:
    std::unique_ptr<webrtc::JsepTransportController> _transport_controller;
    std::unique_ptr<ArcasJsepTransportControllerConfig> _config;
    std::unique_ptr<cricket::PortAllocator> _port_allocator;
    rtc::Thread* const _network_thread;

public:
    ArcasJsepTransportController(rtc::Thread* network_thread,
                                 std::unique_ptr<cricket::PortAllocator> port_allocator,
                                 webrtc::AsyncDnsResolverFactoryInterface* async_dns_resolver_factory,
                                 std::unique_ptr<ArcasJsepTransportControllerConfig> config)
    : _network_thread{network_thread}
    {
        _config = std::move(config);
        _transport_controller = std::make_unique<webrtc::JsepTransportController>(network_thread,
                                                                                  port_allocator.get(),
                                                                                  async_dns_resolver_factory,
                                                                                  _config->get_config());
        _port_allocator = std::move(port_allocator);
        _network_thread->Invoke<void>(RTC_FROM_HERE, [this]() { _port_allocator->Initialize(); });
    }

    ~ArcasJsepTransportController()
    {
        RTC_DCHECK(_network_thread);
        auto members_which_should_be_destructed_on_the_network_thread = [this]()
        {
            _transport_controller.reset();
            _port_allocator.reset();
        };
        if (_network_thread)
        {
            RTC_DCHECK(!_network_thread->IsQuitting());
            if (_network_thread->IsCurrent())
            {
                members_which_should_be_destructed_on_the_network_thread();
            }
            else
            {
                _network_thread->Invoke<void>(RTC_FROM_HERE, members_which_should_be_destructed_on_the_network_thread);
            }
        }
        else
        {
            members_which_should_be_destructed_on_the_network_thread();
        }
    }

    std::unique_ptr<ArcasRTCError>
    //    set_local_description(webrtc::SdpType sdp_type,std::unique_ptr<ArcasSessionDescription> description)
    set_local_description(webrtc::SdpType sdp_type, ArcasSessionDescription const& description)
    {
        // Ensure we always run this method on the correct thread..
        if (!_network_thread->IsCurrent())
        {
            return _network_thread->Invoke<std::unique_ptr<ArcasRTCError>>(RTC_FROM_HERE,
                                                                           [&]
                                                                           { return this->set_local_description(sdp_type, std::move(description)); });
        }
        auto lower_raw_ptr = description.jsep_session_description();
        auto err = _transport_controller->SetLocalDescription(sdp_type, lower_raw_ptr);
        return std::make_unique<ArcasRTCError>(err);
    }

    //    std::unique_ptr<ArcasRTCError>
    //    set_remote_description(webrtc::SdpType sdp_type,
    //                           std::unique_ptr<ArcasSessionDescription> description)
    //    {
    //        return this->set_remote_description(sdp_type, *description);
    //    }
    std::unique_ptr<ArcasRTCError> set_remote_description(webrtc::SdpType sdp_type, ArcasSessionDescription const& description)
    {
        // Ensure we always run this method on the correct thread..
        if (!_network_thread->IsCurrent())
        {
            return _network_thread->Invoke<std::unique_ptr<ArcasRTCError>>(RTC_FROM_HERE,
                                                                           [this, sdp_type, &description]()
                                                                           { return this->set_remote_description(sdp_type, description); });
        }
        auto err = _transport_controller->SetRemoteDescription(sdp_type, description.jsep_session_description());
        return std::make_unique<ArcasRTCError>(err);
    }

    using srtp_t = webrtc::SrtpTransport;
    srtp_t* get_srtp_transport(rust::String mid) const
    {
        // Ensure we always run this method on the correct thread..
        if (!_network_thread->IsCurrent())
        {
            return _network_thread->Invoke<srtp_t*>(RTC_FROM_HERE, [&] { return this->get_srtp_transport(mid); });
        }
        auto base_ptr = _transport_controller->GetRtpTransport(mid.c_str());
        return unsafe_downcast<srtp_t*>(base_ptr);
    }

    webrtc::DataChannelTransportInterface* get_data_channel_transport(rust::String mid) const
    {
        // Ensure we always run this method on the correct thread..
        if (!_network_thread->IsCurrent())
        {
            return _network_thread->Invoke<webrtc::DataChannelTransportInterface*>(RTC_FROM_HERE,
                                                                                   [&] { return this->get_data_channel_transport(mid); });
        }

        return _transport_controller->GetDataChannelTransport(mid.c_str());
    }

    void set_ice_config(std::unique_ptr<ArcasP2PIceConfig> config)
    {
        // Ensure we always run this method on the correct thread..
        if (!_network_thread->IsCurrent())
        {
            return _network_thread->Invoke<void>(RTC_FROM_HERE, [&] { this->set_ice_config(std::move(config)); });
        }

        _transport_controller->SetIceConfig(config->get_config());
    }

    void set_needs_ice_restart_flag()
    {
        _transport_controller->SetNeedsIceRestartFlag();
    }

    bool needs_ice_restart(rust::String mid) const
    {
        return _transport_controller->NeedsIceRestart(mid.c_str());
    }

    void maybe_start_gathering()
    {
        _transport_controller->MaybeStartGathering();
    }

    void set_local_certificate(std::unique_ptr<ArcasSSLCertificate> cert)
    {
        _transport_controller->SetLocalCertificate(cert->get_certificate());
    }

    std::unique_ptr<ArcasSSLCertificate> get_local_certificate(rust::String mid) const
    {
        auto cert = _transport_controller->GetLocalCertificate(mid.c_str());
        return std::make_unique<ArcasSSLCertificate>(cert);
    }

    std::unique_ptr<ArcasRTCError> add_remote_candidates(rust::String mid, rust::Vec<ArcasCandidateWrapper> candidates);
    std::unique_ptr<ArcasRTCError> remove_remote_candidates(rust::Vec<ArcasCandidateWrapper> candidates);

    std::unique_ptr<ArcasRTCError> add_remote_candidates(std::string mid, std::vector<cricket::Candidate> candidates);

    // We use a rust vector here to represent the optional type. No values is none.
    rust::Vec<rtc::SSLRole> get_dtls_role(rust::String mid) const;

    void set_active_reset_srtp_params(bool reset_active)
    {
        _transport_controller->SetActiveResetSrtpParams(reset_active);
    }

    std::unique_ptr<ArcasRTCError> rollback_transports()
    {
        auto err = _transport_controller->RollbackTransports();
        return std::make_unique<ArcasRTCError>(err);
    }
    void subsribe_ice_connection_state(std::function<void(cricket::IceConnectionState)> f)
    {
        auto on_net = [&] { _transport_controller->SubscribeIceConnectionState(f); };
        if (_network_thread->IsCurrent())
        {
            on_net();
        }
        else
        {
            _network_thread->Invoke<void>(RTC_FROM_HERE, on_net);
        }
    }
};

std::unique_ptr<ArcasDTLSTransport> gen_arcas_cxx_dtls_transport();

std::unique_ptr<ArcasJsepTransportControllerConfig> create_arcas_jsep_transport_controller_config();
std::unique_ptr<cricket::PortAllocator> create_arcas_cxx_port_allocator(rtc::NetworkManager* network_manager);
std::unique_ptr<ArcasJsepTransportController>
create_arcas_jsep_transport_controller(rtc::Thread* network_thread,
                                       std::unique_ptr<cricket::PortAllocator> port_allocator,
                                       webrtc::AsyncDnsResolverFactoryInterface* async_dns_resolver_factory,
                                       std::unique_ptr<ArcasJsepTransportControllerConfig> config);

std::unique_ptr<ArcasJsepTransportControllerObserver> to_cxx(rust::Box<ArcasRustJsepTransportControllerObserver> r);
inline bool send_rtp_packet(webrtc::SrtpTransport& transport, rtc::CopyOnWriteBuffer& packet, rtc::Thread& network_thread)
{
    return network_thread.Invoke<bool>(RTC_FROM_HERE, [&]() { return transport.SendRtpPacket(&packet, {}, {}); });
}

inline std::unique_ptr<rtc::CopyOnWriteBuffer> create_buffer(::std::uint64_t capacity)
{
    return std::make_unique<rtc::CopyOnWriteBuffer>(capacity);
}
inline std::unique_ptr<rtc::CopyOnWriteBuffer> create_buffer_with_data(rust::Slice<std::uint8_t const> bytes)
{
    return std::make_unique<rtc::CopyOnWriteBuffer>(bytes.data(), bytes.size());
}

inline void init_port_alloc(cricket::PortAllocator& port_alloc, rtc::Thread& network_thread)
{
    network_thread.Invoke<void>(RTC_FROM_HERE, [&port_alloc]() { port_alloc.Initialize(); });
}
inline rust::String get_transport_name(webrtc::SrtpTransport const& transport, rtc::Thread& net_thr)
{
    auto name = net_thr.Invoke<std::string>(RTC_FROM_HERE, [&]() { return transport.transport_name(); });
    return {name.data(), name.size()};
}
inline bool is_writable(webrtc::SrtpTransport const& transport)
{
    return transport.IsWritable(true) || transport.IsWritable(false);
}

inline void set_rtp_params(webrtc::SrtpTransport& transport,
                           std::int32_t send_cs,
                           rust::String send_key,
                           std::int32_t recv_cs,
                           rust::String recv_key,
                           rust::Vec<std::int32_t> recv_extension_ids,
                           rtc::Thread& net_thr)
{
    auto on_net_thr = [&]()
    {
        auto sk = reinterpret_cast<std::uint8_t const*>(send_key.data());
        auto rk = reinterpret_cast<std::uint8_t const*>(recv_key.data());
        transport.SetRtpParams(1, sk, send_key.size(), {}, 1, rk, recv_key.size(), {});
    };
    net_thr.Invoke<void>(RTC_FROM_HERE, on_net_thr);
}
