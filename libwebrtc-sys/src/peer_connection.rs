#[cfg(test)]
use crate::ffi::ArcasPeerConnectionObserver;
#[cfg(test)]
use std::sync::mpsc::{channel, Receiver, Sender};

use cxx::UniquePtr;

use crate::ffi::{
    self, ArcasCandidatePairChangeEvent, ArcasDataChannel, ArcasICECandidate,
    ArcasIceConnectionState, ArcasIceGatheringState, ArcasMediaStream, ArcasPeerConnectionState,
    ArcasRTCSignalingState, ArcasRTPReceiver,
};

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
