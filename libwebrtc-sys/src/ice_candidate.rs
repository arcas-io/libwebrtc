#[cxx::bridge]
pub mod ffi {
    struct ArcasICECandidateError {
        line: String,
        description: String,
    }
    struct ArcasCreateICECandidateResult {
        ok: bool,
        candidate: UniquePtr<ArcasICECandidate>,
        error: ArcasICECandidateError,
    }
    unsafe extern "C++" {
        include!("include/ice_candidate.h");
        type ArcasICECandidate;
        type ArcasCandidate = crate::candidate::ffi::ArcasCandidate;

        fn id(self: &ArcasICECandidate) -> String;
        fn to_string(self: &ArcasICECandidate) -> String;
        fn sdp_mid(self: &ArcasICECandidate) -> String;
        fn sdp_mline_index(self: &ArcasICECandidate) -> u32;
        fn get_candidate(self: &ArcasICECandidate) -> UniquePtr<ArcasCandidate>;

        fn create_arcas_ice_candidate(
            sdp_mid: String,
            sdp_mline_index: u32,
            sdp: String,
        ) -> ArcasCreateICECandidateResult;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_to_string() {
        let sdp =
            "a=candidate:1208975227 1 udp 1845501695 xxx.xxx.xxx.xxx 46794 typ srflx raddr 192.168.1.144 rport 46794 generation 0"
                .to_string();
        let sdp_mid = "0".to_string();
        let ice_candidate_result = ffi::create_arcas_ice_candidate(sdp_mid, 0, sdp);
        assert!(ice_candidate_result.ok);
        let ice_candidate = ice_candidate_result.candidate;
        let candidate = ice_candidate.get_candidate();
        assert_eq!(candidate.cxx_to_string(), "Cand[:1208975227:1:udp:1845501695:xxx.xxx.xxx.xxx:46794:stun:192.168.1.144:46794:::0:0:0]".to_string());
    }
}
