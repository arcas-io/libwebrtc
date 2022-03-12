#[cxx::bridge]
pub mod ffi {
    enum CandidateComponent {
        Default,
        Rtp,
        Rtcp,
    }

    unsafe extern "C++" {
        include!("include/candidate.h");

        #[namespace = "rtc"]
        type AdapterType = crate::rtc_base::base::ffi::AdapterType;
        type ArcasCandidate;

        // ArcasCandidate
        fn id(self: &ArcasCandidate) -> String;
        fn component(self: &ArcasCandidate) -> i32;
        fn set_component(self: Pin<&mut ArcasCandidate>, val: CandidateComponent);
        fn protocol(self: &ArcasCandidate) -> String;
        fn set_protocol(self: Pin<&mut ArcasCandidate>, proto: String);
        fn relay_protocol(self: &ArcasCandidate) -> String;
        fn address(self: &ArcasCandidate) -> String;
        fn set_address(self: Pin<&mut ArcasCandidate>, host_colon_port: String);
        fn priority(self: &ArcasCandidate) -> u32;
        fn preference(self: &ArcasCandidate) -> f32;
        fn username(self: &ArcasCandidate) -> String;
        fn password(self: &ArcasCandidate) -> String;
        fn candidate_type(self: &ArcasCandidate) -> String;
        fn network_name(self: &ArcasCandidate) -> String;
        fn network_type(self: &ArcasCandidate) -> AdapterType;
        fn generation(self: &ArcasCandidate) -> u32;
        fn network_cost(self: &ArcasCandidate) -> u16;
        fn foundation(self: &ArcasCandidate) -> String;
        fn related_address(self: &ArcasCandidate) -> String;
        fn tcptype(self: &ArcasCandidate) -> String;
        fn transport_name(self: &ArcasCandidate) -> String;
        fn url(self: &ArcasCandidate) -> String;
        fn is_equivalent(self: &ArcasCandidate, other: &ArcasCandidate) -> bool;
        fn cxx_to_string(self: &ArcasCandidate) -> String;
        fn to_sensitive_string(self: &ArcasCandidate) -> String;

        fn create_arcas_candidate() -> UniquePtr<ArcasCandidate>;
    }
}
