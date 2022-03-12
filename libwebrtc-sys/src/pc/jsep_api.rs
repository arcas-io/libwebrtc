use crate::candidate::ffi::{create_arcas_candidate, CandidateComponent};
use cxx::UniquePtr;

#[cxx::bridge]
pub mod ffi {
    #[derive(Debug)]
    #[repr(u32)]
    #[namespace = "webrtc"]
    enum SdpType {
        kOffer,
        // Description must be treated as an SDP offer.
        kPrAnswer,
        // Description must be treated as an SDP answer, but not a final
        // answer.
        kAnswer,
        // Description must be treated as an SDP final answer, and the
        // offer-answer exchange must be considered complete after
        // receiving this.
        kRollback, // Resets any pending offers and sets signaling state back to
                   // stable.
    }

    struct ArcasCandidateWrapper {
        pub(crate) ptr: UniquePtr<ArcasCandidate>,
    }

    struct ArcasCandidatePairChangeEventJSEP {
        local_candidate: UniquePtr<ArcasCandidate>,
        remote_candidate: UniquePtr<ArcasCandidate>,
        last_data_received_ms: i64,
        reason: String,
        estimated_disconnected_time_ms: i64,
    }

    unsafe extern "C++" {
        include!("include/pc/jsep_api.h");

        type ArcasJsepTransportControllerConfig;
        type ArcasJsepTransportController;
        type ArcasJsepTransportControllerObserver;
        type ArcasCxxSSLHandshakeError = crate::rtc_base::base::ffi::ArcasCxxSSLHandshakeError;
        type ArcasCxxBundlePolicy = crate::peer_connection_factory::ffi::ArcasCxxBundlePolicy;
        type ArcasCxxRtcpMuxPolicy = crate::peer_connection_factory::ffi::ArcasCxxRtcpMuxPolicy;
        #[namespace = "rtc"]
        type Thread = crate::rtc_base::base::ffi::Thread;
        #[namespace = "webrtc"]
        type IceTransportFactory = crate::ice_transport::ffi::IceTransportFactory;
        type ArcasDTLSTransport;
        #[namespace = "rtc"]
        type CopyOnWriteBuffer;
        #[namespace = "webrtc"]
        type RtpTransportInternal;
        #[namespace = "webrtc"]
        type SrtpTransport;
        #[namespace = "webrtc"]
        type DataChannelTransportInterface;
        #[namespace = "cricket"]
        type DtlsTransportInternal;
        #[namespace = "cricket"]
        type PortAllocator;
        #[namespace = "rtc"]
        type NetworkManager = crate::rtc_base::base::ffi::NetworkManager;
        #[namespace = "webrtc"]
        type SdpType;
        #[namespace = "webrtc"]
        type AsyncDnsResolverFactoryInterface =
            crate::async_dns_resolver_factory::ffi::AsyncDnsResolverFactoryInterface;
        type ArcasSessionDescription = crate::session_description::ffi::ArcasSessionDescription;
        type ArcasRTCError = crate::error::ffi::ArcasRTCError;
        type ArcasP2PIceConfig = crate::p2p::ice_transport_internal::ffi::ArcasP2PIceConfig;
        type ArcasSSLCertificate = crate::rtc_base::certificates::ffi::ArcasSSLCertificate;
        type ArcasCandidate = crate::candidate::ffi::ArcasCandidate;
        #[namespace = "rtc"]
        type SSLRole = crate::rtc_base::certificates::ffi::SSLRole;
        #[namespace = "cricket"]
        type IceConnectionState = crate::p2p::ice_transport_internal::ffi::IceConnectionState;
        type ArcasPeerConnectionState =
            crate::peer_connection_observer::ffi::ArcasPeerConnectionState;
        type ArcasIceConnectionState = crate::shared_bridge::ffi::ArcasIceConnectionState;
        #[namespace = "cricket"]
        type IceGatheringState = crate::p2p::ice_transport_internal::ffi::IceGatheringState;

        // ArcasJsepTransportControllerConfig
        fn create_arcas_jsep_transport_controller_config(
        ) -> UniquePtr<ArcasJsepTransportControllerConfig>;

        fn set_redetermine_role_on_ice_restart(
            self: Pin<&mut ArcasJsepTransportControllerConfig>,
            restart: bool,
        );

        fn set_bundle_policy(
            self: Pin<&mut ArcasJsepTransportControllerConfig>,
            bundle_policy: ArcasCxxBundlePolicy,
        );

        fn set_rtcp_mux_policy(
            self: Pin<&mut ArcasJsepTransportControllerConfig>,
            rtcp_mux_policy: ArcasCxxRtcpMuxPolicy,
        );

        fn set_ice_transport_factory(
            self: Pin<&mut ArcasJsepTransportControllerConfig>,
            ice_transport_factory: UniquePtr<IceTransportFactory>,
        );

        fn set_transport_observer(
            self: Pin<&mut ArcasJsepTransportControllerConfig>,
            obs: UniquePtr<ArcasJsepTransportControllerObserver>,
        );

        fn set_rtcp_handler(
            self: Pin<&mut ArcasJsepTransportControllerConfig>,
            handler: Box<ArcasRustJsepRTCPHandler>,
        );

        fn set_dtls_handshake_error_handler(
            self: Pin<&mut ArcasJsepTransportControllerConfig>,
            callback: Box<ArcasRustDTLSHandshakeErrorHandler>,
        );

        fn create_buffer(capacity: u64) -> UniquePtr<CopyOnWriteBuffer>;
        fn create_buffer_with_data(bytes: &[u8]) -> UniquePtr<CopyOnWriteBuffer>;

        fn send_rtp_packet(
            transport: Pin<&mut SrtpTransport>,
            packet: Pin<&mut CopyOnWriteBuffer>,
            on_net_thr: Pin<&mut Thread>,
        ) -> bool;

        fn set_rtp_params(
            transport: Pin<&mut SrtpTransport>,
            send_cs: i32,
            send_key: String,
            recv_cs: i32,
            recv_key: String,
            recv_extension_ids: Vec<i32>,
            on_net_thr: Pin<&mut Thread>,
        );

        fn get_transport_name(transport: &SrtpTransport, on_net_thr: Pin<&mut Thread>) -> String;
        fn is_writable(transport: &SrtpTransport) -> bool;

        // port allocator

        /// # Safety
        ///
        /// NetworkManager must outlive the port allocator.
        ///
        unsafe fn create_arcas_cxx_port_allocator(
            network_manager_ptr: *mut NetworkManager,
        ) -> UniquePtr<PortAllocator>;

        #[cxx_name = "SetPortRange"]
        pub fn set_port_range(self: Pin<&mut PortAllocator>, min_port: i32, max_port: i32) -> bool;

        pub fn init_port_alloc(
            port_alloc: Pin<&mut PortAllocator>,
            network_thread: Pin<&mut Thread>,
        );

        // transport controller

        /// # Safety
        ///
        /// It is expected that the pointers passed here will not be deallocated
        /// while the transport controller is using them.
        unsafe fn create_arcas_jsep_transport_controller(
            network_thread: *mut Thread,
            port_allocator: UniquePtr<PortAllocator>,
            async_dns_resolver_factory: *mut AsyncDnsResolverFactoryInterface,
            config: UniquePtr<ArcasJsepTransportControllerConfig>,
        ) -> UniquePtr<ArcasJsepTransportController>;

        fn set_remote_description(
            self: Pin<&mut ArcasJsepTransportController>,
            sdp_type: SdpType,
            sdp: &ArcasSessionDescription,
        ) -> UniquePtr<ArcasRTCError>;

        fn set_local_description(
            self: Pin<&mut ArcasJsepTransportController>,
            sdp_type: SdpType,
            sdp: &ArcasSessionDescription,
        ) -> UniquePtr<ArcasRTCError>;

        fn get_srtp_transport(
            self: &ArcasJsepTransportController,
            mid: String,
        ) -> *mut SrtpTransport;

        fn get_data_channel_transport(
            self: &ArcasJsepTransportController,
            mid: String,
        ) -> *mut DataChannelTransportInterface;

        fn set_ice_config(
            self: Pin<&mut ArcasJsepTransportController>,
            config: UniquePtr<ArcasP2PIceConfig>,
        );
        fn set_needs_ice_restart_flag(self: Pin<&mut ArcasJsepTransportController>);
        fn needs_ice_restart(self: &ArcasJsepTransportController, mid: String) -> bool;
        fn maybe_start_gathering(self: Pin<&mut ArcasJsepTransportController>);
        fn set_local_certificate(
            self: Pin<&mut ArcasJsepTransportController>,
            certificate: UniquePtr<ArcasSSLCertificate>,
        );
        fn get_dtls_role(self: &ArcasJsepTransportController, mid: String) -> Vec<SSLRole>;
        fn get_local_certificate(
            self: &ArcasJsepTransportController,
            mid: String,
        ) -> UniquePtr<ArcasSSLCertificate>;

        fn add_remote_candidates(
            self: Pin<&mut ArcasJsepTransportController>,
            mid: String,
            candidates: Vec<ArcasCandidateWrapper>,
        ) -> UniquePtr<ArcasRTCError>;

        fn remove_remote_candidates(
            self: Pin<&mut ArcasJsepTransportController>,
            candidates: Vec<ArcasCandidateWrapper>,
        ) -> UniquePtr<ArcasRTCError>;

        fn set_active_reset_srtp_params(self: Pin<&mut ArcasJsepTransportController>, active: bool);
        fn rollback_transports(
            self: Pin<&mut ArcasJsepTransportController>,
        ) -> UniquePtr<ArcasRTCError>;

        // gen unique ptr wrappers... Do not call these.
        fn gen_arcas_cxx_dtls_transport() -> UniquePtr<ArcasDTLSTransport>;

        fn to_cxx(
            r: Box<ArcasRustJsepTransportControllerObserver>,
        ) -> UniquePtr<ArcasJsepTransportControllerObserver>;
    }

    extern "Rust" {
        type ArcasRustJsepRTCPHandler;
        type ArcasRustDTLSHandshakeErrorHandler;
        type ArcasRustJsepTransportControllerObserver;
        type ArcasRustJsepTransportControllerObserverWrapper;

        // ArcasJsepRTCPHandler
        fn invoke(self: &ArcasRustJsepRTCPHandler, packet: &CopyOnWriteBuffer, packet_time: i64);
        // DTLSHandshakeErrorHandler
        fn invoke(self: &ArcasRustDTLSHandshakeErrorHandler, error_type: ArcasCxxSSLHandshakeError);

        unsafe fn invoke(
            self: &ArcasRustJsepTransportControllerObserver,
            mid: String,
            rtp_transport: *mut RtpTransportInternal,
            dtls_transport: UniquePtr<ArcasDTLSTransport>,
            data_channel_transport_interface: *mut DataChannelTransportInterface,
        ) -> bool;

        // ArcasRustJsepTransportControllerObserverWrapper
        fn ice_candidates_gathered(
            self: &ArcasRustJsepTransportControllerObserverWrapper,
            mid: String,
            candidate: Vec<ArcasCandidateWrapper>,
        );
        fn ice_connection_state(
            self: &ArcasRustJsepTransportControllerObserverWrapper,
            state: IceConnectionState,
        );
        fn connection_state(
            self: &ArcasRustJsepTransportControllerObserverWrapper,
            state: ArcasPeerConnectionState,
        );

        fn standardized_ice_connection_state(
            self: &ArcasRustJsepTransportControllerObserverWrapper,
            state: ArcasIceConnectionState,
        );

        fn ice_gathering_state(
            self: &ArcasRustJsepTransportControllerObserverWrapper,
            state: IceGatheringState,
        );

        fn ice_candidate_error(
            self: &ArcasRustJsepTransportControllerObserverWrapper,
            address: String,
            url: String,
            error_code: i32,
            error_text: String,
        );
        fn ice_candidates_removed(
            self: &ArcasRustJsepTransportControllerObserverWrapper,
            candidate: Vec<ArcasCandidateWrapper>,
        );
        fn ice_candidate_pair_change(
            self: &ArcasRustJsepTransportControllerObserverWrapper,
            event: ArcasCandidatePairChangeEventJSEP,
        );
    }

    impl Box<ArcasRustJsepRTCPHandler> {}

    impl Box<ArcasRustDTLSHandshakeErrorHandler> {}
}

impl Default for ffi::ArcasCandidateWrapper {
    fn default() -> Self {
        Self {
            ptr: create_arcas_candidate(),
        }
    }
}

impl ffi::ArcasCandidateWrapper {
    pub fn set_address(&mut self, host_colon_port: String) {
        self.ptr.pin_mut().set_address(host_colon_port);
    }
    pub fn set_component(&mut self, val: CandidateComponent) {
        self.ptr.pin_mut().set_component(val);
    }
    pub fn set_protocol(&mut self, val: String) {
        self.ptr.pin_mut().set_protocol(val);
    }
}

pub trait JsepTransportControllerObserver {
    fn ice_candidates_gathered(&self, mid: String, candidates: Vec<ffi::ArcasCandidateWrapper>);
    fn ice_connection_state(&self, state: ffi::IceConnectionState);
    fn connection_state(&self, state: ffi::ArcasPeerConnectionState);
    fn standardized_ice_connection_state(&self, state: ffi::ArcasIceConnectionState);
    fn ice_gathering_state(&self, state: ffi::IceGatheringState);
    fn ice_candidate_error(
        &self,
        address: String,
        url: String,
        error_code: i32,
        error_text: String,
    );
    fn ice_candidates_removed(&self, candidates: Vec<ffi::ArcasCandidateWrapper>);
    fn ice_candidate_pair_change(&self, event: ffi::ArcasCandidatePairChangeEventJSEP);
}

pub struct ArcasRustJsepTransportControllerObserverWrapper {
    observer: Box<dyn JsepTransportControllerObserver>,
}

impl ArcasRustJsepTransportControllerObserverWrapper {
    pub fn new(observer: Box<dyn JsepTransportControllerObserver>) -> Self {
        Self { observer }
    }

    pub fn ice_candidates_gathered(
        &self,
        mid: String,
        candidates: Vec<ffi::ArcasCandidateWrapper>,
    ) {
        self.observer.ice_candidates_gathered(mid, candidates);
    }

    pub fn ice_connection_state(&self, state: ffi::IceConnectionState) {
        self.observer.ice_connection_state(state);
    }

    pub fn connection_state(&self, state: ffi::ArcasPeerConnectionState) {
        self.observer.connection_state(state);
    }

    pub fn standardized_ice_connection_state(&self, state: ffi::ArcasIceConnectionState) {
        self.observer.standardized_ice_connection_state(state);
    }

    pub fn ice_gathering_state(&self, state: ffi::IceGatheringState) {
        self.observer.ice_gathering_state(state);
    }

    pub fn ice_candidate_error(
        &self,
        address: String,
        url: String,
        error_code: i32,
        error_text: String,
    ) {
        self.observer
            .ice_candidate_error(address, url, error_code, error_text);
    }

    pub fn ice_candidates_removed(&self, candidates: Vec<ffi::ArcasCandidateWrapper>) {
        self.observer.ice_candidates_removed(candidates);
    }

    pub fn ice_candidate_pair_change(&self, event: ffi::ArcasCandidatePairChangeEventJSEP) {
        self.observer.ice_candidate_pair_change(event);
    }
}

pub struct ArcasRustJsepRTCPHandler {
    inner: Box<dyn Fn(&ffi::CopyOnWriteBuffer, i64)>,
}

impl ArcasRustJsepRTCPHandler {
    pub fn new(inner: Box<dyn Fn(&ffi::CopyOnWriteBuffer, i64)>) -> Self {
        Self { inner }
    }

    pub fn invoke(&self, packet: &ffi::CopyOnWriteBuffer, packet_time: i64) {
        (self.inner)(packet, packet_time);
    }
}

pub struct ArcasRustDTLSHandshakeErrorHandler {
    inner: Box<dyn Fn(self::ffi::ArcasCxxSSLHandshakeError)>,
}

impl ArcasRustDTLSHandshakeErrorHandler {
    pub fn new(inner: Box<dyn Fn(self::ffi::ArcasCxxSSLHandshakeError)>) -> Self {
        Self { inner }
    }

    pub fn invoke(&self, error: self::ffi::ArcasCxxSSLHandshakeError) {
        (self.inner)(error);
    }
}

pub type OnTransportChanged = dyn Fn(
    String,
    *mut ffi::RtpTransportInternal,
    UniquePtr<ffi::ArcasDTLSTransport>,
    *mut ffi::DataChannelTransportInterface,
) -> bool;

pub struct ArcasRustJsepTransportControllerObserver {
    inner: Box<OnTransportChanged>,
}

impl ArcasRustJsepTransportControllerObserver {
    pub fn new(inner: Box<OnTransportChanged>) -> Self {
        Self { inner }
    }

    /// # Safety
    ///
    /// The mutable pointers must be used only on the network thread for the transport controller.
    ///
    pub unsafe fn invoke(
        &self,
        mid: String,
        rtp_transport: *mut ffi::RtpTransportInternal,
        dtls_transport: UniquePtr<ffi::ArcasDTLSTransport>,
        data_channel_transport_interface: *mut ffi::DataChannelTransportInterface,
    ) -> bool {
        (self.inner)(
            mid,
            rtp_transport,
            dtls_transport,
            data_channel_transport_interface,
        )
    }
}
