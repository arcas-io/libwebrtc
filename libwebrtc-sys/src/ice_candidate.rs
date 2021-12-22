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

        fn id(self: &ArcasICECandidate) -> String;
        fn to_string(self: &ArcasICECandidate) -> String;
        fn sdp_mid(self: &ArcasICECandidate) -> String;
        fn sdp_mline_index(self: &ArcasICECandidate) -> u32;

        fn create_arcas_ice_candidate(
            sdp_mid: String,
            sdp_mline_index: u32,
            sdp: String,
        ) -> ArcasCreateICECandidateResult;
    }
}
