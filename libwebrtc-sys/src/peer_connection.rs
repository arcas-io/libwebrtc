#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use cxx::CxxString;

    use crate::{
        ffi, ArcasRustCreateSessionDescriptionObserver, ArcasRustPeerConnectionObserver,
        ArcasRustSetSessionDescriptionObserver,
    };

    use super::*;

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
            ice_servers: vec![ice.clone()],
            sdp_semantics: ffi::ArcasSDPSemantics::kUnifiedPlan,
        });

        let factory = ffi::create_factory();
        let observer = ArcasRustPeerConnectionObserver::new();
        let pc = factory.create_peer_connection(config, Box::new(observer));
        let _transceiver = pc.add_simple_media_transceiver(ffi::ArcasMediaType::MEDIA_TYPE_VIDEO);
        let (tx, rx) = mpsc::channel();

        pc.create_offer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
            Box::new(move |session_description| {
                tx.send(session_description)
                    .expect("Can send set desc message");
            }),
            Box::new(move |_err| assert!(false, "Failed to set description")),
        )));

        let sdp = rx.recv().expect("Can get offer");
        assert!(sdp.to_string().len() > 0, "has sdp string");

        let (set_tx, set_rx) = mpsc::channel();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_tx.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| assert!(false, "Failed to set description")),
        );
        let cc_observer = Box::new(observer);
        pc.set_local_description(cc_observer, sdp.clone());
        set_rx.recv().expect("Can set description");

        let observer2 = ArcasRustPeerConnectionObserver::new();
        let pc2 = factory.create_peer_connection(config2, Box::new(observer2));
        let (set_remote_tx, set_remote_rx) = mpsc::channel();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_remote_tx.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| assert!(false, "Failed to set description")),
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
            Box::new(move |err| {
                println!("got some kind of error");
                assert!(false, "Failed to create session description");
            }),
        )));
        let answer = rx_answer.recv().expect("Creates answer");
        let answer_for_remote = answer.clone();

        let (set_local_tx2, set_local_rx2) = mpsc::channel();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_local_tx2.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| assert!(false, "Failed to set description")),
        );
        pc2.set_local_description(Box::new(observer), answer);
        set_local_rx2.recv().expect("Can finish connection loop");

        let (set_remote_tx2, set_remote_rx2) = mpsc::channel();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_remote_tx2.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| assert!(false, "Failed to set description")),
        );
        pc.set_remote_description(Box::new(observer), answer_for_remote);
        set_remote_rx2.recv().expect("Can finish connection loop");
    }
}
