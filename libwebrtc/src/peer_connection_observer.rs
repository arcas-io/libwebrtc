use cxx::UniquePtr;
use libwebrtc_sys::{
    ffi::{
        create_peer_connection_observer, ArcasCandidatePairChangeEvent, ArcasDataChannel,
        ArcasICECandidate, ArcasMediaStream, ArcasPeerConnectionObserver, ArcasPeerConnectionState,
        ArcasRTCSignalingState, ArcasRTPVideoTransceiver,
    },
    peer_connection::PeerConnectionObserverImpl,
    PeerConnectionObserverProxy,
};
use tokio::sync::{
    broadcast,
    mpsc::{channel, Receiver, Sender},
};

use crate::{
    cxx_get_mut, cxx_ref,
    error::{Result, WebRTCError},
    ice_candidate::ICECandidate,
    ok_or_return, take_or_err,
    transceiver::VideoTransceiver,
};

const CONNECTION_STATE_BUFFERING: usize = 100;
const ICE_CANDIDATE_BUFFERING: usize = 50;
const VIDEO_TRACK_BUFFERING: usize = 50;

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
struct PeerConnectionObserverHandler {
    connection_state_tx: Sender<ConnectionState>,
    ice_candidate_tx: Sender<ICECandidate>,
    video_track_tx: Sender<VideoTransceiver>,
}

impl PeerConnectionObserverImpl for PeerConnectionObserverHandler {
    fn on_signaling_state_change(&self, _state: ArcasRTCSignalingState) {}

    fn on_add_stream(&self, _stream: UniquePtr<ArcasMediaStream>) {}

    fn on_remove_stream(&self, _stream: UniquePtr<ArcasMediaStream>) {}

    fn on_datachannel(&self, _data_channel: UniquePtr<ArcasDataChannel>) {}

    fn on_renegotiation_needed(&self) {}

    fn on_renegotiation_needed_event(&self, _event: u32) {}

    fn on_ice_connection_change(&self, _state: libwebrtc_sys::ffi::ArcasIceConnectionState) {}

    fn on_connection_change(&self, state: libwebrtc_sys::ffi::ArcasPeerConnectionState) {
        ok_or_return!(self.connection_state_tx.blocking_send(state.into()));
    }

    fn on_ice_gathering_change(&self, _state: libwebrtc_sys::ffi::ArcasIceGatheringState) {}

    fn on_ice_candidate(&self, candidate: UniquePtr<ArcasICECandidate>) {
        ok_or_return!(self
            .ice_candidate_tx
            .blocking_send(ICECandidate::new(candidate)));
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

    fn on_add_track(&self, _receiver: UniquePtr<libwebrtc_sys::ffi::ArcasRTPReceiver>) {}

    fn on_video_track(&self, transceiver: UniquePtr<ArcasRTPVideoTransceiver>) {
        ok_or_return!(self
            .video_track_tx
            .blocking_send(VideoTransceiver::new(transceiver)));
    }

    fn on_audio_track(
        &self,
        _transceiver: UniquePtr<libwebrtc_sys::ffi::ArcasRTPAudioTransceiver>,
    ) {
    }

    fn on_remove_track(&self, _receiver: UniquePtr<libwebrtc_sys::ffi::ArcasRTPReceiver>) {}

    fn on_interesting_usage(&self, _pattern: i32) {}
}

/// PeerConnectionObserver abstraction handles the internals of receiving events on libwebrtc
/// threads and forwarding them to an appropriate tokio channel.
pub struct PeerConnectionObserver {
    cxx_observer: UniquePtr<ArcasPeerConnectionObserver>,
    connection_state_rx: Option<Receiver<ConnectionState>>,
    ice_candidate_rx: Option<Receiver<ICECandidate>>,
    video_track_rx: Option<Receiver<VideoTransceiver>>,
}

impl PeerConnectionObserver {
    pub fn new() -> Result<Self> {
        let (connection_state_tx, connection_state_rx) = channel(CONNECTION_STATE_BUFFERING);
        let (ice_candidate_tx, ice_candidate_rx) = channel(ICE_CANDIDATE_BUFFERING);
        let (video_track_tx, video_track_rx) = channel(VIDEO_TRACK_BUFFERING);

        let handler = Box::new(PeerConnectionObserverHandler {
            connection_state_tx,
            ice_candidate_tx,
            video_track_tx,
        });

        let cxx_observer =
            create_peer_connection_observer(Box::new(PeerConnectionObserverProxy::new(handler)));

        Ok(Self {
            cxx_observer,
            connection_state_rx: Some(connection_state_rx),
            ice_candidate_rx: Some(ice_candidate_rx),
            video_track_rx: Some(video_track_rx),
        })
    }

    pub(crate) fn cxx_mut_ptr(&mut self) -> Result<*mut ArcasPeerConnectionObserver> {
        unsafe { cxx_get_mut!(self.cxx_observer) }
    }

    pub fn take_connection_state_rx(&mut self) -> Result<Receiver<ConnectionState>> {
        take_or_err!(self.connection_state_rx)
    }

    pub fn take_ice_candidate_rx(&mut self) -> Result<Receiver<ICECandidate>> {
        take_or_err!(self.ice_candidate_rx)
    }

    pub fn take_video_track_rx(&mut self) -> Result<Receiver<VideoTransceiver>> {
        take_or_err!(self.video_track_rx)
    }
}
