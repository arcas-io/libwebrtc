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

struct ArcasRustJsepRTCPHandler;
struct ArcasRustDTLSHandshakeErrorHandler;
struct ArcasRustJsepTransportControllerObserver;
struct ArcasCandidateWrapper;

class ArcasJsepTransportControllerObserver : public webrtc::JsepTransportController::Observer
{
private:
    rust::Box<ArcasRustJsepTransportControllerObserver> _observer;

public:
    ArcasJsepTransportControllerObserver(
        rust::Box<ArcasRustJsepTransportControllerObserver> observer)
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

    void set_observer(std::unique_ptr<webrtc::JsepTransportController::Observer> observer)
    {
        _transport_observer = std::move(observer);
        _config.transport_observer = _transport_observer.get();
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
        _config.rtcp_handler = [this](rtc::CopyOnWriteBuffer packet, int64_t packet_time_us)
        { this->invoke_rtcp_handler(packet, packet_time_us); };
    }

    void set_dtls_handshake_error_handler(rust::Box<ArcasRustDTLSHandshakeErrorHandler> handler)
    {
        _dtls_handshake_error_handler = std::move(handler);
        _config.on_dtls_handshake_error_ = [this](rtc::SSLHandshakeError error)
        { this->invoke_dtls_handshake_error_handler(error); };
    }

    void set_transport_observer(rust::Box<ArcasRustJsepTransportControllerObserver> handler)
    {
        _transport_observer =
            std::make_unique<ArcasJsepTransportControllerObserver>(std::move(handler));
        _config.transport_observer = _transport_observer.get();
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
        return _config;
    }
};

class ArcasJsepTransportController
{
private:
    std::unique_ptr<webrtc::JsepTransportController> _transport_controller;
    std::unique_ptr<ArcasJsepTransportControllerConfig> _config;
    rtc::Thread* _network_thread;

public:
    ArcasJsepTransportController(
        rtc::Thread* network_thread,
        cricket::PortAllocator* port_allocator,
        webrtc::AsyncDnsResolverFactoryInterface* async_dns_resolver_factory,
        std::unique_ptr<ArcasJsepTransportControllerConfig> config)
    {
        _config = std::move(config);
        _transport_controller =
            std::make_unique<webrtc::JsepTransportController>(network_thread,
                                                              port_allocator,
                                                              async_dns_resolver_factory,
                                                              config->get_config());
        _network_thread = network_thread;
    }

    std::unique_ptr<ArcasRTCError>
    set_local_description(webrtc::SdpType sdp_type,
                          std::unique_ptr<ArcasSessionDescription> description)
    {
        // Ensure we always run this method on the correct thread..
        if (!_network_thread->IsCurrent())
        {
            return _network_thread->Invoke<std::unique_ptr<ArcasRTCError>>(
                RTC_FROM_HERE,
                [&] { return this->set_local_description(sdp_type, std::move(description)); });
        }

        auto err =
            _transport_controller->SetLocalDescription(sdp_type,
                                                       description->jsep_session_description());
        return std::make_unique<ArcasRTCError>(err);
    }

    std::unique_ptr<ArcasRTCError>
    set_remote_description(webrtc::SdpType sdp_type,
                           std::unique_ptr<ArcasSessionDescription> description)
    {
        // Ensure we always run this method on the correct thread..
        if (!_network_thread->IsCurrent())
        {
            return _network_thread->Invoke<std::unique_ptr<ArcasRTCError>>(
                RTC_FROM_HERE,
                [&] { return this->set_remote_description(sdp_type, std::move(description)); });
        }

        auto err =
            _transport_controller->SetLocalDescription(sdp_type,
                                                       description->jsep_session_description());
        return std::make_unique<ArcasRTCError>(err);
    }

    webrtc::RtpTransportInternal* get_rtp_transport(rust::String mid) const
    {
        // Ensure we always run this method on the correct thread..
        if (!_network_thread->IsCurrent())
        {
            return _network_thread->Invoke<webrtc::RtpTransportInternal*>(
                RTC_FROM_HERE,
                [&] { return this->get_rtp_transport(mid); });
        }

        return _transport_controller->GetRtpTransport(mid.c_str());
    }

    webrtc::DataChannelTransportInterface* get_data_channel_transport(rust::String mid) const
    {
        // Ensure we always run this method on the correct thread..
        if (!_network_thread->IsCurrent())
        {
            return _network_thread->Invoke<webrtc::DataChannelTransportInterface*>(
                RTC_FROM_HERE,
                [&] { return this->get_data_channel_transport(mid); });
        }

        return _transport_controller->GetDataChannelTransport(mid.c_str());
    }

    void set_ice_config(std::unique_ptr<ArcasP2PIceConfig> config)
    {
        // Ensure we always run this method on the correct thread..
        if (!_network_thread->IsCurrent())
        {
            return _network_thread->Invoke<void>(RTC_FROM_HERE,
                                                 [&] { this->set_ice_config(std::move(config)); });
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

    std::unique_ptr<ArcasRTCError>
    add_remote_candidates(rust::String mid, rust::Vec<ArcasCandidateWrapper> candidates);
    std::unique_ptr<ArcasRTCError>
    remove_remote_candidates(rust::Vec<ArcasCandidateWrapper> candidates);

    // We use a rust vector here to represent the optional type. No values is none.
    rust::Vec<rtc::SSLRole> get_dtls_role(rust::String mid) const
    {
        auto role = _transport_controller->GetDtlsRole(mid.c_str());
        rust::Vec<rtc::SSLRole> out;
        if (role.has_value())
        {
            out.push_back(role.value());
        }
        return out;
    }

    void set_active_reset_srtp_params(bool reset_active)
    {
        _transport_controller->SetActiveResetSrtpParams(reset_active);
    }

    std::unique_ptr<ArcasRTCError> rollback_transports()
    {
        auto err = _transport_controller->RollbackTransports();
        return std::make_unique<ArcasRTCError>(err);
    }
};

std::unique_ptr<ArcasDTLSTransport> gen_arcas_cxx_dtls_transport();

std::unique_ptr<ArcasJsepTransportControllerConfig> create_arcas_jsep_transport_controller_config();
std::unique_ptr<cricket::PortAllocator>
create_arcas_cxx_port_allocator(rtc::NetworkManager* network_manager);
std::unique_ptr<ArcasJsepTransportController> create_arcas_jsep_transport_controller(
    rtc::Thread* network_thread,
    cricket::PortAllocator* port_allocator,
    webrtc::AsyncDnsResolverFactoryInterface* async_dns_resolver_factory,
    std::unique_ptr<ArcasJsepTransportControllerConfig> config);
