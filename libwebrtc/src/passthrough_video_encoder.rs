use std::sync::Arc;

use cxx::UniquePtr;
use libwebrtc_sys::{
    ffi::{
        self, create_sdp_video_format_list, ArcasEncodedImageCallback, ArcasVideoEncoderInfo,
        ArcasVideoEncoderScalingSettings,
    },
    video_encoding::{VideoEncoderFactoryImpl, VideoEncoderImpl},
    VideoEncoderProxy, VideoEncoderSelectorProxy, VIDEO_CODEC_OK,
};
use parking_lot::Mutex;

pub struct PassThroughVideoEncoderFactory {}

impl PassThroughVideoEncoderFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for PassThroughVideoEncoderFactory {
    fn default() -> Self {
        Self::new()
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
        &mut self,
        _codec_settings: *const ffi::ArcasCxxVideoCodec,
        _number_of_cores: i32,
        _max_payload_sizesize: usize,
    ) -> i32 {
        *VIDEO_CODEC_OK
    }

    fn register_encode_complete_callback(
        &mut self,
        callback: UniquePtr<ArcasEncodedImageCallback>,
    ) -> i32 {
        self.state.lock().callback = Some(Mutex::new(callback));
        *VIDEO_CODEC_OK
    }

    fn release(&mut self) -> i32 {
        *VIDEO_CODEC_OK
    }

    unsafe fn encode(
        &mut self,
        frame: &ffi::ArcasCxxVideoFrame,
        _frame_types: *const cxx::CxxVector<ffi::ArcasCxxVideoFrameType>,
    ) -> i32 {
        match &self.state.lock().callback {
            Some(callback) => {
                let video_frame_data = ffi::extract_arcas_video_frame_to_raw_frame_buffer(frame);
                callback.lock().as_mut().unwrap().on_encoded_image(
                    video_frame_data.encoded_image_ref(),
                    video_frame_data
                        .arcas_codec_specific_info()
                        .as_ref()
                        .unwrap(),
                );
            }
            None => {}
        }

        *VIDEO_CODEC_OK
    }

    fn set_rates(&mut self, _parameters: UniquePtr<ffi::ArcasVideoEncoderRateControlParameters>) {}

    fn on_packet_loss_rate_update(&mut self, _packet_loss_rate: f32) {}

    fn on_rtt_update(&mut self, _rtt: i64) {}

    fn on_loss_notification(&mut self, _loss_notification: ffi::ArcasVideoEncoderLossNotification) {
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
    use crossbeam_channel::{select, unbounded};

    use tokio::{sync::mpsc::channel, time::sleep};

    use crate::{
        encoded_video_frame_producer::{
            EncodedFrameProducerProducer, DEFAULT_HEIGHT, DEFAULT_WIDTH,
        },
        factory::Factory,
        peer_connection::PeerConnectionConfig,
        peer_connection_observer::ObserverSenders,
        raw_video_frame_producer::{GStreamerRawFrameProducer, RawFrameProducer},
        video_codec::VideoCodec,
        video_encoder::VideoEncoderSettings,
        video_track_source::VideoTrackSource,
    };

    #[tokio::test]
    async fn test_custom_video_encoder_factory() {
        let api_factory = Factory::new();
        let api_factory2 = Factory::new();
        let config = PeerConnectionConfig::default();
        let config2 = PeerConnectionConfig::default();

        let (ice_tx, mut ice_rx) = channel(100);
        let (ice_tx2, mut ice_rx2) = channel(100);

        let pc_factory_passthrough = api_factory
            .create_peer_connection_factory_passthrough()
            .unwrap();

        let mut pc = pc_factory_passthrough
            .create_peer_connection(
                config,
                ObserverSenders {
                    ice_candidate: Some(ice_tx),
                    ..Default::default()
                },
            )
            .unwrap();
        let (source, source_writer) = VideoTrackSource::create();
        let track = pc_factory_passthrough
            .create_video_track("test".into(), &source)
            .unwrap();
        pc.add_video_track(["test".into()].to_vec(), track)
            .await
            .unwrap();

        let sdp = pc.create_offer().await.unwrap();
        pc.set_local_description(sdp.copy_to_remote().unwrap())
            .await
            .unwrap();

        let pc_factory2 = api_factory2.create_peer_connection_factory().unwrap();
        let mut pc2 = pc_factory2
            .create_peer_connection(
                config2,
                ObserverSenders {
                    ice_candidate: Some(ice_tx2),
                    ..Default::default()
                },
            )
            .unwrap();
        pc2.set_remote_description(sdp.copy_to_remote().unwrap())
            .await
            .unwrap();

        let answer = pc2.create_answer().await.unwrap();
        let answer_for_remote = answer.copy_to_remote().unwrap();

        pc2.set_local_description(answer).await.unwrap();
        pc.set_remote_description(answer_for_remote).await.unwrap();

        let pc1_ice = ice_rx.recv().await.expect("Can get ice");
        let pc2_ice = ice_rx2.recv().await.expect("Can get ice");
        pc.add_ice_candidate(pc2_ice).await.unwrap();
        pc2.add_ice_candidate(pc1_ice).await.unwrap();

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
                    encoder.queue(frame).unwrap();
                    let frame = encoder.encoded_rx.recv().unwrap();
                    source_writer.push_encoded_frame(frame).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            }
        });

        let (test_done_tx, mut test_done_rx) = channel(1);
        tokio::spawn(async move {
            loop {
                let stats = pc2.get_stats().await.unwrap();
                for stat in stats.video_receiver_stats {
                    if stat.frames_decoded > 1 {
                        if cancel_push_tx.send(()).is_ok() {}
                        test_done_tx.send(1).await.unwrap();
                        break;
                    }
                }
                sleep(std::time::Duration::from_millis(1000)).await
            }
        });
        test_done_rx.recv().await.unwrap();
    }
}
