#pragma once
#include <iostream>
#include "net/dcsctp/socket/dcsctp_socket.h"
#include "rust/cxx.h"

struct ArcasRustSctpCallbacksProxy;
struct ArcasRustSctpTimeoutProxy;
struct ArcasRustSctpPacketObserverProxy;
struct ArcasSctpMessage;
struct ArcasSctpMetrics;

class ArcasSctpSendOptions
{
private:
    dcsctp::SendOptions _options;

public:
    ArcasSctpSendOptions(){};

    void set_unordered(bool unordered)
    {
        _options.unordered = dcsctp::IsUnordered(unordered);
    }

    void set_lifetime(int32_t lifetime)
    {
        _options.lifetime = dcsctp::DurationMs(lifetime);
    }

    void set_max_retransmissions(size_t max_retransmissions)
    {
        _options.max_retransmissions = max_retransmissions;
    }

    dcsctp::SendOptions& ref()
    {
        return _options;
    }
};

class ArcasSctpOptions
{
private:
    dcsctp::DcSctpOptions _options;

public:
    ArcasSctpOptions() {}

    dcsctp::DcSctpOptions& ref()
    {
        return _options;
    }

    void set_local_port(int port)
    {
        _options.local_port = port;
    }

    void set_remote_port(int port)
    {
        _options.remote_port = port;
    }

    void set_max_message_size(size_t size)
    {
        _options.max_message_size = size;
    }

    void set_max_receiver_window_buffer_size(size_t size)
    {
        _options.max_receiver_window_buffer_size = size;
    }

    void set_max_send_buffer_size(size_t size)
    {
        _options.max_send_buffer_size = size;
    }

    void set_total_buffered_amount_low_threshold(size_t size)
    {
        _options.total_buffered_amount_low_threshold = size;
    }

    void set_max_retransmissions(int max_retransmissions)
    {
        _options.max_retransmissions = max_retransmissions;
    }

    void set_enable_partial_reliability(bool enable_partial_reliability)
    {
        _options.enable_partial_reliability = enable_partial_reliability;
    }

    void set_heartbeat_interval(uint16_t heartbeat_interval)
    {
        _options.heartbeat_interval = dcsctp::DurationMs(heartbeat_interval);
    }
};

class ArcasSctpPacketObserverProxyWrapper : public dcsctp::PacketObserver
{
private:
    rust::Box<ArcasRustSctpPacketObserverProxy> _proxy;

public:
    ArcasSctpPacketObserverProxyWrapper(rust::Box<ArcasRustSctpPacketObserverProxy> proxy)
    : _proxy(std::move(proxy)){};

    // Called when a packet is sent, with the current time (in milliseconds) as
    // `now`, and the packet payload as `payload`.
    void OnSentPacket(dcsctp::TimeMs now, rtc::ArrayView<const uint8_t> payload);

    // Called when a packet is received, with the current time (in milliseconds)
    // as `now`, and the packet payload as `payload`.
    void OnReceivedPacket(dcsctp::TimeMs now, rtc::ArrayView<const uint8_t> payload);
};

class ArcasSctpTimeoutProxyWrapper : public dcsctp::Timeout
{
private:
    rust::Box<ArcasRustSctpTimeoutProxy> _proxy;

public:
    ArcasSctpTimeoutProxyWrapper(rust::Box<ArcasRustSctpTimeoutProxy> proxy)
    : _proxy(std::move(proxy))
    {
    }

    void Start(dcsctp::DurationMs duration, dcsctp::TimeoutID timeout_id) override;
    void Stop() override;
};

class ArcasSctpCallbacksProxyWrapper : public dcsctp::DcSctpSocketCallbacks
{
private:
    rust::Box<ArcasRustSctpCallbacksProxy> _proxy;


public:
    ArcasSctpCallbacksProxyWrapper(rust::Box<ArcasRustSctpCallbacksProxy> proxy)
    : _proxy(std::move(proxy))
    {
    }

    void SendPacket(rtc::ArrayView<const uint8_t> data);
    std::unique_ptr<dcsctp::Timeout> CreateTimeout() override;
    dcsctp::TimeMs TimeMillis() override;
    uint32_t GetRandomInt(uint32_t low, uint32_t high) override;
    void OnMessageReceived(dcsctp::DcSctpMessage message) override;
    void OnError(dcsctp::ErrorKind error, absl::string_view message) override;
    void OnAborted(dcsctp::ErrorKind error, absl::string_view message) override;
    void OnConnected() override;
    void OnClosed() override;
    void OnConnectionRestarted() override;
    void OnStreamsResetFailed(rtc::ArrayView<const dcsctp::StreamID> outgoing_streams,
                              absl::string_view reason) override;
    void OnStreamsResetPerformed(rtc::ArrayView<const dcsctp::StreamID> outgoing_streams) override;
    void OnIncomingStreamsReset(rtc::ArrayView<const dcsctp::StreamID> incoming_streams) override;
    void OnBufferedAmountLow(dcsctp::StreamID stream_id) override;
    void OnTotalBufferedAmountLow() override;
};

class ArcasSctpSocket
{
private:
    std::unique_ptr<dcsctp::DcSctpSocket> _socket;

public:
    ArcasSctpSocket(rust::String prefix,
                    ArcasSctpCallbacksProxyWrapper& callbacks,
                    std::unique_ptr<ArcasSctpPacketObserverProxyWrapper> packet_observer,
                    ArcasSctpOptions& options);

    ~ArcasSctpSocket()
    {
        std::cout << "Destructor called" << std::endl;
    }

    void receive_packet(rust::Slice<const uint8_t> data);
    void handle_timeout(uint64_t timeout_id);
    void connect();
    void shutdown();
    void close();
    dcsctp::SendStatus send(uint16_t stream_id,
                            uint32_t ppid,
                            rust::Vec<uint8_t> payload,
                            ArcasSctpSendOptions& options);
    dcsctp::ResetStreamsStatus reset_streams(rust::Vec<uint16_t> incoming_streams);
    dcsctp::SocketState state() const;
    void set_max_message_size(size_t size);
    size_t buffered_amount(uint16_t stream_id) const;
    size_t buffered_amount_low_threshold(uint16_t stream_id) const;
    void set_buffered_amount_low_threshold(uint16_t stream_id, size_t size);
    ArcasSctpMetrics get_metrics() const;
    uint32_t verification_tag() const;
};

std::unique_ptr<ArcasSctpCallbacksProxyWrapper>
create_arcas_sctp_callback_wrapper(rust::Box<ArcasRustSctpCallbacksProxy> proxy);

std::unique_ptr<ArcasSctpTimeoutProxyWrapper>
create_arcas_sctp_timeout(rust::Box<ArcasRustSctpTimeoutProxy> proxy);

std::unique_ptr<ArcasSctpPacketObserverProxyWrapper>
create_arcas_sctp_packet_observer(rust::Box<ArcasRustSctpPacketObserverProxy> proxy);

std::unique_ptr<ArcasSctpOptions> create_arcas_sctp_options();

std::unique_ptr<ArcasSctpSocket>
create_arcas_sctp_socket(rust::String prefix,
                         ArcasSctpCallbacksProxyWrapper& callbacks,
                         std::unique_ptr<ArcasSctpPacketObserverProxyWrapper> packet_observer,
                         ArcasSctpOptions& options);

std::unique_ptr<ArcasSctpSendOptions> create_arcas_sctp_send_options();
