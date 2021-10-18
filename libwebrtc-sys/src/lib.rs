use std::fmt;
use std::sync::Arc;

use cxx::CxxString;
use cxx::CxxVector;
use cxx::UniquePtr;
use parking_lot::lock_api::RawMutex;
use parking_lot::Mutex;

#[cxx::bridge]
pub mod ffi {
    #[derive(Debug)]
    // TODO: This works only with u32 but may need changes on other architectures?
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
    enum ArcasSDPSemantics {
        kPlanB,
        kUnifiedPlan,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasIceGatheringState {
        kIceGatheringNew,
        kIceGatheringGathering,
        kIceGatheringComplete,
    }

    #[derive(Debug)]
    #[repr(u32)]
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
    enum ArcasTlsCertPolicy {
        kTlsCertPolicySecure,
        kTlsCertPolicyInsecureNoCheck,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum ArcasICETransportType {
        kNone,
        kRelay,
        kNoHost,
        kAll,
    }

    #[derive(Debug)]
    struct ArcasRTCICEServer {
        // url is deprecated so we don't support it here...
        urls: Vec<String>,
        username: String,
        password: String,
    }

    #[derive(Debug)]
    struct ArcasRTCPeerConnectionConfig {
        ice_servers: Vec<ArcasRTCICEServer>,
        sdp_semantics: ArcasSDPSemantics,
    }

    extern "Rust" {
        type ArcasRustPeerConnectionObserver;
        type ArcasRustCreateSessionDescriptionObserver;

        fn on_signaling_state_change(
            self: &ArcasRustPeerConnectionObserver,
            state: Box<ArcasRTCSignalingState>,
        );

        fn call_session_observer_success(
            observer: Box<ArcasRustCreateSessionDescriptionObserver>,
            sdp: &SessionDescriptionInterface,
        );

        fn call_session_observer_failure(
            observer: Box<ArcasRustCreateSessionDescriptionObserver>,
            error: &RTCError,
        );
    }

    unsafe extern "C++" {
        include!("libwebrtc-sys/include/alias.h");
        include!("libwebrtc-sys/include/rust_entry.h");

        // Shared enums exported by alias.h
        type ArcasRTCSignalingState;
        type ArcasIceConnectionState;
        type ArcasICETransportType;
        type ArcasPeerConnectionState;
        type ArcasSDPSemantics;
        type ArcasIceGatheringState;
        type ArcasTlsCertPolicy;
        #[namespace = "webrtc"]
        type RTCError;
        #[namespace = "webrtc"]
        type SessionDescriptionInterface;
        type ArcasCreateSessionDescriptionObserver;

        type ArcasPeerConnectionFactory;
        type ArcasPeerConnection;

        fn create_factory() -> UniquePtr<ArcasPeerConnectionFactory>;
        fn create_peer_connection(
            self: &ArcasPeerConnectionFactory,
            cfg: ArcasRTCPeerConnectionConfig,
            observer: Box<ArcasRustPeerConnectionObserver>,
        ) -> UniquePtr<ArcasPeerConnection>;

        // peer connection methods
        fn create_offer(
            self: &ArcasPeerConnection,
            observer: Box<ArcasRustCreateSessionDescriptionObserver>,
        );

        fn session_description_to_string(sdp: &SessionDescriptionInterface) -> String;

    }
}

pub fn call_session_observer_success(
    observer: Box<ArcasRustCreateSessionDescriptionObserver>,
    sdp: &ffi::SessionDescriptionInterface,
) {
    (observer.success)(sdp);
}

pub fn call_session_observer_failure(
    observer: Box<ArcasRustCreateSessionDescriptionObserver>,
    error: &ffi::RTCError,
) {
    (observer.failure)(error);
}

pub struct ArcasRustCreateSessionDescriptionObserver {
    pub success: Box<dyn Fn(&ffi::SessionDescriptionInterface) -> ()>,
    pub failure: Box<dyn Fn(&ffi::RTCError) -> ()>,
}

impl ArcasRustCreateSessionDescriptionObserver {
    pub fn new(
        success: Box<dyn Fn(&ffi::SessionDescriptionInterface) -> ()>,
        failure: Box<dyn Fn(&ffi::RTCError) -> ()>,
    ) -> Self {
        Self { success, failure }
    }
}

pub struct ArcasRustPeerConnectionObserver {
    signaling_state_cb: Option<Arc<Mutex<Box<dyn Fn(Box<ffi::ArcasRTCSignalingState>)>>>>,
}

impl ArcasRustPeerConnectionObserver {
    fn new() -> ArcasRustPeerConnectionObserver {
        ArcasRustPeerConnectionObserver {
            signaling_state_cb: None,
        }
    }

    fn set_signaling_state_cb(&mut self, cb: Box<dyn Fn(Box<ffi::ArcasRTCSignalingState>) -> ()>) {
        self.signaling_state_cb = Some(Arc::new(Mutex::new(cb)));
    }

    fn on_signaling_state_change(&self, state: Box<ffi::ArcasRTCSignalingState>) {
        match &self.signaling_state_cb {
            Some(mutex) => {
                let cb = mutex.lock();
                (cb)(state);
            }
            None => {
                // log no callback?
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::mpsc, thread::sleep_ms};

    use super::*;

    #[test]
    fn test_ffi() {
        let ice = ffi::ArcasRTCICEServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            username: "".to_owned(),
            password: "".to_owned(),
        };
        let config = ffi::ArcasRTCPeerConnectionConfig {
            ice_servers: vec![ice],
            sdp_semantics: ffi::ArcasSDPSemantics::kUnifiedPlan,
        };
        let factory = ffi::create_factory();
        let mut observer = ArcasRustPeerConnectionObserver::new();
        observer.set_signaling_state_cb(Box::new(|state| {
            println!("got state {:?}", state);
        }));
        let pc = factory.create_peer_connection(config, Box::new(observer));
        // let (tx, rx) = mpsc::channel();
        pc.create_offer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
            Box::new(|session_description| {
                let sdp = ffi::session_description_to_string(session_description);
                println!("got session description: {} \n", sdp);
            }),
            Box::new(|err| {
                println!("got some kind of error");
            }),
        )));

        // rx.recv();
        sleep_ms(1000);
    }
}
