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
        os::unix::thread,
        sync::{mpsc, Arc},
        thread::sleep_ms,
    };

    use cxx::{SharedPtr, UniquePtr};
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

    fn create_test_observer() -> SharedPtr<ffi::ArcasPeerConnectionObserver> {
        ffi::create_peer_connection_observer(Box::new(PeerConnectionObserverProxy::new(Box::new(
            DummyPeerConnectionObserver {},
        ))))
    }

    pub struct DummyEncoderFactory {}

    impl VideoEncoderFactoryImpl for DummyEncoderFactory {
        fn get_supported_formats(&self) -> UniquePtr<cxx::CxxVector<ffi::ArcasCxxSdpVideoFormat>> {
            create_sdp_video_format_list(crate::ffi::ArcasSdpVideoFormatVecInit {
                list: vec![crate::ffi::ArcasSdpVideoFormatInit {
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
            format: &ffi::ArcasCxxSdpVideoFormat,
        ) -> ffi::ArcasVideoEncoderFactoryCodecInfo {
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
            if name == "VP9" {
                return ffi::ArcasVideoEncoderFactoryCodecSupport {
                    is_supported: true,
                    is_power_efficient: true,
                };
            }

            return ffi::ArcasVideoEncoderFactoryCodecSupport {
                is_supported: false,
                is_power_efficient: false,
            };
        }

        fn create_video_encoder(
            &self,
            format: &ffi::ArcasCxxSdpVideoFormat,
        ) -> Box<VideoEncoderProxy> {
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
            return ArcasVideoEncoderInfo {
                scaling_settings: ArcasVideoEncoderScalingSettings {
                    kOff: true,
                    low: 0,
                    high: 1,
                    min_pixels: 1,
                    thresholds: vec![],
                },
                requested_resolution_alignment: 420,
                apply_alignment_to_all_simulcast_layers: false,
                supports_native_handle: false,
                implementation_name: "test-encoder".into(),
                has_trusted_rate_controller: false,
                is_hardware_accelerated: false,
                has_internal_source: false,
                fps_allocation: vec![],
                resolution_bitrate_limits: vec![
                    crate::ffi::ArcasVideoEncoderResolutionBitrateLimits {
                        frame_size_pixels: 420,
                        min_start_bitrate_bps: 10000,
                        max_bitrate_bps: 100000000,
                        min_bitrate_bps: 100000,
                    },
                ],
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
            ice_servers: vec![ice.clone()],
            sdp_semantics: ffi::ArcasSDPSemantics::kUnifiedPlan,
        });

        let video_encoder_factory = ffi::create_arcas_video_encoder_factory(Box::new(
            VideoEncoderFactoryProxy::new(Box::new(DummyEncoderFactory {})),
        ));

        // Each api has it's own threadpool...
        let api1 = ffi::create_arcas_api();
        let api2 = ffi::create_arcas_api();

        let mut factory1 =
            api1.create_factory_with_arcas_video_encoder_factory(video_encoder_factory);
        let observer = create_test_observer();
        let pc = factory1.create_peer_connection(config, observer.clone());
        let mut source = crate::ffi::create_arcas_video_track_source();
        let track = unsafe {
            factory1
                .as_mut()
                .unwrap()
                .create_video_track("test".into(), source.pin_mut())
        };
        pc.add_video_track(track, ["test".into()].to_vec());
        // ensure we don't crash easily...

        let transceiver = pc.add_video_transceiver();
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
        let set_session_observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_tx.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| assert!(false, "Failed to set description")),
        );
        let cc_observer = Box::new(set_session_observer);
        pc.set_local_description(cc_observer, sdp.clone());
        set_rx.recv().expect("Can set description");

        let factory2 = api2.create_factory();
        let observer2 = create_test_observer();
        let pc2 = factory2.create_peer_connection(config2, observer2.clone());
        let (set_remote_tx, set_remote_rx) = mpsc::channel();
        let set_session_observer = ArcasRustSetSessionDescriptionObserver::new(
            Box::new(move || {
                set_remote_tx.send(1).expect("Can send set desc message");
            }),
            Box::new(move |_err| assert!(false, "Failed to set description")),
        );
        pc2.set_remote_description(Box::new(set_session_observer), sdp.clone());
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
                set_remote_tx2.send(1).expect("Cyn send set desc message");
            }),
            Box::new(move |_err| assert!(false, "Failed to set description")),
        );
        pc.set_remote_description(Box::new(observer), answer_for_remote);
        set_remote_rx2.recv().expect("Can finish connection loop");

        for i in 0..255 {
            std::thread::sleep_ms(1);
            let zeroed = &mut [1u8, 2, 3, i];
            unsafe {
                source.push_i420_data(100, 100, 0, 0, 0, zeroed.as_ptr());
            }
        }
    }
}
