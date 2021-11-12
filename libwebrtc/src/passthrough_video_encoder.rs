use std::sync::Arc;

use cxx::UniquePtr;
use libwebrtc_sys::{
    ffi::{
        self, create_sdp_video_format_list, ArcasEncodedImageCallback, ArcasVideoEncoderInfo,
        ArcasVideoEncoderScalingSettings,
    },
    video_encoder::VideoEncoderImpl,
    video_encoder_factory::VideoEncoderFactoryImpl,
    VideoEncoderProxy, VideoEncoderSelectorProxy, VIDEO_CODEC_OK,
};
use parking_lot::Mutex;

pub struct PassThroughVideoEncoderFactory {}

impl PassThroughVideoEncoderFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl VideoEncoderFactoryImpl for PassThroughVideoEncoderFactory {
    fn get_supported_formats(&self) -> UniquePtr<cxx::CxxVector<ffi::ArcasCxxSdpVideoFormat>> {
        create_sdp_video_format_list(ffi::ArcasSdpVideoFormatVecInit {
            list: vec![ffi::ArcasSdpVideoFormatInit {
                // TODO: Support more than VP9
                name: "VP9".to_string(),
                parameters: vec![],
            }],
        })
    }

    fn get_implementations(&self) -> UniquePtr<cxx::CxxVector<ffi::ArcasCxxSdpVideoFormat>> {
        self.get_supported_formats()
    }

    fn query_video_encoder(
        &self,
        _format: &ffi::ArcasCxxSdpVideoFormat,
    ) -> ffi::ArcasVideoEncoderFactoryCodecInfo {
        ffi::ArcasVideoEncoderFactoryCodecInfo {
            has_internal_source: false,
        }
    }

    fn query_codec_support(
        &self,
        format: &ffi::ArcasCxxSdpVideoFormat,
        _scalability_mode: Vec<String>,
    ) -> ffi::ArcasVideoEncoderFactoryCodecSupport {
        let name = ffi::sdp_video_format_get_name(format);
        if name == "VP9" {
            return ffi::ArcasVideoEncoderFactoryCodecSupport {
                is_supported: true,
                is_power_efficient: true,
            };
        }

        ffi::ArcasVideoEncoderFactoryCodecSupport {
            is_supported: false,
            is_power_efficient: false,
        }
    }

    fn create_video_encoder(
        &self,
        _format: &ffi::ArcasCxxSdpVideoFormat,
    ) -> Box<VideoEncoderProxy> {
        Box::new(VideoEncoderProxy::new(Box::new(
            PassThroughVideoEncoder::new(),
        )))
    }

    fn get_encoder_selector(&self) -> Option<VideoEncoderSelectorProxy> {
        None
    }
}

pub struct EncoderState {
    pub callback: Option<Mutex<UniquePtr<ArcasEncodedImageCallback>>>,
}

pub struct PassThroughVideoEncoder {
    state: Arc<Mutex<EncoderState>>,
    // Hold CXX reference.
    #[allow(unused)]
    image_factory: UniquePtr<ffi::ArcasEncodedImageFactory>,
}

impl PassThroughVideoEncoder {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(EncoderState { callback: None })),
            image_factory: ffi::create_arcas_encoded_image_factory(),
        }
    }
}

impl Default for PassThroughVideoEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl VideoEncoderImpl for PassThroughVideoEncoder {
    unsafe fn init_encode(
        &self,
        _codec_settings: *const ffi::ArcasCxxVideoCodec,
        _number_of_cores: i32,
        _max_payload_sizesize: usize,
    ) -> i32 {
        *VIDEO_CODEC_OK
    }

    fn register_encode_complete_callback(
        &self,
        callback: UniquePtr<ArcasEncodedImageCallback>,
    ) -> i32 {
        self.state.lock().callback = Some(Mutex::new(callback));
        *VIDEO_CODEC_OK
    }

    fn release(&self) -> i32 {
        *VIDEO_CODEC_OK
    }

    fn encode(
        &self,
        frame: &ffi::ArcasCxxVideoFrame,
        _frame_types: *const cxx::CxxVector<ffi::ArcasCxxVideoFrameType>,
    ) -> i32 {
        match &self.state.lock().callback {
            Some(callback) => {
                let video_frame_data = ffi::extract_arcas_video_frame_to_raw_frame_buffer(frame);
                let _out = unsafe {
                    callback.lock().as_mut().unwrap().on_encoded_image(
                        video_frame_data.encoded_image_ref(),
                        video_frame_data
                            .arcas_codec_specific_info()
                            .as_ref()
                            .unwrap(),
                    )
                };
            }
            None => {}
        }

        *VIDEO_CODEC_OK
    }

    fn set_rates(&self, _parameters: UniquePtr<ffi::ArcasVideoEncoderRateControlParameters>) {}

    fn on_packet_loss_rate_update(&self, _packet_loss_rate: f32) {}

    fn on_rtt_update(&self, _rtt: i64) {}

    fn on_loss_notification(&self, _loss_notification: ffi::ArcasVideoEncoderLossNotification) {
        println!("LOSS NOTIFICATION");
    }

    fn get_encoder_info(&self) -> ArcasVideoEncoderInfo {
        ArcasVideoEncoderInfo {
            scaling_settings: ArcasVideoEncoderScalingSettings {
                kOff: true,
                low: 0,
                high: 1,
                min_pixels: 1,
                thresholds: vec![],
            },
            requested_resolution_alignment: 420,
            apply_alignment_to_all_simulcast_layers: false,
            supports_native_handle: true,
            implementation_name: "passthrough-encoder".into(),
            has_trusted_rate_controller: false,
            is_hardware_accelerated: false,
            has_internal_source: false,
            fps_allocation: vec![],
            resolution_bitrate_limits: vec![ffi::ArcasVideoEncoderResolutionBitrateLimits {
                frame_size_pixels: 420,
                min_start_bitrate_bps: 10000,
                max_bitrate_bps: 100000000,
                min_bitrate_bps: 100000,
            }],
            supports_simulcast: false,
            preferred_pixel_formats: vec![ffi::ArcasCxxVideoFrameBufferType::kNative],
            is_qp_trusted: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crossbeam_channel::{select, unbounded, Receiver, Sender};

    use libwebrtc_sys::{
        ffi::{
            create_peer_connection_observer, ArcasICECandidate, ArcasPeerConnectionObserver,
            ArcasRTPVideoTransceiver,
        },
        peer_connection::PeerConnectionObserverImpl,
        ArcasRustCreateSessionDescriptionObserver, ArcasRustRTCStatsCollectorCallback,
        ArcasRustSetSessionDescriptionObserver, PeerConnectionObserverProxy,
        VideoEncoderFactoryProxy,
    };

    use crate::{
        encoded_video_frame_producer::{
            EncodedFrameProducerProducer, DEFAULT_HEIGHT, DEFAULT_WIDTH,
        },
        raw_video_frame_producer::{GStreamerRawFrameProducer, RawFrameProducer},
        video_codec::VideoCodec,
        video_encoder::VideoEncoderSettings,
        video_frame::AsCxxVideoFrame,
    };

    use super::*;
    pub struct TestIcePeerConnectionObserver {
        ice_tx: Sender<UniquePtr<ArcasICECandidate>>,
        video_tx: Sender<UniquePtr<ArcasRTPVideoTransceiver>>,
    }

    impl TestIcePeerConnectionObserver {
        pub fn new(
            video_tx: Sender<UniquePtr<ArcasRTPVideoTransceiver>>,
            ice_tx: Sender<UniquePtr<ArcasICECandidate>>,
        ) -> TestIcePeerConnectionObserver {
            TestIcePeerConnectionObserver { ice_tx, video_tx }
        }
    }

    impl PeerConnectionObserverImpl for TestIcePeerConnectionObserver {
        fn on_ice_candidate(&self, candidate: UniquePtr<ArcasICECandidate>) {
            self.ice_tx.send(candidate).unwrap();
        }

        fn on_connection_change(&self, state: ffi::ArcasPeerConnectionState) {
            println!("GOT CONNECTION CHANGE: {:?}", state);
        }

        fn on_video_track(&self, transceiver: UniquePtr<ffi::ArcasRTPVideoTransceiver>) {
            println!("GOT TRANSCEIVER: {:?}", transceiver.mid());
            self.video_tx.send(transceiver).unwrap();
        }
    }

    pub fn create_test_ice_peer_connection_observer() -> (
        Receiver<UniquePtr<ArcasICECandidate>>,
        Receiver<UniquePtr<ArcasRTPVideoTransceiver>>,
        UniquePtr<ArcasPeerConnectionObserver>,
    ) {
        let (tx, rx) = unbounded();
        let (tx_video, rx_video) = unbounded();
        let out = create_peer_connection_observer(Box::new(PeerConnectionObserverProxy::new(
            Box::new(TestIcePeerConnectionObserver::new(tx_video, tx)),
        )));
        (rx, rx_video, out)
    }

    #[test]
    fn test_custom_video_encoder_factory() {
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
            ice_servers: vec![ice],
            sdp_semantics: ffi::ArcasSDPSemantics::kUnifiedPlan,
        });

        let video_encoder_factory = ffi::create_arcas_video_encoder_factory(Box::new(
            VideoEncoderFactoryProxy::new(Box::new(PassThroughVideoEncoderFactory {})),
        ));

        // Each api has it's own threadpool...
        let api1 = ffi::create_arcas_api();
        let api2 = ffi::create_arcas_api();

        let mut factory1 =
            api1.create_factory_with_arcas_video_encoder_factory(video_encoder_factory);
        let (ice_rx, _, mut observer) = create_test_ice_peer_connection_observer();
        let pc = unsafe {
            factory1.create_peer_connection(config, observer.pin_mut().get_unchecked_mut())
        };
        let source = ffi::create_arcas_video_track_source();
        let track = factory1
            .as_mut()
            .unwrap()
            .create_video_track("test".into(), source.as_ref().unwrap());
        pc.add_video_track(track, ["test".into()].to_vec());

        let (tx, rx) = unbounded();

        pc.create_offer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
            Box::new(move |session_description| {
                tx.send(session_description)
                    .expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        )));

        let sdp = rx.recv().expect("Can get offer");
        assert!(!sdp.cxx_to_string().is_empty(), "has sdp string");

        let (set_tx, set_rx) = unbounded();
        let set_session_observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_tx.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        let cc_observer = Box::new(set_session_observer);
        println!("SET OFFER\n: {}", sdp.cxx_to_string());
        pc.set_local_description(cc_observer, sdp.clone_cxx());
        set_rx.recv().expect("Can set description");

        let factory2 = api2.create_factory();
        let (ice_rx2, video_transceiver_rx, mut observer2) =
            create_test_ice_peer_connection_observer();
        let pc2 = unsafe {
            factory2.create_peer_connection(config2, observer2.pin_mut().get_unchecked_mut())
        };
        let (set_remote_tx, set_remote_rx) = unbounded();
        let set_session_observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_remote_tx.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        pc2.set_remote_description(Box::new(set_session_observer), sdp.clone_cxx());
        set_remote_rx.recv().expect("Can set description");
        let (tx_answer, rx_answer) = unbounded();
        pc2.create_answer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
            Box::new(move |session_description| {
                assert_eq!(session_description.get_type(), ffi::ArcasSDPType::kAnswer);
                println!("got sdp: {}", session_description.cxx_to_string(),);
                tx_answer.send(session_description).expect("Can send");
            }),
            Box::new(move |_err| {
                println!("got some kind of error");
                panic!("Failed to create session description");
            }),
        )));
        let answer = rx_answer.recv().expect("Creates answer");
        let answer_for_remote = answer.clone_cxx();

        let (set_local_tx2, set_local_rx2) = unbounded();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_local_tx2.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        pc2.set_local_description(Box::new(observer), answer);
        set_local_rx2.recv().expect("Can finish connection loop");

        let (set_remote_tx2, set_remote_rx2) = unbounded();
        let observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_remote_tx2.send(1).expect("Cyn send set desc message");
            }),
            Box::new(move |_err| panic!("Failed to set description")),
        );
        pc.set_remote_description(Box::new(observer), answer_for_remote);
        set_remote_rx2.recv().expect("Can finish connection loop");

        let pc1_ice = ice_rx.recv().expect("Can get ice");
        let pc2_ice = ice_rx2.recv().expect("Can get ice");
        pc.add_ice_candidate(pc2_ice);
        pc2.add_ice_candidate(pc1_ice);

        let mut codec = VideoCodec::vp9(DEFAULT_WIDTH / 2, DEFAULT_HEIGHT / 2, 10);
        codec.primary.max_bitrate_kbs = 2000;
        codec.primary.min_bitrate_kbs = 30;
        codec.primary.target_bitrate_kbs = 2000;

        let mut raw_frames = GStreamerRawFrameProducer::default_pipeline(&codec).unwrap();
        let encoder =
            EncodedFrameProducerProducer::new(codec, VideoEncoderSettings::default()).unwrap();
        let raw_frames_rx = raw_frames.start().unwrap();

        let (cancel_push_tx, cancel_push_rx) = unbounded::<()>();
        std::thread::spawn(move || loop {
            select! {
                recv(cancel_push_rx) -> _ => {
                    return;
                },
                recv(raw_frames_rx) -> frame_result => {
                    let frame = frame_result.unwrap();
                    encoder.raw_frame_tx.send(frame).unwrap();
                    let frame = encoder.encoded_rx.recv().unwrap();
                    source.push_frame(frame.as_cxx_video_frame_ref().unwrap());
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        });

        std::thread::sleep(std::time::Duration::from_millis(1000));
        let (test_done_tx, test_done_rx) = unbounded();
        std::thread::spawn(move || loop {
            let (stat_tx, stat_rx) = unbounded();
            let observer = Box::new(ArcasRustRTCStatsCollectorCallback::new(Box::new(
                move |video_recv, _, _, _| {
                    stat_tx.send(video_recv).expect("Can send");
                },
            )));
            pc2.get_stats(observer);
            let stats = stat_rx.recv().unwrap();
            for stat in stats {
                if stat.frames_decoded > 1 {
                    if cancel_push_tx.send(()).is_ok() {}
                    test_done_tx.send(1).unwrap();
                    // Very important otherwise we crash.
                    pc2.close();
                    break;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        });
        let _video_transceiver = video_transceiver_rx.recv().unwrap();
        test_done_rx
            .recv_deadline(Instant::now() + std::time::Duration::from_secs(10))
            .unwrap();
        // Very important otherwise we crash.
        pc.close();
    }
}
