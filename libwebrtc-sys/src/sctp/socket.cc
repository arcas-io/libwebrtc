#pragma once
#include "libwebrtc-sys/include/sctp/socket.h"
#include "libwebrtc-sys/src/sctp/socket.rs.h"


void ArcasSctpPacketObserverProxyWrapper::OnSentPacket(dcsctp::TimeMs now,
                                                       rtc::ArrayView<const uint8_t> payload)
{
    _proxy->on_sent_packet(now.value(), rust::Slice<const uint8_t>(payload.data(), payload.size()));
}

void ArcasSctpPacketObserverProxyWrapper::OnReceivedPacket(dcsctp::TimeMs now,
                                                           rtc::ArrayView<const uint8_t> payload)
{
    _proxy->on_received_packet(now.value(),
                               rust::Slice<const uint8_t>(payload.data(), payload.size()));
}

void ArcasSctpTimeoutProxyWrapper::Start(dcsctp::DurationMs duration, dcsctp::TimeoutID timeout_id)
{
    _proxy->start(duration.value(), timeout_id.value());
};

void ArcasSctpTimeoutProxyWrapper::Stop()
{
    _proxy->stop();
};

void ArcasSctpCallbacksProxyWrapper::SendPacket(rtc::ArrayView<const uint8_t> data)
{
    _proxy->send_packet(rust::Slice<const uint8_t>(data.data(), data.size()));
}

std::unique_ptr<dcsctp::Timeout> ArcasSctpCallbacksProxyWrapper::CreateTimeout()
{
    auto timeout = _proxy->create_timeout();
    return std::make_unique<ArcasSctpTimeoutProxyWrapper>(std::move(timeout));
}

dcsctp::TimeMs ArcasSctpCallbacksProxyWrapper::TimeMillis()
{
    return dcsctp::TimeMs(_proxy->current_time_ms());
}

uint32_t ArcasSctpCallbacksProxyWrapper::GetRandomInt(uint32_t low, uint32_t high)
{
    return _proxy->get_random_int(low, high);
}

void ArcasSctpCallbacksProxyWrapper::OnMessageReceived(dcsctp::DcSctpMessage message)
{
    auto stream_id = message.stream_id().value();
    auto ppid = message.ppid().value();
    auto payload = std::make_unique<std::vector<uint8_t>>(std::move(message).ReleasePayload());
    _proxy->on_message_received(ArcasSctpMessage{stream_id, ppid, std::move(payload)});
}

void ArcasSctpCallbacksProxyWrapper::OnError(dcsctp::ErrorKind error, absl::string_view message)
{
    _proxy->on_error(error, rust::String(message.data()));
}

void ArcasSctpCallbacksProxyWrapper::OnAborted(dcsctp::ErrorKind error, absl::string_view message)
{
    _proxy->on_error(error, rust::String(message.data()));
}

void ArcasSctpCallbacksProxyWrapper::OnConnected()
{
    _proxy->on_connected();
}

void ArcasSctpCallbacksProxyWrapper::OnClosed()
{
    _proxy->on_closed();
}

void ArcasSctpCallbacksProxyWrapper::OnConnectionRestarted()
{
    _proxy->on_connection_restarted();
}

void ArcasSctpCallbacksProxyWrapper::OnStreamsResetFailed(
    rtc::ArrayView<const dcsctp::StreamID> outgoing_streams, absl::string_view reason)
{
    rust::Vec<uint16_t> vec;

    for (auto stream_id : outgoing_streams) { vec.push_back(stream_id.value()); }
    _proxy->on_streams_reset_failed(std::move(vec), rust::String(reason.data()));
};

void ArcasSctpCallbacksProxyWrapper::OnStreamsResetPerformed(
    rtc::ArrayView<const dcsctp::StreamID> outgoing_streams)
{
    rust::Vec<uint16_t> vec;
    for (auto stream_id : outgoing_streams) { vec.push_back(stream_id.value()); }
    _proxy->on_streams_reset_performed(std::move(vec));
}
void ArcasSctpCallbacksProxyWrapper::OnIncomingStreamsReset(
    rtc::ArrayView<const dcsctp::StreamID> incoming_streams)
{
    rust::Vec<uint16_t> vec;
    for (auto stream_id : incoming_streams) { vec.push_back(stream_id.value()); }
    _proxy->on_incoming_streams_reset(std::move(vec));
}

void ArcasSctpCallbacksProxyWrapper::OnBufferedAmountLow(dcsctp::StreamID stream_id)
{
    _proxy->on_buffered_amount_low(stream_id.value());
}
void ArcasSctpCallbacksProxyWrapper::OnTotalBufferedAmountLow()
{
    _proxy->on_total_bufferred_amount_low();
}

std::unique_ptr<ArcasSctpCallbacksProxyWrapper>
create_arcas_sctp_callback_wrapper(rust::Box<ArcasRustSctpCallbacksProxy> proxy)
{
    return std::make_unique<ArcasSctpCallbacksProxyWrapper>(std::move(proxy));
}

std::unique_ptr<ArcasSctpTimeoutProxyWrapper>
create_arcas_sctp_timeout(rust::Box<ArcasRustSctpTimeoutProxy> proxy)
{
    return std::make_unique<ArcasSctpTimeoutProxyWrapper>(std::move(proxy));
}


std::unique_ptr<ArcasSctpPacketObserverProxyWrapper>
create_arcas_sctp_packet_observer(rust::Box<ArcasRustSctpPacketObserverProxy> proxy)
{
    return std::make_unique<ArcasSctpPacketObserverProxyWrapper>(std::move(proxy));
}

std::unique_ptr<ArcasSctpOptions> create_arcas_sctp_options()
{
    return std::make_unique<ArcasSctpOptions>();
}


std::unique_ptr<ArcasSctpSendOptions> create_arcas_sctp_send_options()
{
    return std::make_unique<ArcasSctpSendOptions>();
}


ArcasSctpSocket::ArcasSctpSocket(
    rust::String prefix,
    ArcasSctpCallbacksProxyWrapper& callbacks,
    std::unique_ptr<ArcasSctpPacketObserverProxyWrapper> packet_observer,
    ArcasSctpOptions& options)
: _socket(std::make_unique<dcsctp::DcSctpSocket>(
    prefix.data(), callbacks, std::move(packet_observer), options.ref()))
{
}

void ArcasSctpSocket::receive_packet(rust::Slice<const uint8_t> packet)
{
    rtc::ArrayView<const uint8_t> view(packet.data(), packet.size());
    _socket->ReceivePacket(view);
}
void ArcasSctpSocket::handle_timeout(uint64_t timeout_id)
{
    _socket->HandleTimeout(dcsctp::TimeoutID(timeout_id));
}

void ArcasSctpSocket::connect()
{
    _socket->Connect();
}

void ArcasSctpSocket::shutdown()
{
    _socket->Shutdown();
}

void ArcasSctpSocket::close()
{
    _socket->Close();
}


dcsctp::SendStatus ArcasSctpSocket::send(uint16_t stream_id,
                                         uint32_t ppid,
                                         rust::Vec<uint8_t> payload,
                                         ArcasSctpSendOptions& options)

{
    // TODO: Is there some way to avoid copying these bytes between rust and C++?
    std::vector<uint8_t> cxx_vec(payload.begin(), payload.end());
    dcsctp::DcSctpMessage msg{dcsctp::StreamID(stream_id), dcsctp::PPID(ppid), cxx_vec};

    return _socket->Send(std::move(msg), options.ref());
}

dcsctp::ResetStreamsStatus ArcasSctpSocket::reset_streams(rust::Vec<uint16_t> incoming_streams)
{
    std::vector<dcsctp::StreamID> view;

    for (auto stream_id : incoming_streams) { view.push_back(dcsctp::StreamID(stream_id)); }


    return _socket->ResetStreams(rtc::ArrayView<const dcsctp::StreamID>(view.data(), view.size()));
}

dcsctp::SocketState ArcasSctpSocket::state() const
{
    return _socket->state();
}
void ArcasSctpSocket::set_max_message_size(size_t size)
{
    _socket->SetMaxMessageSize(size);
}
size_t ArcasSctpSocket::buffered_amount(uint16_t stream_id) const
{
    return _socket->buffered_amount(dcsctp::StreamID(stream_id));
}
size_t ArcasSctpSocket::buffered_amount_low_threshold(uint16_t stream_id) const
{
    return _socket->buffered_amount_low_threshold(dcsctp::StreamID(stream_id));
}
void ArcasSctpSocket::set_buffered_amount_low_threshold(uint16_t stream_id, size_t size)
{
    _socket->SetBufferedAmountLowThreshold(dcsctp::StreamID(stream_id), size);
}
ArcasSctpMetrics ArcasSctpSocket::get_metrics() const
{
    ArcasSctpMetrics rust_metrics;
    auto metrics = _socket->GetMetrics();

    rust_metrics.tx_packets_count = metrics.tx_packets_count;
    rust_metrics.tx_messages_count = metrics.tx_messages_count;
    // rust_metrics.cwnd_bytes = metrics.cwnd_bytes;

    if (metrics.cwnd_bytes.has_value())
    {
        rust_metrics.cwnd_bytes.push_back(metrics.cwnd_bytes.value());
    }

    if (metrics.srtt_ms.has_value())
    {
        rust_metrics.srtt_ms.push_back(metrics.srtt_ms.value());
    }

    rust_metrics.unack_data_count = metrics.unack_data_count;
    rust_metrics.rx_packets_count = metrics.rx_packets_count;
    rust_metrics.rx_messages_count = metrics.rx_messages_count;

    if (metrics.peer_rwnd_bytes.has_value())
    {
        rust_metrics.peer_rwnd_bytes.push_back(metrics.peer_rwnd_bytes.value());
    }

    return rust_metrics;
}

uint32_t ArcasSctpSocket::verification_tag() const
{
    return _socket->verification_tag().value();
}


std::unique_ptr<ArcasSctpSocket>
create_arcas_sctp_socket(rust::String prefix,
                         ArcasSctpCallbacksProxyWrapper& callbacks,
                         std::unique_ptr<ArcasSctpPacketObserverProxyWrapper> packet_observer,
                         ArcasSctpOptions& options)
{
    return std::make_unique<ArcasSctpSocket>(prefix,
                                             callbacks,
                                             std::move(packet_observer),
                                             options);
}
