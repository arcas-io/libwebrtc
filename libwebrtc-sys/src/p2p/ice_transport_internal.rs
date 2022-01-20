#[cxx::bridge]
pub mod ffi {

    #[derive(Debug)]
    #[repr(u32)]
    #[namespace = "cricket"]
    enum IceConnectionState {
        kIceConnectionConnecting = 0,
        kIceConnectionFailed,
        kIceConnectionConnected, // Writable, but still checking one or more
        // connections
        kIceConnectionCompleted,
    }

    #[derive(Debug)]
    #[repr(u32)]
    #[namespace = "cricket"]
    enum IceGatheringState {
        kIceGatheringNew = 0,
        kIceGatheringGathering,
        kIceGatheringComplete,
    }

    #[derive(Debug)]
    #[repr(u32)]
    #[namespace = "cricket"]
    enum ContinualGatheringPolicy {
        // All port allocator sessions will stop after a writable connection is found.
        GATHER_ONCE = 0,
        // The most recent port allocator session will keep on running.
        GATHER_CONTINUALLY,
    }

    #[derive(Debug)]
    #[repr(u32)]
    #[namespace = "cricket"]
    enum NominationMode {
        REGULAR,    // Nominate once per ICE restart (Not implemented yet).
        AGGRESSIVE, // Nominate every connection except that it will behave as if
        // REGULAR when the remote is an ICE-LITE endpoint.
        SEMI_AGGRESSIVE, // Our current implementation of the nomination algorithm.
                         // The details are described in P2PTransportChannel.
    }

    unsafe extern "C++" {
        include!("libwebrtc-sys/include/p2p/ice_transport_internal.h");
        #[namespace = "cricket"]
        type IceConnectionState;
        #[namespace = "cricket"]
        type IceGatheringState;
        #[namespace = "cricket"]
        type ContinualGatheringPolicy;
        type ArcasP2PIceConfig;
        #[namespace = "cricket"]
        type NominationMode;
        #[namespace = "rtc"]
        type AdapterType = crate::rtc_base::base::ffi::AdapterType;

        // ArcasP2PIceConfig
        fn create_arcas_p2p_ice_config() -> UniquePtr<ArcasP2PIceConfig>;
        fn set_receiving_timeout(self: Pin<&mut ArcasP2PIceConfig>, receiving_timeout: i32);
        fn set_backup_connection_ping_interval(
            self: Pin<&mut ArcasP2PIceConfig>,
            backup_connection_ping_interval: i32,
        );
        fn set_continual_gathering_policy(
            self: Pin<&mut ArcasP2PIceConfig>,
            continual_gather_policy: ContinualGatheringPolicy,
        );
        fn set_prioritize_most_likely_candidate_pairs(
            self: Pin<&mut ArcasP2PIceConfig>,
            prioritize_most_likely_candidate_pairs: bool,
        );
        fn set_stable_writable_connection_ping_interval(
            self: Pin<&mut ArcasP2PIceConfig>,
            stable_writable_connection_ping_interval: i32,
        );
        fn set_presume_writable_when_fully_relayed(
            self: Pin<&mut ArcasP2PIceConfig>,
            presume_writable_when_fully_relayed: bool,
        );
        fn set_surface_ice_candidates_on_ice_transport_type_changed(
            self: Pin<&mut ArcasP2PIceConfig>,
            surface_ice_candidates_on_ice_transport_type: bool,
        );
        fn set_regather_on_failed_networks_interval(
            self: Pin<&mut ArcasP2PIceConfig>,
            regather_on_failed_networks_interval: i32,
        );
        fn set_receiving_switching_delay(
            self: Pin<&mut ArcasP2PIceConfig>,
            receiving_switching_delay: i32,
        );
        fn set_default_nomination_mode(
            self: Pin<&mut ArcasP2PIceConfig>,
            default_nomination_mode: NominationMode,
        );
        fn set_ice_check_interval_strong_connectivity(
            self: Pin<&mut ArcasP2PIceConfig>,
            ice_check_interval_strong_connectivity: i32,
        );
        fn set_ice_check_interval_weak_connectivity(
            self: Pin<&mut ArcasP2PIceConfig>,
            ice_check_interval_weak_connectivity: i32,
        );
        fn set_ice_check_min_interval(
            self: Pin<&mut ArcasP2PIceConfig>,
            ice_check_min_interval: i32,
        );
        fn set_ice_unwritable_timeout(
            self: Pin<&mut ArcasP2PIceConfig>,
            ice_unwritable_timeout: i32,
        );
        fn set_ice_unwritable_min_checks(
            self: Pin<&mut ArcasP2PIceConfig>,
            ice_unwritable_min_checks: i32,
        );
        fn set_ice_inactive_timeout(self: Pin<&mut ArcasP2PIceConfig>, ice_inactive_timeout: i32);
        fn set_stun_keepalive_interval(
            self: Pin<&mut ArcasP2PIceConfig>,
            stun_keepalive_interval: i32,
        );
        fn set_network_preference(
            self: Pin<&mut ArcasP2PIceConfig>,
            network_preference: AdapterType,
        );

    }
}
