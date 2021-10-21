use std::error;
use std::fmt;
use std::sync::Arc;

use cxx::memory::SharedPtrTarget;
use cxx::CxxString;
use cxx::CxxVector;
use cxx::SharedPtr;
use cxx::UniquePtr;
use parking_lot::lock_api::RawMutex;
use parking_lot::Mutex;

pub mod peer_connection;

#[cxx::bridge]
pub mod ffi {
    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasMediaType {
        MEDIA_TYPE_AUDIO,
        MEDIA_TYPE_VIDEO,
        MEDIA_TYPE_DATA,
        MEDIA_TYPE_UNSUPPORTED,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasSDPType {
        kOffer,
        kPrAnswer,
        kAnswer,
        kRollback,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasRTCSignalingState {
        kStable,
        kHaveLocalOffer,
        kHaveLocalPrAnswer,
        kHaveRemoteOffer,
        kHaveRemotePrAnswer,
        kClosed,
    }

    #[derive(Debug)]
    #[repr(u32)]
    // See https://w3c.github.io/webrtc-pc/#dom-rtcicegatheringstate
    enum ArcasIceGatheringState {
        kIceGatheringNew,
        kIceGatheringGathering,
        kIceGatheringComplete,
    }

    #[derive(Debug)]
    #[repr(u32)]
    // See https://w3c.github.io/webrtc-pc/#dom-rtcpeerconnectionstate
    enum ArcasPeerConnectionState {
        kNew,
        kConnecting,
        kConnected,
        kDisconnected,
        kFailed,
        kClosed,
    }

    #[derive(Debug)]
    #[repr(u32)]
    // See https://w3c.github.io/webrtc-pc/#dom-rtciceconnectionstate
    enum ArcasIceConnectionState {
        kIceConnectionNew,
        kIceConnectionChecking,
        kIceConnectionConnected,
        kIceConnectionCompleted,
        kIceConnectionFailed,
        kIceConnectionDisconnected,
        kIceConnectionClosed,
        kIceConnectionMax,
    }

    #[derive(Debug)]
    #[repr(u32)]
    // TLS certificate policy.
    enum ArcasTlsCertPolicy {
        // For TLS based protocols, ensure the connection is secure by not
        // circumventing certificate validation.
        kTlsCertPolicySecure,
        // For TLS based protocols, disregard security completely by skipping
        // certificate validation. This is insecure and should never be used unless
        // security is irrelevant in that particular context.
        kTlsCertPolicyInsecureNoCheck,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasSDPSemantics {
        kPlanB,
        kUnifiedPlan,
    }

    #[derive(Debug)]
    struct ArcasICECandidate {
        id: String,
        sdp_mid: String,
        sdp_mline_index: i32,
        sdp: String,
    }

    #[derive(Debug)]
    struct ArcasCandidatePairChangeEvent {
        selected_remote_id: String,
        selected_local_id: String,
        last_data_received_ms: i64,
        reason: String,
        estimated_disconnected_time_ms: i64,
    }

    #[derive(Debug, Clone)]
    struct ArcasICEServer {
        urls: Vec<String>,
        username: String,
        password: String,
    }

    #[derive(Debug)]
    struct ArcasPeerConnectionConfig {
        ice_servers: Vec<ArcasICEServer>,
        sdp_semantics: ArcasSDPSemantics,
    }

    unsafe extern "C++" {
        include!("libwebrtc-sys/include/alias.h");
        include!("libwebrtc-sys/include/rust_entry.h");

        // cricket
        #[namespace = "cricket"]
        #[cxx_name = "Candidate"]
        type CricketCandidate;
        #[namespace = "cricket"]
        #[cxx_name = "CandidatePair"]
        type CricketCandidatePair;

        #[namespace = "webrtc"]
        type PeerConnectionInterface;
        type ArcasSDPType;
        #[namespace = "webrtc"]
        type RTCError;
        #[cxx_name = "MediaType"]
        #[namespace = "cricket"]
        type ArcasMediaType;
        type ArcasRTCSignalingState;
        type ArcasIceConnectionState;
        type ArcasPeerConnectionState;
        type ArcasIceGatheringState;
        type ArcasTlsCertPolicy;
        #[namespace = "webrtc"]
        #[cxx_name = "SdpSemantics"]
        type ArcasSDPSemantics;

        // Our types
        type ArcasRTPTransceiver;
        type ArcasRTPReceiver;
        type ArcasMediaStream;
        type ArcasDataChannel;
        type ArcasPeerConnectionFactory;
        type ArcasSessionDescription;
        type ArcasPeerConnection;
        type ArcasRTCConfiguration;
        type ArcasCreateSessionDescriptionObserver;
        type ArcasSetDescriptionObserver;

        fn create_factory() -> UniquePtr<ArcasPeerConnectionFactory>;
        fn create_peer_connection(
            self: &ArcasPeerConnectionFactory,
            config: UniquePtr<ArcasRTCConfiguration>,
            observer: Box<ArcasRustPeerConnectionObserver>,
        ) -> UniquePtr<ArcasPeerConnection>;

        // peer connection
        fn create_offer(
            self: &ArcasPeerConnection,
            observer: Box<ArcasRustCreateSessionDescriptionObserver>,
        );
        fn create_answer(
            self: &ArcasPeerConnection,
            observer: Box<ArcasRustCreateSessionDescriptionObserver>,
        );
        fn set_local_description(
            self: &ArcasPeerConnection,
            observer: Box<ArcasRustSetSessionDescriptionObserver>,
            session: UniquePtr<ArcasSessionDescription>,
        );

        fn set_remote_description(
            self: &ArcasPeerConnection,
            observer: Box<ArcasRustSetSessionDescriptionObserver>,
            session: UniquePtr<ArcasSessionDescription>,
        );

        fn add_simple_media_transceiver(
            self: &ArcasPeerConnection,
            media_type: ArcasMediaType,
        ) -> UniquePtr<ArcasRTPTransceiver>;

        // session description
        fn to_string(self: &ArcasSessionDescription) -> String;
        fn get_type(self: &ArcasSessionDescription) -> ArcasSDPType;
        fn clone(self: &ArcasSessionDescription) -> UniquePtr<ArcasSessionDescription>;

        // utilities
        // fn create_session_description_observer(
        //     observer: Box<ArcasRustCreateSessionDescriptionObserver>,
        // ) -> SharedPtr<ArcasCreateSessionDescriptionObserver>;

        // fn set_session_description_observer(
        //     observer: Box<ArcasRustSetSessionDescriptionObserver>,
        // ) -> SharedPtr<ArcasSetDescriptionObserver>;

        fn create_rtc_configuration(
            config: ArcasPeerConnectionConfig,
        ) -> UniquePtr<ArcasRTCConfiguration>;

        // XXX: Hacks to ensure CXX generates the unique ptr bindings for these return types.
        fn nutbar() -> UniquePtr<ArcasDataChannel>;
        fn supbar() -> UniquePtr<ArcasMediaStream>;
    }

    extern "Rust" {
        type ArcasRustPeerConnectionObserver;
        type ArcasRustCreateSessionDescriptionObserver;
        type ArcasRustSetSessionDescriptionObserver;

        // ArcasRustCreateSessionDescriptionObserver
        fn on_success(
            self: &ArcasRustCreateSessionDescriptionObserver,
            success: UniquePtr<ArcasSessionDescription>,
        );
        fn on_failure(
            self: &ArcasRustCreateSessionDescriptionObserver,
            failure: UniquePtr<RTCError>,
        );
        // ArcasRustSetSessionDescriptionObserver
        fn on_success(self: &ArcasRustSetSessionDescriptionObserver);
        fn on_failure(self: &ArcasRustSetSessionDescriptionObserver, failure: UniquePtr<RTCError>);

        fn on_signaling_state_change(
            self: &ArcasRustPeerConnectionObserver,
            state: ArcasRTCSignalingState,
        );
        fn on_add_stream(
            self: &ArcasRustPeerConnectionObserver,
            stream: UniquePtr<ArcasMediaStream>,
        );
        fn on_remove_stream(
            self: &ArcasRustPeerConnectionObserver,
            stream: UniquePtr<ArcasMediaStream>,
        );
        fn on_datachannel(
            self: &ArcasRustPeerConnectionObserver,
            data_channel: UniquePtr<ArcasDataChannel>,
        );
        fn on_renegotiation_needed(self: &ArcasRustPeerConnectionObserver);
        fn on_renegotiation_needed_event(self: &ArcasRustPeerConnectionObserver, event: u32);
        fn on_ice_connection_change(
            self: &ArcasRustPeerConnectionObserver,
            state: ArcasIceConnectionState,
        );
        fn on_connection_change(
            self: &ArcasRustPeerConnectionObserver,
            state: ArcasPeerConnectionState,
        );
        fn on_ice_gathering_change(
            self: &ArcasRustPeerConnectionObserver,
            state: ArcasIceGatheringState,
        );
        fn on_ice_candidate(self: &ArcasRustPeerConnectionObserver, candidate: ArcasICECandidate);
        fn on_ice_candidate_error(
            self: &ArcasRustPeerConnectionObserver,
            host_candidate: String,
            url: String,
            error_code: i32,
            error_text: String,
        );

        fn on_ice_candidate_error_address_port(
            self: &ArcasRustPeerConnectionObserver,
            address: String,
            port: i32,
            url: String,
            error_code: i32,
            error_text: String,
        );

        fn on_ice_candidates_removed(self: &ArcasRustPeerConnectionObserver, removed: Vec<String>);

        fn on_ice_connection_receiving_change(
            self: &ArcasRustPeerConnectionObserver,
            receiving: bool,
        );

        fn on_ice_selected_candidate_pair_change(
            self: &ArcasRustPeerConnectionObserver,
            event: ArcasCandidatePairChangeEvent,
        );

        fn on_add_track(
            self: &ArcasRustPeerConnectionObserver,
            receiver: UniquePtr<ArcasRTPReceiver>,
            // TODO: Need a collection type that we can use here.
            // streams: UniquePtr<CxxVector<UniquePtr<ArcasMediaStream>>>,
        );

        fn on_track(
            self: &ArcasRustPeerConnectionObserver,
            transceiver: UniquePtr<ArcasRTPTransceiver>,
        );

        fn on_remove_track(
            self: &ArcasRustPeerConnectionObserver,
            receiver: UniquePtr<ArcasRTPReceiver>,
        );

        fn on_interesting_usage(self: &ArcasRustPeerConnectionObserver, pattern: i32);
    }
}

pub struct ArcasRustCreateSessionDescriptionObserver {
    success: Box<Fn(UniquePtr<crate::ffi::ArcasSessionDescription>) -> ()>,
    failure: Box<Fn(UniquePtr<crate::ffi::RTCError>) -> ()>,
}

impl ArcasRustCreateSessionDescriptionObserver {
    pub fn new(
        success: Box<Fn(UniquePtr<crate::ffi::ArcasSessionDescription>) -> ()>,
        failure: Box<Fn(UniquePtr<crate::ffi::RTCError>) -> ()>,
    ) -> Self {
        Self { success, failure }
    }

    fn on_success(&self, description: UniquePtr<crate::ffi::ArcasSessionDescription>) {
        (self.success)(description);
    }
    fn on_failure(&self, err: UniquePtr<crate::ffi::RTCError>) {
        (self.failure)(err);
    }
}

pub struct ArcasRustSetSessionDescriptionObserver {
    success: Box<Fn() -> ()>,
    failure: Box<Fn(UniquePtr<crate::ffi::RTCError>) -> ()>,
}

impl ArcasRustSetSessionDescriptionObserver {
    pub fn new(
        success: Box<Fn() -> ()>,
        failure: Box<Fn(UniquePtr<crate::ffi::RTCError>) -> ()>,
    ) -> Self {
        Self { success, failure }
    }

    fn on_success(&self) {
        (self.success)();
    }
    fn on_failure(&self, err: UniquePtr<crate::ffi::RTCError>) {
        (self.failure)(err);
    }
}

pub struct ArcasRustPeerConnectionObserver {}

impl ArcasRustPeerConnectionObserver {
    fn new() -> ArcasRustPeerConnectionObserver {
        ArcasRustPeerConnectionObserver {}
    }

    fn on_signaling_state_change(&self, state: crate::ffi::ArcasRTCSignalingState) {}

    fn on_add_stream(&self, stream: UniquePtr<crate::ffi::ArcasMediaStream>) {
        println!("got media stream in rust");
    }

    fn on_remove_stream(&self, stream: UniquePtr<crate::ffi::ArcasMediaStream>) {
        println!("got media stream in rust");
    }

    fn on_datachannel(&self, data_channel: UniquePtr<crate::ffi::ArcasDataChannel>) {
        println!("got data channel in rust");
    }

    fn on_renegotiation_needed(&self) {
        println!("got onrenegotiation needed in rust");
    }

    fn on_renegotiation_needed_event(&self, event: u32) {
        println!("got onrenegotiation needed in rust with event: {}", event);
    }

    fn on_ice_connection_change(&self, state: crate::ffi::ArcasIceConnectionState) {
        println!("got on ice connection change in rust : {:?}", state);
    }

    fn on_connection_change(&self, state: crate::ffi::ArcasPeerConnectionState) {
        println!("got on connection change in rust : {:?}", state);
    }

    fn on_ice_gathering_change(&self, state: crate::ffi::ArcasIceGatheringState) {
        println!("got on ice gathering state in rust : {:?}", state);
    }

    fn on_ice_candidate(&self, candidate: crate::ffi::ArcasICECandidate) {
        println!("got on ice candidate in rust : {:?}", candidate.sdp);
    }

    fn on_ice_candidate_error(
        &self,
        host_candidate: String,
        url: String,
        error_code: i32,
        error_text: String,
    ) {
        println!(
            "got ice canddiate errors {} {} code={} {}",
            host_candidate, url, error_code, error_text
        );
    }

    fn on_ice_candidate_error_address_port(
        &self,
        address: String,
        port: i32,
        url: String,
        error_code: i32,
        error_text: String,
    ) {
        println!(
            "got ice canddiate errors {} {} code={} {}",
            address, url, error_code, error_text,
        );
    }

    fn on_ice_candidates_removed(&self, removed: Vec<String>) {
        println!("got ice candidate removes: {:?}", removed);
    }

    fn on_ice_connection_receiving_change(&self, receiving: bool) {
        println!("rust receiving ice {}", receiving);
    }

    fn on_ice_selected_candidate_pair_change(
        self: &ArcasRustPeerConnectionObserver,
        event: crate::ffi::ArcasCandidatePairChangeEvent,
    ) {
        println!("on ice selected candidate pair change: {:?}", event.reason);
    }

    fn on_add_track(
        self: &ArcasRustPeerConnectionObserver,
        receiver: UniquePtr<crate::ffi::ArcasRTPReceiver>,
    ) {
        println!("on add track");
    }

    fn on_track(
        self: &ArcasRustPeerConnectionObserver,
        transceiver: UniquePtr<crate::ffi::ArcasRTPTransceiver>,
    ) {
        println!("on track");
    }

    fn on_remove_track(
        self: &ArcasRustPeerConnectionObserver,
        receiver: UniquePtr<crate::ffi::ArcasRTPReceiver>,
    ) {
        println!("on remove track");
    }

    fn on_interesting_usage(self: &ArcasRustPeerConnectionObserver, pattern: i32) {
        println!("on interesting usage {}", pattern);
    }
}
