use cxx::UniquePtr;

use self::ffi::{ArcasRTCError, ArcasSessionDescription};

#[cxx::bridge]
pub mod ffi {

    struct ArcasTransceiverInit {
        stream_ids: Vec<String>,
        direction: ArcasCxxRtpTransceiverDirection,
    }

    // stats types
    #[derive(Debug)]
    struct ArcasVideoReceiverStats {
        pub ssrc: u32,
        pub packets_received: u32,
        pub packets_lost: i32,
        pub packets_repaired: u32,
        pub bytes_received: u64,
        pub frames_decoded: u32,
        pub keyframes_decoded: u32,
        pub frames_dropped: u32,
        pub total_decode_time: f64,
        pub frame_width: u32,
        pub frame_height: u32,
    }

    #[derive(Debug)]
    struct ArcasVideoSenderStats {
        pub ssrc: u32,
        pub packets_sent: u32,
        pub bytes_sent: u64,
        pub frames_encoded: u32,
        pub key_frames_encoded: u32,
        pub total_encode_time: f64,
        pub frame_width: u32,
        pub frame_height: u32,
        pub retransmitted_packets_sent: u64,
        pub retransmitted_bytes_sent: u64,
        pub total_packet_send_delay: f64,
        pub nack_count: u32,
        pub fir_count: u32,
        pub pli_count: u32,
        pub quality_limitation_reason: u32, // 0 - kNone, 1 - kCpu, 2 - kBandwidth, 3 - kOther
        pub quality_limitation_resolution_changes: u32,
        pub remote_packets_lost: i32,
        pub remote_jitter: f64,
        pub remote_round_trip_time: f64,
    }

    #[derive(Debug)]
    struct ArcasAudioReceiverStats {
        pub ssrc: u32,
        pub packets_received: u32,
        pub packets_lost: i32,
        pub bytes_received: u64,
        pub total_samples_received: u64,
        pub total_samples_duration: f64,
        pub audio_level: f64,
        pub total_audio_energy: f64,
    }

    #[derive(Debug)]
    struct ArcasAudioSenderStats {
        pub ssrc: u32,
        pub packets_sent: u32,
        pub bytes_sent: u64,
        pub remote_packets_lost: i32,
        pub remote_jitter: f64,
        pub remote_round_trip_time: f64,
        pub audio_level: f64,
        pub total_audio_energy: f64,
    }

    unsafe extern "C++" {
        include!("include/alias.h");
        include!("include/peer_connection_stats_callback.h");
        include!("include/peer_connection.h");
        include!("libwebrtc-sys/src/video_track.rs.h");
        type ArcasPeerConnection;
        type ArcasSDPSemantics = crate::shared_bridge::ffi::ArcasSDPSemantics;
        type ArcasRTCConfiguration = crate::shared_bridge::ffi::ArcasRTCConfiguration;
        type ArcasSessionDescription = crate::session_description::ffi::ArcasSessionDescription;
        type ArcasRTPVideoTransceiver = crate::rtp_transceiver::ffi::ArcasRTPVideoTransceiver;
        type ArcasRTPAudioTransceiver = crate::rtp_transceiver::ffi::ArcasRTPAudioTransceiver;
        type ArcasRTPTransceiver = crate::rtp_transceiver::ffi::ArcasRTPTransceiver;
        type ArcasVideoTrack = crate::video_track::ffi::ArcasVideoTrack;
        type ArcasICECandidate = crate::ice_candidate::ffi::ArcasICECandidate;
        type ArcasRTCError = crate::error::ffi::ArcasRTCError;
        type ArcasCxxRtpTransceiverDirection =
            crate::shared_bridge::ffi::ArcasCxxRtpTransceiverDirection;
        type ArcasRTCStatsCollectorCallback;
        type ArcasAudioTrack = crate::audio_track::ffi::ArcasAudioTrack;

        fn gen_shared_peer_connection() -> SharedPtr<ArcasPeerConnection>;

        // ArcasPeerConnection
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

        fn add_video_transceiver(self: &ArcasPeerConnection)
            -> UniquePtr<ArcasRTPVideoTransceiver>;

        fn add_video_transceiver_with_track(
            self: &ArcasPeerConnection,
            track: UniquePtr<ArcasVideoTrack>,
            init: ArcasTransceiverInit,
        ) -> UniquePtr<ArcasRTPVideoTransceiver>;

        fn add_audio_transceiver_with_track(
            self: &ArcasPeerConnection,
            track: UniquePtr<ArcasAudioTrack>,
            init: ArcasTransceiverInit,
        ) -> UniquePtr<ArcasRTPAudioTransceiver>;

        fn add_audio_transceiver(self: &ArcasPeerConnection)
            -> UniquePtr<ArcasRTPAudioTransceiver>;

        fn add_video_track(
            self: &ArcasPeerConnection,
            track: UniquePtr<ArcasVideoTrack>,
            stream_ids: Vec<String>,
        );

        fn add_audio_track(
            self: &ArcasPeerConnection,
            track: UniquePtr<ArcasAudioTrack>,
            stream_ids: Vec<String>,
        );

        fn get_stats(self: &ArcasPeerConnection, callback: Box<ArcasRustRTCStatsCollectorCallback>);
        fn add_ice_candidate(self: &ArcasPeerConnection, candidate: UniquePtr<ArcasICECandidate>);
        fn close(self: &ArcasPeerConnection);
        fn get_transceivers(
            self: &ArcasPeerConnection,
        ) -> UniquePtr<CxxVector<ArcasRTPTransceiver>>;

    }

    extern "Rust" {
        type ArcasRustCreateSessionDescriptionObserver;
        type ArcasRustSetSessionDescriptionObserver;
        type ArcasRustRTCStatsCollectorCallback;
        fn on_stats_delivered(
            self: &ArcasRustRTCStatsCollectorCallback,
            video_rx: Vec<ArcasVideoReceiverStats>,
            audio_rx: Vec<ArcasAudioReceiverStats>,
            video_tx: Vec<ArcasVideoSenderStats>,
            audio_tx: Vec<ArcasAudioSenderStats>,
        );

        // ArcasRustCreateSessionDescriptionObserver
        fn on_success(
            self: &ArcasRustCreateSessionDescriptionObserver,
            success: UniquePtr<ArcasSessionDescription>,
        );
        fn on_failure(
            self: &ArcasRustCreateSessionDescriptionObserver,
            failure: UniquePtr<ArcasRTCError>,
        );
        // ArcasRustSetSessionDescriptionObserver
        fn on_success(self: &ArcasRustSetSessionDescriptionObserver);
        fn on_failure(
            self: &ArcasRustSetSessionDescriptionObserver,
            failure: UniquePtr<ArcasRTCError>,
        );
    }
}

pub struct ArcasRustCreateSessionDescriptionObserver {
    success: Box<dyn Fn(UniquePtr<ArcasSessionDescription>)>,
    failure: Box<dyn Fn(UniquePtr<ArcasRTCError>)>,
}

impl ArcasRustCreateSessionDescriptionObserver {
    pub fn new(
        success: Box<dyn Fn(UniquePtr<ArcasSessionDescription>)>,
        failure: Box<dyn Fn(UniquePtr<ArcasRTCError>)>,
    ) -> Self {
        Self { success, failure }
    }

    fn on_success(&self, description: UniquePtr<ArcasSessionDescription>) {
        (self.success)(description);
    }
    fn on_failure(&self, err: UniquePtr<ArcasRTCError>) {
        (self.failure)(err);
    }
}

pub struct ArcasRustSetSessionDescriptionObserver {
    success: Box<dyn Fn()>,
    failure: Box<dyn Fn(UniquePtr<ArcasRTCError>)>,
}

impl ArcasRustSetSessionDescriptionObserver {
    pub fn new(
        success: Box<dyn Fn()>,
        failure: Box<dyn Fn(UniquePtr<self::ffi::ArcasRTCError>)>,
    ) -> Self {
        Self { success, failure }
    }

    fn on_success(&self) {
        (self.success)();
    }
    fn on_failure(&self, err: UniquePtr<self::ffi::ArcasRTCError>) {
        (self.failure)(err);
    }
}

type StatsCallbackFn = dyn Fn(
    Vec<self::ffi::ArcasVideoReceiverStats>,
    Vec<self::ffi::ArcasAudioReceiverStats>,
    Vec<self::ffi::ArcasVideoSenderStats>,
    Vec<self::ffi::ArcasAudioSenderStats>,
);

pub struct ArcasRustRTCStatsCollectorCallback {
    cb: Box<StatsCallbackFn>,
}

impl ArcasRustRTCStatsCollectorCallback {
    pub fn new(cb: Box<StatsCallbackFn>) -> Self {
        Self { cb }
    }

    fn on_stats_delivered(
        self: &ArcasRustRTCStatsCollectorCallback,
        video_rx: Vec<self::ffi::ArcasVideoReceiverStats>,
        audio_rx: Vec<self::ffi::ArcasAudioReceiverStats>,
        video_tx: Vec<self::ffi::ArcasVideoSenderStats>,
        audio_tx: Vec<self::ffi::ArcasAudioSenderStats>,
    ) {
        (self.cb)(video_rx, audio_rx, video_tx, audio_tx)
    }
}
