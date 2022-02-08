use cxx::UniquePtr;
use libwebrtc_sys::{
    data_channel::ffi::ArcasDataChannel,
    ffi::{
        create_peer_connection_observer, ArcasCandidatePairChangeEvent, ArcasICECandidate,
        ArcasIceConnectionState, ArcasIceGatheringState, ArcasMediaStream,
        ArcasPeerConnectionObserver, ArcasPeerConnectionState, ArcasRTCSignalingState,
        ArcasRTPAudioTransceiver, ArcasRTPReceiver, ArcasRTPVideoTransceiver,
    },
    peer_connection_observer::{PeerConnectionObserverImpl, PeerConnectionObserverProxy},
};
use tokio::sync::mpsc::Sender;

use crate::{
    cxx_get_mut, data_channel::DataChannel, error::Result, ice_candidate::ICECandidate, send_event,
    transceiver::VideoTransceiver,
};

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionState {
    New,
    Connecting,
    Connected,
    Disconnected,
    Failed,
    Closed,
}

impl From<ArcasPeerConnectionState> for ConnectionState {
    fn from(value: ArcasPeerConnectionState) -> Self {
        match value {
            ArcasPeerConnectionState::kNew => ConnectionState::New,
            ArcasPeerConnectionState::kConnecting => ConnectionState::Connecting,
            ArcasPeerConnectionState::kConnected => ConnectionState::Connected,
            ArcasPeerConnectionState::kDisconnected => ConnectionState::Disconnected,
            ArcasPeerConnectionState::kFailed => ConnectionState::Failed,
            ArcasPeerConnectionState::kClosed => ConnectionState::Closed,
            _ => ConnectionState::New,
        }
    }
}

/// Internal handler for the PeerConnectionObserver interface in C++.
/// These methods are always called from a webrtc thread (signaling probably) and
/// never are called directly from rust code.
#[derive(Default)]
pub struct ObserverSenders {
    pub connection_state: Option<Sender<ConnectionState>>,
    pub ice_candidate: Option<Sender<ICECandidate>>,
    pub data_channel: Option<Sender<DataChannel>>,
    pub video_track: Option<Sender<VideoTransceiver>>,
}

impl PeerConnectionObserverImpl for ObserverSenders {
    fn on_signaling_state_change(&self, _state: ArcasRTCSignalingState) {}

    fn on_add_stream(&self, _stream: UniquePtr<ArcasMediaStream>) {}

    fn on_remove_stream(&self, _stream: UniquePtr<ArcasMediaStream>) {}

    fn on_datachannel(&self, data_channel: UniquePtr<ArcasDataChannel>) {
        send_event!(self.data_channel, DataChannel::new(data_channel));
    }

    fn on_renegotiation_needed(&self) {}

    fn on_renegotiation_needed_event(&self, _event: u32) {}

    fn on_ice_connection_change(&self, _state: ArcasIceConnectionState) {}

    fn on_connection_change(&self, state: ArcasPeerConnectionState) {
        send_event!(self.connection_state, ConnectionState::from(state));
    }

    fn on_ice_gathering_change(&self, _state: ArcasIceGatheringState) {}

    fn on_ice_candidate(&self, candidate: UniquePtr<ArcasICECandidate>) {
        send_event!(self.ice_candidate, ICECandidate::new(candidate));
    }

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

    fn on_video_track(&self, transceiver: UniquePtr<ArcasRTPVideoTransceiver>) {
        send_event!(self.video_track, VideoTransceiver::new(transceiver));
    }

    fn on_audio_track(&self, _transceiver: UniquePtr<ArcasRTPAudioTransceiver>) {}

    fn on_remove_track(&self, _receiver: UniquePtr<ArcasRTPReceiver>) {}

    fn on_interesting_usage(&self, _pattern: i32) {}
}

/// PeerConnectionObserver abstraction handles the internals of receiving events on libwebrtc
/// threads and forwarding them to an appropriate tokio channel.
pub struct PeerConnectionObserver {
    cxx_observer: UniquePtr<ArcasPeerConnectionObserver>,
}

impl PeerConnectionObserver {
    pub fn new(handler: ObserverSenders) -> Result<Self> {
        let cxx_observer = create_peer_connection_observer(Box::new(
            PeerConnectionObserverProxy::new(Box::new(handler)),
        ));

        Ok(Self { cxx_observer })
    }

    pub(crate) fn cxx_mut_ptr(&mut self) -> Result<*mut ArcasPeerConnectionObserver> {
        unsafe { cxx_get_mut!(self.cxx_observer) }
    }
}
