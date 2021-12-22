#[cxx::bridge]
pub mod ffi {
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
        include!("include/alias.h");
        include!("include/peer_connection_factory.h");
        type ArcasPeerConnectionFactory;
        type ArcasRTCConfiguration = crate::shared_bridge::ffi::ArcasRTCConfiguration;
        type ArcasPeerConnectionObserver =
            crate::peer_connection_observer::ffi::ArcasPeerConnectionObserver;
        type ArcasPeerConnection = crate::peer_connection::ffi::ArcasPeerConnection;
        type ArcasVideoTrackSource = crate::video_track_source::ffi::ArcasVideoTrackSource;
        type ArcasVideoTrack = crate::video_track::ffi::ArcasVideoTrack;
        type ArcasSDPSemantics = crate::shared_bridge::ffi::ArcasSDPSemantics;

        fn create_rtc_configuration(
            config: ArcasPeerConnectionConfig,
        ) -> UniquePtr<ArcasRTCConfiguration>;

        fn gen_unique_peer_connection_factory() -> UniquePtr<ArcasPeerConnectionFactory>;

        // ArcasPeerConnectionFactory
        /// PeerConnection objects are threadsafe and can be shared between threads.
        /// the actual work happens on the worker thread.
        ///
        /// # Safety
        ///
        /// The observer must be kept alive as long as the peer connection object.
        ///
        unsafe fn create_peer_connection(
            self: &ArcasPeerConnectionFactory,
            config: UniquePtr<ArcasRTCConfiguration>,
            observer: *mut ArcasPeerConnectionObserver,
        ) -> SharedPtr<ArcasPeerConnection>;

        fn create_video_track(
            self: &ArcasPeerConnectionFactory,
            id: String,
            source: &ArcasVideoTrackSource,
        ) -> UniquePtr<ArcasVideoTrack>;
    }
}
