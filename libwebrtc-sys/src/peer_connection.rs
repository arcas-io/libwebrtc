#[cfg(test)]
use crate::ffi::ArcasPeerConnectionObserver;
#[cfg(test)]
use cxx::SharedPtr;
#[cfg(test)]
use std::sync::mpsc::{channel, Receiver, Sender};

use cxx::UniquePtr;

use crate::ffi::{
    self, ArcasCandidatePairChangeEvent, ArcasDataChannel, ArcasICECandidate,
    ArcasIceConnectionState, ArcasIceGatheringState, ArcasMediaStream, ArcasPeerConnectionState,
    ArcasRTCSignalingState, ArcasRTPReceiver,
};

unsafe impl<'a> Sync for ffi::ArcasPeerConnection<'a> {}
unsafe impl<'a> Send for ffi::ArcasPeerConnection<'a> {}
unsafe impl<'a> Sync for ffi::ArcasPeerConnectionFactory<'a> {}
unsafe impl<'a> Send for ffi::ArcasPeerConnectionFactory<'a> {}

pub trait PeerConnectionObserverImpl {
    fn on_signaling_state_change(&self, _state: ArcasRTCSignalingState) {}
    fn on_add_stream(&self, _stream: UniquePtr<ArcasMediaStream>) {}
    fn on_remove_stream(&self, _stream: UniquePtr<ArcasMediaStream>) {}
    fn on_datachannel(&self, _data_channel: UniquePtr<ArcasDataChannel>) {}
    fn on_renegotiation_needed(&self) {}
    fn on_renegotiation_needed_event(&self, _event: u32) {}
    fn on_ice_connection_change(&self, _state: ArcasIceConnectionState) {}
    fn on_connection_change(&self, _state: ArcasPeerConnectionState) {}
    fn on_ice_gathering_change(&self, _state: ArcasIceGatheringState) {}
    fn on_ice_candidate(&self, _candidate: UniquePtr<ArcasICECandidate>) {}
    fn on_ice_candidate_error(
        &self,
        _host_candidate: String,
        _url: String,
        _error_code: i32,
        _error_text: String,
    ) {
    }

    fn on_ice_candidate_error_address_port(
        &self,
        _address: String,
        _port: i32,
        _url: String,
        _error_code: i32,
        _error_text: String,
    ) {
    }

    fn on_ice_candidates_removed(&self, _removed: Vec<String>) {}
    fn on_ice_connection_receiving_change(&self, _receiving: bool) {}
    fn on_ice_selected_candidate_pair_change(&self, _event: ArcasCandidatePairChangeEvent) {}

    fn on_add_track(&self, _receiver: UniquePtr<ArcasRTPReceiver>) {}
    fn on_video_track(&self, _transceiver: UniquePtr<ffi::ArcasRTPVideoTransceiver>) {}
    fn on_audio_track(&self, _transceiver: UniquePtr<ffi::ArcasRTPAudioTransceiver>) {}
    fn on_remove_track(&self, _receiver: UniquePtr<ArcasRTPReceiver>) {}
    fn on_interesting_usage(&self, _pattern: i32) {}
}

pub struct PeerConnectionObserverProxy {
    api: Box<dyn PeerConnectionObserverImpl>,
}

impl PeerConnectionObserverProxy {
    pub fn new(api: Box<dyn PeerConnectionObserverImpl>) -> PeerConnectionObserverProxy {
        PeerConnectionObserverProxy { api }
    }

    pub fn on_signaling_state_change(&self, state: ArcasRTCSignalingState) {
        self.api.on_signaling_state_change(state)
    }

    pub fn on_add_stream(&self, stream: UniquePtr<ArcasMediaStream>) {
        self.api.on_add_stream(stream);
    }

    pub fn on_remove_stream(&self, stream: UniquePtr<ArcasMediaStream>) {
        self.api.on_remove_stream(stream);
    }

    pub fn on_datachannel(&self, data_channel: UniquePtr<ArcasDataChannel>) {
        self.api.on_datachannel(data_channel);
    }

    pub fn on_renegotiation_needed(&self) {
        self.api.on_renegotiation_needed();
    }

    pub fn on_renegotiation_needed_event(&self, event: u32) {
        self.api.on_renegotiation_needed_event(event);
    }

    pub fn on_ice_connection_change(&self, state: ArcasIceConnectionState) {
        self.api.on_ice_connection_change(state);
    }

    pub fn on_connection_change(&self, state: ArcasPeerConnectionState) {
        self.api.on_connection_change(state);
    }

    pub fn on_ice_gathering_change(&self, state: ArcasIceGatheringState) {
        self.api.on_ice_gathering_change(state);
    }

    pub fn on_ice_candidate(&self, candidate: UniquePtr<ArcasICECandidate>) {
        self.api.on_ice_candidate(candidate);
    }

    pub fn on_ice_candidate_error(
        &self,
        host_candidate: String,
        url: String,
        error_code: i32,
        error_text: String,
    ) {
        self.api
            .on_ice_candidate_error(host_candidate, url, error_code, error_text);
    }

    pub fn on_ice_candidate_error_address_port(
        &self,
        address: String,
        port: i32,
        url: String,
        error_code: i32,
        error_text: String,
    ) {
        self.api
            .on_ice_candidate_error_address_port(address, port, url, error_code, error_text);
    }

    pub fn on_ice_candidates_removed(&self, removed: Vec<String>) {
        self.api.on_ice_candidates_removed(removed);
    }

    pub fn on_ice_connection_receiving_change(&self, receiving: bool) {
        self.api.on_ice_connection_receiving_change(receiving)
    }

    pub fn on_ice_selected_candidate_pair_change(
        self: &PeerConnectionObserverProxy,
        event: ArcasCandidatePairChangeEvent,
    ) {
        self.api.on_ice_selected_candidate_pair_change(event);
    }

    pub fn on_add_track(&self, receiver: UniquePtr<ArcasRTPReceiver>) {
        self.api.on_add_track(receiver);
    }

    pub fn on_audio_track(&self, transceiver: UniquePtr<ffi::ArcasRTPAudioTransceiver>) {
        self.api.on_audio_track(transceiver);
    }

    pub fn on_video_track(&self, transceiver: UniquePtr<ffi::ArcasRTPVideoTransceiver>) {
        self.api.on_video_track(transceiver);
    }

    pub fn on_remove_track(&self, receiver: UniquePtr<ArcasRTPReceiver>) {
        self.api.on_remove_track(receiver);
    }

    pub fn on_interesting_usage(&self, pattern: i32) {
        self.api.on_interesting_usage(pattern);
    }
}

#[cfg(test)]
pub struct DummyPeerConnectionObserver {}

#[cfg(test)]
pub struct TestIcePeerConnectionObserver {
    ice_tx: Sender<UniquePtr<ArcasICECandidate>>,
}

#[cfg(test)]
impl TestIcePeerConnectionObserver {
    pub fn new(ice_tx: Sender<UniquePtr<ArcasICECandidate>>) -> TestIcePeerConnectionObserver {
        TestIcePeerConnectionObserver { ice_tx }
    }
}

#[cfg(test)]
pub fn create_test_ice_peer_connection_observer() -> (
    Receiver<UniquePtr<ArcasICECandidate>>,
    UniquePtr<ArcasPeerConnectionObserver>,
) {
    let (tx, rx) = channel();
    let out = crate::ffi::create_peer_connection_observer(Box::new(
        PeerConnectionObserverProxy::new(Box::new(TestIcePeerConnectionObserver { ice_tx: tx })),
    ));
    (rx, out)
}

#[cfg(test)]
impl PeerConnectionObserverImpl for TestIcePeerConnectionObserver {
    fn on_ice_candidate(&self, candidate: UniquePtr<ArcasICECandidate>) {
        self.ice_tx.send(candidate).unwrap();
    }
}

#[cfg(test)]
impl PeerConnectionObserverImpl for DummyPeerConnectionObserver {}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use cxx::SharedPtr;

    use super::*;
    use crate::{
        ffi::{self},
        ArcasRustCreateSessionDescriptionObserver, ArcasRustRTCStatsCollectorCallback,
        ArcasRustSetSessionDescriptionObserver, PeerConnectionObserverProxy,
    };

    fn create_test_observer() -> UniquePtr<ffi::ArcasPeerConnectionObserver> {
        ffi::create_peer_connection_observer(Box::new(PeerConnectionObserverProxy::new(Box::new(
            DummyPeerConnectionObserver {},
        ))))
    }

    #[test]
    fn test_basic_peer_connection_init() {
        let ice = ffi::ArcasICEServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            username: "".to_owned(),
            password: "".to_owned(),
        };
        let config = ffi::create_rtc_configuration(ffi::ArcasPeerConnectionConfig {
            ice_servers: vec![ice.clone()],
            sdp_semantics: ffi::ArcasSDPSemantics::kUnifiedPlan,
        });
        let config2 = ffi::create_rtc_configuration(ffi::ArcasPeerConnectionConfig {
            ice_servers: vec![ice],
            sdp_semantics: ffi::ArcasSDPSemantics::kUnifiedPlan,
        });

        let api = ffi::create_arcas_api();

        let factory = api.create_factory();
        let mut observer = create_test_observer();
        let pc = unsafe {
            factory.create_peer_connection(config, observer.pin_mut().get_unchecked_mut())
        };
        let _transceiver = pc.add_video_transceiver();
        let (tx, rx) = mpsc::channel();

        pc.create_offer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
            Box::new(move |session_description| {
                tx.send(session_description)
                    .expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        )));

        let sdp = rx.recv().expect("Can get offer");
        assert!(!sdp.to_string().is_empty(), "has sdp string");

        let (set_tx, set_rx) = mpsc::channel();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_tx.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        let cc_observer = Box::new(observer);
        pc.set_local_description(cc_observer, sdp.clone());
        set_rx.recv().expect("Can set description");

        let mut observer2 = create_test_observer();
        let pc2 = unsafe {
            factory.create_peer_connection(config2, observer2.pin_mut().get_unchecked_mut())
        };
        let (set_remote_tx, set_remote_rx) = mpsc::channel();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_remote_tx.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        pc2.set_remote_description(Box::new(observer), sdp.clone());
        set_remote_rx.recv().expect("Can set description");
        let (tx_answer, rx_answer) = mpsc::channel();
        pc2.create_answer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
            Box::new(move |session_description| {
                assert_eq!(session_description.get_type(), ffi::ArcasSDPType::kAnswer);
                println!("got sdp: {}", session_description.to_string(),);
                tx_answer.send(session_description).expect("Can send");
            }),
            Box::new(move |_err| {
                println!("got some kind of error");
                panic!("Failed to create session description");
            }),
        )));
        let answer = rx_answer.recv().expect("Creates answer");
        let answer_for_remote = answer.clone();

        let (set_local_tx2, set_local_rx2) = mpsc::channel();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_local_tx2.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        pc2.set_local_description(Box::new(observer), answer);
        set_local_rx2.recv().expect("Can finish connection loop");

        let (set_remote_tx2, set_remote_rx2) = mpsc::channel();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_remote_tx2.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        pc.set_remote_description(Box::new(observer), answer_for_remote);
        set_remote_rx2.recv().expect("Can finish connection loop");
    }

    #[test]
    fn test_basic_get_stats() {
        let ice = ffi::ArcasICEServer {
            urls: vec!["stun:stun.l.google.com:19302".to_owned()],
            username: "".to_owned(),
            password: "".to_owned(),
        };
        let config = ffi::create_rtc_configuration(ffi::ArcasPeerConnectionConfig {
            ice_servers: vec![ice.clone()],
            sdp_semantics: ffi::ArcasSDPSemantics::kUnifiedPlan,
        });
        let config2 = ffi::create_rtc_configuration(ffi::ArcasPeerConnectionConfig {
            ice_servers: vec![ice],
            sdp_semantics: ffi::ArcasSDPSemantics::kUnifiedPlan,
        });

        let api = ffi::create_arcas_api();
        let mut source = crate::ffi::create_arcas_video_track_source();
        let mut factory1 = api.create_factory();
        let mut observer = create_test_observer();
        let pc = unsafe {
            factory1.create_peer_connection(config, observer.pin_mut().get_unchecked_mut())
        };
        let track = unsafe {
            factory1
                .as_mut()
                .unwrap()
                .create_video_track("test".into(), source.pin_mut())
        };
        pc.add_video_track(track, ["test".into()].to_vec());

        // ensure we don't crash easily...
        // for _i in 0..100 {
        //     let zeroed = &mut [1u8, 2, 3];
        //     unsafe {
        //         source.push_i420_data(100, 100, 0, 0, 0, zeroed.as_ptr());
        //     }
        // }

        let (tx, rx) = mpsc::channel();
        pc.create_offer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
            Box::new(move |session_description| {
                tx.send(session_description)
                    .expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        )));

        let sdp = rx.recv().expect("Can get offer");
        assert!(!sdp.to_string().is_empty(), "has sdp string");

        let (set_tx, set_rx) = mpsc::channel();
        let set_session_observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_tx.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        let cc_observer = Box::new(set_session_observer);
        pc.set_local_description(cc_observer, sdp.clone());
        set_rx.recv().expect("Can set description");

        let factory2 = api.create_factory();
        let mut observer2 = create_test_observer();
        let pc2 = unsafe {
            factory2.create_peer_connection(config2, observer2.pin_mut().get_unchecked_mut())
        };
        let (set_remote_tx, set_remote_rx) = mpsc::channel();
        let set_session_observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_remote_tx.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        pc2.set_remote_description(Box::new(set_session_observer), sdp.clone());
        set_remote_rx.recv().expect("Can set description");
        let (tx_answer, rx_answer) = mpsc::channel();
        pc2.create_answer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
            Box::new(move |session_description| {
                assert_eq!(session_description.get_type(), ffi::ArcasSDPType::kAnswer);
                println!("got sdp: {}", session_description.to_string(),);
                tx_answer.send(session_description).expect("Can send");
            }),
            Box::new(move |_err| {
                println!("got some kind of error");
                panic!("Failed to create session description");
            }),
        )));
        let answer = rx_answer.recv().expect("Creates answer");
        let answer_for_remote = answer.clone();

        let (set_local_tx2, set_local_rx2) = mpsc::channel();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_local_tx2.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        pc2.set_local_description(Box::new(observer), answer);
        set_local_rx2.recv().expect("Can finish connection loop");

        let (set_remote_tx2, set_remote_rx2) = mpsc::channel();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_remote_tx2.send(1).expect("Cyn send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        pc.set_remote_description(Box::new(observer), answer_for_remote);
        set_remote_rx2.recv().expect("Can finish connection loop");

        let (tx, rx) = mpsc::channel::<()>();
        let stats_cb =
            ArcasRustRTCStatsCollectorCallback::new(Box::new(move |v_rx, a_rx, v_tx, a_tx| {
                // println!("{:?} {:?} {:?} {:?}", v_rx, a_rx, v_tx, a_tx);
                assert!(v_rx.is_empty());
                assert!(a_rx.is_empty());
                assert!(v_tx.len() == 1);
                assert!(a_tx.is_empty());
                tx.send(()).expect("get_stats send failed")
            }));
        pc.get_stats(Box::new(stats_cb));
        rx.recv().expect("awaiting message");
    }
}
