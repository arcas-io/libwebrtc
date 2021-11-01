use cxx::{CxxVector, UniquePtr};

use crate::{
    ffi::{
        ArcasCxxDataRate, ArcasCxxSdpVideoFormat, ArcasVideoEncoderFactoryCodecInfo,
        ArcasVideoEncoderFactoryCodecSupport,
    },
    VideoEncoderProxy,
};

pub trait VideoEncoderFactoryImpl {
    /// Returns a list of supported video formats in order of preference, to use
    /// for signaling etc.
    fn get_supported_formats(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

    /// Returns a list of supported video formats in order of preference, that can
    /// also be tagged with additional information to allow the VideoEncoderFactory
    /// to separate between different implementations when CreateVideoEncoder is
    /// called.
    fn get_implementations(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

    /// Returns information about how this format will be encoded. The specified
    /// format must be one of the supported formats by this factory.
    fn query_video_encoder(
        &self,
        format: &ArcasCxxSdpVideoFormat,
    ) -> ArcasVideoEncoderFactoryCodecInfo;

    /// Query whether the specifed format is supported or not and if it will be
    /// power efficient, which is currently interpreted as if there is support for
    /// hardware acceleration.
    /// See https://w3c.github.io/webrtc-svc/#scalabilitymodes* for a specification
    /// of valid values for `scalability_mode`.
    fn query_codec_support(
        &self,
        format: &ArcasCxxSdpVideoFormat,
        scalability_mode: Vec<String>,
    ) -> ArcasVideoEncoderFactoryCodecSupport;

    /// Create video encoder returning a Box'ed trait object for the VideoEncoderImpl.
    fn create_video_encoder(&self, format: &ArcasCxxSdpVideoFormat) -> Box<VideoEncoderProxy>;

    /// Return an optional encoder selector (see `VideoEncoderSelectorImpl`) empty Vec is none.
    fn get_encoder_selector(&self) -> Option<VideoEncoderSelectorProxy>;
}
pub struct VideoEncoderFactoryProxy {
    api: Box<dyn VideoEncoderFactoryImpl>,
}

impl VideoEncoderFactoryProxy {
    pub fn new(api: Box<dyn VideoEncoderFactoryImpl>) -> Self {
        Self { api }
    }

    pub fn get_supported_formats(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.get_supported_formats()
    }
    pub fn get_implementations(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.get_implementations()
    }

    pub fn query_video_encoder(
        &self,
        format: &ArcasCxxSdpVideoFormat,
    ) -> ArcasVideoEncoderFactoryCodecInfo {
        self.api.query_video_encoder(format)
    }

    pub fn query_codec_support(
        &self,
        format: &ArcasCxxSdpVideoFormat,
        scalability_mode: Vec<String>,
    ) -> ArcasVideoEncoderFactoryCodecSupport {
        self.api.query_codec_support(format, scalability_mode)
    }

    pub fn create_video_encoder(&self, format: &ArcasCxxSdpVideoFormat) -> Box<VideoEncoderProxy> {
        self.api.create_video_encoder(format)
    }

    pub fn get_encoder_selector(&self) -> Vec<VideoEncoderSelectorProxy> {
        match self.api.get_encoder_selector() {
            Some(value) => {
                vec![value]
            }
            None => {
                vec![]
            }
        }
    }
}

pub trait VideoEncoderSelectorImpl {
    /// Informs the encoder selector about which encoder that is currently being
    /// used.
    fn on_current_encoder(&self, format: &ArcasCxxSdpVideoFormat);

    /// Called every time the available bitrate is updated. Should return a
    /// non-empty if an encoder switch should be performed.
    fn on_available_bitrate(
        &self,
        data_rate: &ArcasCxxDataRate,
    ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;

    /// Called if the currently used encoder reports itself as broken. Should
    /// return a non-empty if an encoder switch should be performed.
    fn on_encoder_broken(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>>;
}

pub struct VideoEncoderSelectorProxy {
    api: Box<dyn VideoEncoderSelectorImpl>,
}

impl VideoEncoderSelectorProxy {
    pub fn new(api: Box<dyn VideoEncoderSelectorImpl>) -> Self {
        Self { api }
    }

    pub fn on_current_encoder(&self, format: &ArcasCxxSdpVideoFormat) {
        self.api.on_current_encoder(format);
    }

    pub fn on_available_bitrate(
        &self,
        data_rate: &ArcasCxxDataRate,
    ) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.on_available_bitrate(data_rate)
    }

    pub fn on_encoder_broken(&self) -> UniquePtr<CxxVector<ArcasCxxSdpVideoFormat>> {
        self.api.on_encoder_broken()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        convert::TryInto,
        sync::{mpsc, Arc},
    };

    use cxx::UniquePtr;
    use parking_lot::{lock_api::RawMutex, Mutex};

    use crate::{
        ffi::{
            self, create_sdp_video_format_list, ArcasEncodedImageCallback,
            ArcasVideoEncoderFactoryCodecInfo, ArcasVideoEncoderInfo,
            ArcasVideoEncoderScalingSettings,
        },
        peer_connection::DummyPeerConnectionObserver,
        video_encoder::VideoEncoderImpl,
        ArcasRustCreateSessionDescriptionObserver, ArcasRustSetSessionDescriptionObserver,
        PeerConnectionObserverProxy, VideoEncoderFactoryProxy, VideoEncoderProxy,
    };

    use super::VideoEncoderFactoryImpl;

    pub struct DummyEncoderFactory {}

    impl VideoEncoderFactoryImpl for DummyEncoderFactory {
        fn get_supported_formats(&self) -> UniquePtr<cxx::CxxVector<ffi::ArcasCxxSdpVideoFormat>> {
            println!("GET SUPPORT FORMATS");
            create_sdp_video_format_list(crate::ffi::ArcasSdpVideoFormatVecInit {
                list: vec![crate::ffi::ArcasSdpVideoFormatInit {
                    name: "H264".to_string(),
                    parameters: vec![
                        crate::ffi::ArcasRustDict {
                            key: "profile-level-id".to_string(),
                            value: "42f00b".to_string(),
                        },
                        crate::ffi::ArcasRustDict {
                            key: "level-asymmetry-allowed".to_string(),
                            value: "1".to_string(),
                        },
                        crate::ffi::ArcasRustDict {
                            key: "packetization-mode".to_string(),
                            value: "0".to_string(),
                        },
                    ],
                }],
            })
        }

        fn get_implementations(&self) -> UniquePtr<cxx::CxxVector<ffi::ArcasCxxSdpVideoFormat>> {
            self.get_supported_formats()
        }

        fn query_video_encoder(
            &self,
            format: &ffi::ArcasCxxSdpVideoFormat,
        ) -> ffi::ArcasVideoEncoderFactoryCodecInfo {
            println!("GOT HERE xfoo");
            ffi::ArcasVideoEncoderFactoryCodecInfo {
                has_internal_source: false,
            }
        }

        fn query_codec_support(
            &self,
            format: &ffi::ArcasCxxSdpVideoFormat,
            scalability_mode: Vec<String>,
        ) -> ffi::ArcasVideoEncoderFactoryCodecSupport {
            let name = crate::ffi::sdp_video_format_get_name(format);
            println!("XFOOO : here {:?}", name);
            if name == "H264" {
                return ffi::ArcasVideoEncoderFactoryCodecSupport {
                    is_supported: true,
                    is_power_efficient: true,
                };
            }

            println!("XFOOO :  thre");
            return ffi::ArcasVideoEncoderFactoryCodecSupport {
                is_supported: false,
                is_power_efficient: false,
            };
        }

        fn create_video_encoder(
            &self,
            format: &ffi::ArcasCxxSdpVideoFormat,
        ) -> Box<VideoEncoderProxy> {
            println!("GOT HERE create video encoder\n");
            Box::new(VideoEncoderProxy::new(Box::new(DummyVideoEncoder::new())))
        }

        fn get_encoder_selector(&self) -> Option<crate::VideoEncoderSelectorProxy> {
            None
        }
    }

    pub struct EncoderState {
        pub callback: Option<Mutex<UniquePtr<ArcasEncodedImageCallback>>>,
    }

    pub struct DummyVideoEncoder {
        state: Arc<Mutex<EncoderState>>,
        image_factory: UniquePtr<ffi::ArcasEncodedImageFactory>,
    }

    impl DummyVideoEncoder {
        pub fn new() -> Self {
            Self {
                state: Arc::new(Mutex::new(EncoderState { callback: None })),
                image_factory: ffi::create_arcas_encoded_image_factory(),
            }
        }
    }

    impl VideoEncoderImpl for DummyVideoEncoder {
        fn init_encode(
            &self,
            codec_settings: UniquePtr<crate::ffi::ArcasVideoCodec>,
            number_of_cores: i32,
            max_payload_size: usize,
        ) -> i32 {
            return crate::WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_OK;
        }

        fn register_encode_complete_callback(
            &self,
            callback: UniquePtr<ArcasEncodedImageCallback>,
        ) -> i32 {
            self.state.lock().callback = Some(Mutex::new(callback));
            return crate::WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_OK;
        }

        fn release(&self) -> i32 {
            return crate::WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_OK;
        }

        fn encode(
            &self,
            frame: &crate::ffi::CxxVideoFrame,
            frame_types: *const cxx::CxxVector<crate::ffi::ArcasVideoFrameType>,
        ) -> i32 {
            println!("GOT HERE 2\n");
            match &self.state.lock().callback {
                Some(callback) => {
                    let image_buffer = self.image_factory.create_empty_encoded_image_buffer();
                    let image = self.image_factory.create_encoded_image();
                    let info = ffi::create_arcas_codec_specific_info();
                    let image = self
                        .image_factory
                        .set_encoded_image_buffer(image, image_buffer);
                    let _out = unsafe {
                        callback
                            .lock()
                            .as_mut()
                            .unwrap()
                            .on_encoded_image(image.as_ref().unwrap(), info.as_ref().unwrap())
                    };
                }
                None => {}
            }

            return crate::WEBRTC_VIDEO_ENCODING_ERR.VIDEO_CODEC_OK;
        }

        fn get_encoder_info(&self) -> ArcasVideoEncoderInfo {
            println!("GOT HERE\n");
            return ArcasVideoEncoderInfo {
                scaling_settings: ArcasVideoEncoderScalingSettings {
                    kOff: true,
                    low: 0,
                    high: 1,
                    min_pixels: 100,
                    thresholds: vec![],
                },
                requested_resolution_alignment: 1,
                apply_alignment_to_all_simulcast_layers: false,
                supports_native_handle: false,
                implementation_name: "test-encoder".into(),
                has_trusted_rate_controller: false,
                is_hardware_accelerated: false,
                has_internal_source: false,
                fps_allocation: vec![],
                resolution_bitrate_limits: vec![],
                supports_simulcast: false,
                preferred_pixel_formats: vec![crate::ffi::ArcasCxxVideoFrameBufferType::kI420],
                is_qp_trusted: vec![],
            };
        }

        fn set_rates(
            &self,
            _parameters: UniquePtr<crate::ffi::ArcasVideoEncoderRateControlParameters>,
        ) {
        }

        fn on_packet_loss_rate_update(&self, _packet_loss_rate: f32) {}

        fn on_rtt_update(&self, rtt: i64) {}

        fn on_loss_notification(
            &self,
            _loss_notification: crate::ffi::ArcasVideoEncoderLossNotification,
        ) {
        }
    }

    // #[test]
    // fn test_basic_peer_connection_init() {
    //     let ice = ffi::ArcasICEServer {
    //         urls: vec!["stun:stun.l.google.com:19302".to_owned()],
    //         username: "".to_owned(),
    //         password: "".to_owned(),
    //     };
    //     let config = ffi::create_rtc_configuration(ffi::ArcasPeerConnectionConfig {
    //         ice_servers: vec![ice.clone()],
    //         sdp_semantics: ffi::ArcasSDPSemantics::kUnifiedPlan,
    //     });
    //     let config2 = ffi::create_rtc_configuration(ffi::ArcasPeerConnectionConfig {
    //         ice_servers: vec![ice.clone()],
    //         sdp_semantics: ffi::ArcasSDPSemantics::kUnifiedPlan,
    //     });

    //     let video_encoder_factory = ffi::create_arcas_video_encoder_factory(Box::new(
    //         VideoEncoderFactoryProxy::new(Box::new(DummyEncoderFactory {})),
    //     ));

    //     let mut factory =
    //         ffi::create_factory_with_arcas_video_encoder_factory(video_encoder_factory);
    //     let observer = PeerConnectionObserverProxy::new(Box::new(DummyPeerConnectionObserver {}));
    //     let pc = factory.create_peer_connection(config, Box::new(observer));
    //     let source = crate::ffi::create_arcas_video_track_source();
    //     let track = unsafe {
    //         factory
    //             .as_mut()
    //             .unwrap()
    //             .create_video_track("test".into(), source.clone())
    //     };
    //     pc.add_video_track(track, ["test".into()].to_vec());

    //     // ensure we don't crash easily...
    //     for _i in 0..100 {
    //         let zeroed = &mut [1u8, 2, 3];
    //         unsafe {
    //             crate::ffi::push_i420_to_video_track_source(
    //                 source.clone(),
    //                 100,
    //                 100,
    //                 0,
    //                 0,
    //                 0,
    //                 zeroed.as_mut_ptr(),
    //             );
    //         }
    //     }

    //     let _transceiver = pc.add_video_transceiver();
    //     let (tx, rx) = mpsc::channel();

    //     pc.create_offer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
    //         Box::new(move |session_description| {
    //             tx.send(session_description)
    //                 .expect("Can send set desc message");
    //         }),
    //         Box::new(move |_err| assert!(false, "Failed to set description")),
    //     )));

    //     let sdp = rx.recv().expect("Can get offer");
    //     assert!(sdp.to_string().len() > 0, "has sdp string");
    //     println!("VIDEO SDP: {}", sdp.to_string());

    //     let (set_tx, set_rx) = mpsc::channel();
    //     let observer = ArcasRustSetSessionDescriptionObserver::new(
    //         Box::new(move || {
    //             set_tx.send(1).expect("Can send set desc message");
    //         }),
    //         Box::new(move |_err| assert!(false, "Failed to set description")),
    //     );
    //     let cc_observer = Box::new(observer);
    //     pc.set_local_description(cc_observer, sdp.clone());
    //     set_rx.recv().expect("Can set description");

    //     let observer2 = PeerConnectionObserverProxy::new(Box::new(DummyPeerConnectionObserver {}));
    //     let pc2 = factory.create_peer_connection(config2, Box::new(observer2));
    //     let (set_remote_tx, set_remote_rx) = mpsc::channel();
    //     let observer = ArcasRustSetSessionDescriptionObserver::new(
    //         Box::new(move || {
    //             set_remote_tx.send(1).expect("Can send set desc message");
    //         }),
    //         Box::new(move |_err| assert!(false, "Failed to set description")),
    //     );
    //     pc2.set_remote_description(Box::new(observer), sdp.clone());
    //     set_remote_rx.recv().expect("Can set description");
    //     let (tx_answer, rx_answer) = mpsc::channel();
    //     pc2.create_answer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
    //         Box::new(move |session_description| {
    //             assert_eq!(session_description.get_type(), ffi::ArcasSDPType::kAnswer);
    //             println!("got sdp: {}", session_description.to_string(),);
    //             tx_answer.send(session_description).expect("Can send");
    //         }),
    //         Box::new(move |err| {
    //             println!("got some kind of error");
    //             assert!(false, "Failed to create session description");
    //         }),
    //     )));
    //     let answer = rx_answer.recv().expect("Creates answer");
    //     let answer_for_remote = answer.clone();

    //     let (set_local_tx2, set_local_rx2) = mpsc::channel();
    //     let observer = ArcasRustSetSessionDescriptionObserver::new(
    //         Box::new(move || {
    //             set_local_tx2.send(1).expect("Can send set desc message");
    //         }),
    //         Box::new(move |_err| assert!(false, "Failed to set description")),
    //     );
    //     pc2.set_local_description(Box::new(observer), answer);
    //     set_local_rx2.recv().expect("Can finish connection loop");

    //     let (set_remote_tx2, set_remote_rx2) = mpsc::channel();
    //     let observer = ArcasRustSetSessionDescriptionObserver::new(
    //         Box::new(move || {
    //             set_remote_tx2.send(1).expect("Can send set desc message");
    //         }),
    //         Box::new(move |_err| assert!(false, "Failed to set description")),
    //     );
    //     pc.set_remote_description(Box::new(observer), answer_for_remote);
    //     set_remote_rx2.recv().expect("Can finish connection loop");
    // }
}
