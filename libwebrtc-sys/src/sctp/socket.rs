use self::ffi::{ArcasSctpMessage, ErrorKind};

#[cxx::bridge]
pub mod ffi {

    /// Message received from SCTP.
    struct ArcasSctpMessage {
        pub stream_id: u16,
        pub ppid: u32,
        pub payload: UniquePtr<CxxVector<u8>>,
    }

    /// Sctp metrcics information.
    struct ArcasSctpMetrics {
        pub tx_packets_count: usize,
        pub tx_messages_count: usize,
        pub cwnd_bytes: Vec<usize>,
        pub srtt_ms: Vec<i32>,
        pub unack_data_count: usize,
        pub rx_packets_count: usize,
        pub rx_messages_count: usize,
        pub peer_rwnd_bytes: Vec<u32>,
    }

    #[derive(Debug)]
    #[namespace = "dcsctp"]
    #[repr(u32)]
    enum SendStatus {
        /// The message was enqueued successfully. As sending the message is done
        /// asynchronously, this is no guarantee that the message has been actually
        /// sent.
        kSuccess,
        /// The message was rejected as the payload was empty (which is not allowed in
        /// SCTP).
        kErrorMessageEmpty,
        /// The message was rejected as the payload was larger than what has been set
        /// as `DcSctpOptions.max_message_size`.
        kErrorMessageTooLarge,
        /// The message could not be enqueued as the socket is out of resources. This
        /// mainly indicates that the send queue is full.
        kErrorResourceExhaustion,
        /// The message could not be sent as the socket is shutting down.
        kErrorShuttingDown,
    }

    #[derive(Debug, Copy, Clone)]
    #[namespace = "dcsctp"]
    #[repr(u32)]
    enum ErrorKind {
        /// Indicates that no error has occurred. This will never be the case when
        /// `OnError` or `OnAborted` is called.
        kNoError,
        /// There have been too many retries or timeouts, and the library has given up.
        kTooManyRetries,
        /// A command was received that is only possible to execute when the socket is
        /// connected, which it is not.
        kNotConnected,
        /// Parsing of the command or its parameters failed.
        kParseFailed,
        /// Commands are received in the wrong sequence, which indicates a
        /// synchronisation mismatch between the peers.
        kWrongSequence,
        /// The peer has reported an issue using ERROR or ABORT command.
        kPeerReported,
        /// The peer has performed a protocol violation.
        kProtocolViolation,
        /// The receive or send buffers have been exhausted.
        kResourceExhaustion,
        /// The client has performed an invalid operation.
        kUnsupportedOperation,
    }

    #[namespace = "dcsctp"]
    #[repr(u32)]
    #[derive(Debug)]
    enum ResetStreamsStatus {
        /// If the connection is not yet established, this will be returned.
        kNotConnected,
        /// Indicates that ResetStreams operation has been successfully initiated.
        kPerformed,
        /// Indicates that ResetStreams has failed as it's not supported by the peer.
        kNotSupported,
    }

    #[namespace = "dcsctp"]
    #[repr(u32)]
    #[derive(Debug)]
    enum SocketState {
        /// The socket is closed.
        kClosed,
        /// The socket has initiated a connection, which is not yet established. Note
        /// that for incoming connections and for reconnections when the socket is
        /// already connected, the socket will not transition to this state.
        kConnecting,
        /// The socket is connected, and the connection is established.
        kConnected,
        /// The socket is shutting down, and the connection is not yet closed.
        kShuttingDown,
    }

    unsafe extern "C++" {
        include!("include/sctp/socket.h");

        type ArcasSctpCallbacksProxyWrapper;
        type ArcasSctpTimeoutProxyWrapper;
        type ArcasSctpPacketObserverProxyWrapper;
        type ArcasSctpSendOptions;
        type ArcasSctpOptions;
        type ArcasSctpSocket;
        #[namespace = "dcsctp"]
        type SendStatus;
        #[namespace = "dcsctp"]
        type ResetStreamsStatus;
        #[namespace = "dcsctp"]
        type SocketState;
        #[namespace = "dcsctp"]
        type ErrorKind;

        fn create_arcas_sctp_options() -> UniquePtr<ArcasSctpOptions>;

        fn create_arcas_sctp_callback_wrapper(
            proxy: Box<ArcasRustSctpCallbacksProxy>,
        ) -> UniquePtr<ArcasSctpCallbacksProxyWrapper>;

        fn create_arcas_sctp_timeout(
            proxy: Box<ArcasRustSctpTimeoutProxy>,
        ) -> UniquePtr<ArcasSctpTimeoutProxyWrapper>;

        fn create_arcas_sctp_packet_observer(
            proxy: Box<ArcasRustSctpPacketObserverProxy>,
        ) -> UniquePtr<ArcasSctpPacketObserverProxyWrapper>;

        fn create_arcas_sctp_send_options() -> UniquePtr<ArcasSctpSendOptions>;

        fn create_arcas_sctp_socket(
            prefix: String,
            callbacks: Pin<&mut ArcasSctpCallbacksProxyWrapper>,
            packet_observer: UniquePtr<ArcasSctpPacketObserverProxyWrapper>,
            options: Pin<&mut ArcasSctpOptions>,
        ) -> UniquePtr<ArcasSctpSocket>;

        /// ArcasSctpOptions
        fn set_local_port(self: Pin<&mut ArcasSctpOptions>, port: i32);
        fn set_remote_port(self: Pin<&mut ArcasSctpOptions>, port: i32);
        fn set_max_message_size(self: Pin<&mut ArcasSctpOptions>, size: usize);
        fn set_max_receiver_window_buffer_size(self: Pin<&mut ArcasSctpOptions>, size: usize);
        fn set_max_send_buffer_size(self: Pin<&mut ArcasSctpOptions>, size: usize);
        fn set_total_buffered_amount_low_threshold(self: Pin<&mut ArcasSctpOptions>, size: usize);
        fn set_max_retransmissions(self: Pin<&mut ArcasSctpOptions>, retransmissions: i32);
        fn set_enable_partial_reliability(self: Pin<&mut ArcasSctpOptions>, enable: bool);
        fn set_heartbeat_interval(self: Pin<&mut ArcasSctpOptions>, interval: u16);

        /// ArcasSctpSendOptions
        fn set_unordered(self: Pin<&mut ArcasSctpSendOptions>, unordered: bool);
        fn set_lifetime(self: Pin<&mut ArcasSctpSendOptions>, lifetime: i32);
        fn set_max_retransmissions(self: Pin<&mut ArcasSctpSendOptions>, retransmissions: usize);

        /// ArcasSctpSocket
        fn receive_packet(self: Pin<&mut ArcasSctpSocket>, packet: &[u8]);
        fn handle_timeout(self: Pin<&mut ArcasSctpSocket>, timeout: u64);
        fn connect(self: Pin<&mut ArcasSctpSocket>);
        fn shutdown(self: Pin<&mut ArcasSctpSocket>);
        fn close(self: Pin<&mut ArcasSctpSocket>);
        fn send(
            self: Pin<&mut ArcasSctpSocket>,
            stream_id: u16,
            ppid: u32,
            payload: Vec<u8>,
            options: Pin<&mut ArcasSctpSendOptions>,
        ) -> SendStatus;
        fn reset_streams(self: Pin<&mut ArcasSctpSocket>, streams: Vec<u16>) -> ResetStreamsStatus;
        fn state(self: &ArcasSctpSocket) -> SocketState;
        fn set_max_message_size(self: Pin<&mut ArcasSctpSocket>, size: usize);
        fn buffered_amount(self: &ArcasSctpSocket, stream_id: u16) -> usize;
        fn set_buffered_amount_low_threshold(
            self: Pin<&mut ArcasSctpSocket>,
            stream_id: u16,
            size: usize,
        );
        fn get_metrics(self: &ArcasSctpSocket) -> ArcasSctpMetrics;
        fn verification_tag(self: &ArcasSctpSocket) -> u32;

    }

    extern "Rust" {
        type ArcasRustSctpTimeoutProxy;
        type ArcasRustSctpCallbacksProxy;
        type ArcasRustSctpPacketObserverProxy;

        /// ArcasRustSctpTimeout
        fn start(self: &mut ArcasRustSctpTimeoutProxy, timeout_ms: i32, timeout_id: u64);
        fn stop(self: &mut ArcasRustSctpTimeoutProxy);

        /// ArcasRustSctpCallbacksProxy
        fn send_packet(self: &mut ArcasRustSctpCallbacksProxy, packet: &[u8]);
        fn create_timeout(self: &mut ArcasRustSctpCallbacksProxy)
            -> Box<ArcasRustSctpTimeoutProxy>;
        fn current_time_ms(self: &mut ArcasRustSctpCallbacksProxy) -> i64;
        fn get_random_int(self: &mut ArcasRustSctpCallbacksProxy, low: u32, high: u32) -> u32;
        fn on_message_received(self: &mut ArcasRustSctpCallbacksProxy, message: ArcasSctpMessage);
        fn on_error(self: &mut ArcasRustSctpCallbacksProxy, error_kind: ErrorKind, message: String);
        fn on_aborted(
            self: &mut ArcasRustSctpCallbacksProxy,
            error_kind: ErrorKind,
            message: String,
        );
        fn on_connected(self: &mut ArcasRustSctpCallbacksProxy);
        fn on_closed(self: &mut ArcasRustSctpCallbacksProxy);
        fn on_connection_restarted(self: &mut ArcasRustSctpCallbacksProxy);
        fn on_streams_reset_failed(
            self: &mut ArcasRustSctpCallbacksProxy,
            stream_ids: Vec<u16>,
            error: String,
        );
        fn on_streams_reset_performed(self: &mut ArcasRustSctpCallbacksProxy, stream_ids: Vec<u16>);
        fn on_incoming_streams_reset(self: &mut ArcasRustSctpCallbacksProxy, stream_ids: Vec<u16>);
        fn on_buffered_amount_low(self: &mut ArcasRustSctpCallbacksProxy, stream_id: u16);
        fn on_total_bufferred_amount_low(self: &mut ArcasRustSctpCallbacksProxy);

        /// ArcasRustSctpPacketObserverProxy
        fn on_received_packet(
            self: &mut ArcasRustSctpPacketObserverProxy,
            time_ms: i64,
            packet: &[u8],
        );
        fn on_sent_packet(self: &mut ArcasRustSctpPacketObserverProxy, time_ms: i64, packet: &[u8]);
    }
}

pub trait SctpPacketObserver {
    /// Called when a packet is received, with the current time (in milliseconds)
    /// as `now`, and the packet payload as `payload`.
    fn on_received_packet(&mut self, time_ms: i64, packet: &[u8]);

    /// Called when a packet is sent, with the current time (in milliseconds) as
    /// `now`, and the packet payload as `payload`.
    fn on_sent_packet(&mut self, time_ms: i64, packet: &[u8]);
}

pub struct ArcasRustSctpPacketObserverProxy {
    proxy: Box<dyn SctpPacketObserver>,
}

impl ArcasRustSctpPacketObserverProxy {
    pub fn new(proxy: Box<dyn SctpPacketObserver>) -> Self {
        Self { proxy }
    }

    /// Called when a packet is received, with the current time (in milliseconds)
    /// as `now`, and the packet payload as `payload`.
    pub fn on_received_packet(&mut self, time_ms: i64, packet: &[u8]) {
        self.proxy.on_received_packet(time_ms, packet);
    }

    /// Called when a packet is sent, with the current time (in milliseconds) as
    /// `now`, and the packet payload as `payload`.
    pub fn on_sent_packet(&mut self, time_ms: i64, packet: &[u8]) {
        self.proxy.on_sent_packet(time_ms, packet);
    }
}

/// "Timeout" has a very specific job which is to call the HandleTimeout method
/// on the sctp socket interface.  This is left up to the implementation so an
/// optimal timer system can be used.
pub trait SctpTimeout {
    /// Called to start time timeout, with the duration in milliseconds as
    /// `duration` and with the timeout identifier as `timeout_id`, which - if
    /// the timeout expires - shall be provided to `DcSctpSocket::HandleTimeout`.
    ///
    /// `Start` and `Stop` will always be called in pairs. In other words will
    /// ´Start` never be called twice, without a call to `Stop` in between.
    fn start(&mut self, timeout_ms: i32, timeout_id: u64);

    /// Called to stop the running timeout.
    ///
    /// `Start` and `Stop` will always be called in pairs. In other words will
    /// ´Start` never be called twice, without a call to `Stop` in between.
    ///
    /// `Stop` will always be called prior to releasing this object.
    fn stop(&mut self);
}

pub struct ArcasRustSctpTimeoutProxy {
    timeout: Box<dyn SctpTimeout>,
}

impl ArcasRustSctpTimeoutProxy {
    pub fn new(timeout: Box<dyn SctpTimeout>) -> Self {
        Self { timeout }
    }

    pub fn start(&mut self, timeout: i32, timeout_id: u64) {
        self.timeout.start(timeout, timeout_id);
    }

    pub fn stop(&mut self) {
        self.timeout.stop();
    }
}

pub trait SctpSocketCallbacks {
    /// Called when the library wants the packet serialized as `data` to be sent.
    ///
    /// TODO(bugs.webrtc.org/12943): This method is deprecated, see
    /// `SendPacketWithStatus`.
    ///
    /// Note that it's NOT ALLOWED to call into this library from within this
    /// callback.
    fn send_packet(&mut self, packet: &[u8]);

    /// Called when the library wants to create a Timeout. The callback must return
    /// an object that implements that interface.
    ///
    /// Note that it's NOT ALLOWED to call into this library from within this
    /// callback.
    fn create_timeout(&mut self) -> Box<ArcasRustSctpTimeoutProxy>;

    /// Returns the current time in milliseconds (from any epoch).
    ///
    /// Note that it's NOT ALLOWED to call into this library from within this
    /// callback.
    fn current_time_ms(&mut self) -> i64;

    /// Called when the library needs a random number uniformly distributed between
    /// `low` (inclusive) and `high` (exclusive). The random numbers used by the
    /// library are not used for cryptographic purposes. There are no requirements
    /// that the random number generator must be secure.
    ///
    /// Note that it's NOT ALLOWED to call into this library from within this
    /// callback.
    fn get_random_int(&mut self, low: u32, high: u32) -> u32;

    /// Called when the library has received an SCTP message in full and delivers
    /// it to the upper layer.
    ///
    /// It is allowed to call into this library from within this callback.
    fn on_message_received(&mut self, message: ArcasSctpMessage);

    /// Triggered when an non-fatal error is reported by either this library or
    /// from the other peer (by sending an ERROR command). These should be logged,
    /// but no other action need to be taken as the association is still viable.
    ///
    /// It is allowed to call into this library from within this callback.
    fn on_error(&mut self, error_kind: ffi::ErrorKind, message: String);

    /// Triggered when the socket has aborted - either as decided by this socket
    /// due to e.g. too many retransmission attempts, or by the peer when
    /// receiving an ABORT command. No other callbacks will be done after this
    /// callback, unless reconnecting.
    ///
    /// It is allowed to call into this library from within this callback.
    fn on_aborted(&mut self, error_kind: ffi::ErrorKind, message: String);

    /// Called when calling `Connect` succeeds, but also for incoming successful
    /// connection attempts.
    ///
    /// It is allowed to call into this library from within this callback.
    fn on_connected(&mut self);

    /// Called when the socket is closed in a controlled way. No other
    /// callbacks will be done after this callback, unless reconnecting.
    ///
    /// It is allowed to call into this library from within this callback.
    fn on_closed(&mut self);

    /// On connection restarted (by peer). This is just a notification, and the
    /// association is expected to work fine after this call, but there could have
    /// been packet loss as a result of restarting the association.
    ///
    /// It is allowed to call into this library from within this callback.
    fn on_connection_restarted(&mut self);

    /// Indicates that a stream reset request has failed.
    ///
    /// It is allowed to call into this library from within this callback.
    fn on_streams_reset_failed(&mut self, stream_ids: Vec<u16>, error: String);

    /// Indicates that a stream reset request has been performed.
    ///
    /// It is allowed to call into this library from within this callback.
    fn on_streams_reset_performed(&mut self, stream_ids: Vec<u16>);

    /// When a peer has reset some of its outgoing streams, this will be called. An
    /// empty list indicates that all streams have been reset.
    ///
    /// It is allowed to call into this library from within this callback.
    fn on_incoming_streams_reset(&mut self, stream_ids: Vec<u16>);

    /// Will be called when the amount of data buffered to be sent falls to or
    /// below the threshold set when calling `SetBufferedAmountLowThreshold`.
    ///
    /// It is allowed to call into this library from within this callback.
    fn on_buffered_amount_low(&mut self, stream_id: u16);

    /// Will be called when the total amount of data buffered (in the entire send
    /// buffer, for all streams) falls to or below the threshold specified in
    /// `DcSctpOptions::total_buffered_amount_low_threshold`.
    fn on_total_bufferred_amount_low(&mut self);
}

pub struct ArcasRustSctpCallbacksProxy {
    proxy: Box<dyn SctpSocketCallbacks>,
}

impl ArcasRustSctpCallbacksProxy {
    pub fn new(proxy: Box<dyn SctpSocketCallbacks>) -> Self {
        Self { proxy }
    }

    fn send_packet(&mut self, packet: &[u8]) {
        self.proxy.send_packet(packet);
    }

    fn create_timeout(&mut self) -> Box<ArcasRustSctpTimeoutProxy> {
        self.proxy.create_timeout()
    }

    fn current_time_ms(&mut self) -> i64 {
        self.proxy.current_time_ms()
    }

    fn get_random_int(&mut self, low: u32, high: u32) -> u32 {
        self.proxy.get_random_int(low, high)
    }

    fn on_message_received(&mut self, message: ArcasSctpMessage) {
        self.proxy.on_message_received(message);
    }

    fn on_error(&mut self, error_kind: ffi::ErrorKind, message: String) {
        self.proxy.on_error(error_kind, message);
    }

    fn on_aborted(&mut self, error_kind: ffi::ErrorKind, message: String) {
        self.proxy.on_aborted(error_kind, message);
    }

    fn on_connected(&mut self) {
        self.proxy.on_connected();
    }

    fn on_closed(&mut self) {
        self.proxy.on_closed();
    }

    fn on_connection_restarted(&mut self) {
        self.proxy.on_connection_restarted();
    }

    fn on_streams_reset_failed(&mut self, stream_ids: Vec<u16>, error: String) {
        self.proxy.on_streams_reset_failed(stream_ids, error);
    }

    fn on_streams_reset_performed(&mut self, stream_ids: Vec<u16>) {
        self.proxy.on_streams_reset_performed(stream_ids);
    }

    fn on_incoming_streams_reset(&mut self, stream_ids: Vec<u16>) {
        self.proxy.on_incoming_streams_reset(stream_ids);
    }

    fn on_buffered_amount_low(&mut self, stream_id: u16) {
        self.proxy.on_buffered_amount_low(stream_id);
    }

    fn on_total_bufferred_amount_low(&mut self) {
        self.proxy.on_total_bufferred_amount_low();
    }
}

unsafe impl Send for ffi::ArcasSctpCallbacksProxyWrapper {}
unsafe impl Send for ffi::ArcasSctpSocket {}
unsafe impl Send for ffi::ArcasSctpSendOptions {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_arcas_sctp_options() {
        let mut options = ffi::create_arcas_sctp_options();
        options.pin_mut().set_enable_partial_reliability(false);
        options.pin_mut().set_local_port(1234);
        options.pin_mut().set_remote_port(1234);
    }
}
