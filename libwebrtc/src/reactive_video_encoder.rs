use std::{
    cmp::max,
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crossbeam_channel::Sender;
use cxx::UniquePtr;
use libwebrtc_sys::{
    ffi::{
        self, create_arcas_video_encoder_factory_from_builtin, ArcasEncodedImageCallback,
        ArcasVideoCodec, ArcasVideoEncoderInfo, ArcasVideoEncoderRateControlParameters,
    },
    video_encoding::{VideoEncoderFactoryImpl, VideoEncoderImpl},
    VideoEncoderProxy, VideoEncoderSelectorProxy,
};
use log::{error, info};

use crate::{
    error::{Result, WebRTCError},
    ok_or_return,
    video_codec::VideoCodecDescription,
    video_encoder_pool::VideoEncoderPoolRequest,
};

const ENCODER_THREADS_MULTIPLER: f32 = 0.25;
const MAX_ENCODER_THREADS: i32 = 32;
pub const DEFAULT_ENCODING: &str = "VP8";

pub struct ReactiveVideoEncoderFactory {
    encoder_pool_request_tx: Sender<VideoEncoderPoolRequest>,
    builtin_factory: UniquePtr<ffi::ArcasVideoEncoderFactoryWrapper>,
    // This is for the purpose of getting the encoder info only...
    encoder: UniquePtr<ffi::ArcasVideoEncoderWrapper>,
}

impl ReactiveVideoEncoderFactory {
    pub fn create(encoder_pool_request_tx: Sender<VideoEncoderPoolRequest>) -> Result<Self> {
        let builtin_factory = create_arcas_video_encoder_factory_from_builtin();
        let formats = builtin_factory.get_supported_formats();
        let format = formats
            .iter()
            .find(|value| value.get_name() == DEFAULT_ENCODING)
            .unwrap();
        let wrapper = Box::new(libwebrtc_sys::EncodedImageCallbackHandler::new(
            Box::new(move |_drop_reason| {}),
            Box::new(move |_encoded_image, _codec_info| {}),
        ));
        let encoder = builtin_factory.create_encoder(format, wrapper);
        Ok(Self {
            encoder_pool_request_tx,
            builtin_factory,
            encoder,
        })
    }
}

impl VideoEncoderFactoryImpl for ReactiveVideoEncoderFactory {
    fn get_supported_formats(&self) -> UniquePtr<cxx::CxxVector<ffi::ArcasCxxSdpVideoFormat>> {
        let formats = self.builtin_factory.get_supported_formats();
        let format = formats
            .iter()
            .find(|format| format.get_name() == DEFAULT_ENCODING)
            .unwrap();

        format.cxx_format_list()
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
        // for now we hardcode to VP9 support for simplicity
        let name = ffi::sdp_video_format_get_name(format);
        info!("query codec support = {:?}", name.to_str());
        if name == DEFAULT_ENCODING {
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
        let request_tx = self.encoder_pool_request_tx.clone();
        Box::new(VideoEncoderProxy::new(Box::new(ReactiveVideoEncoder::new(
            self.encoder.get_encoder_info(),
            request_tx,
        ))))
    }

    fn get_encoder_selector(&self) -> Option<VideoEncoderSelectorProxy> {
        None
    }
}

pub struct ReactiveVideoEncoder {
    id: String,
    codec: Option<UniquePtr<ArcasVideoCodec>>,
    number_of_cores: Option<i32>,
    max_payload_size: Option<usize>,
    sender: Sender<VideoEncoderPoolRequest>,
    callback: Option<UniquePtr<ArcasEncodedImageCallback>>,
    sent: bool,
    controller_id: Option<String>,
    info: ArcasVideoEncoderInfo,
}

impl ReactiveVideoEncoder {
    pub fn new(info: ArcasVideoEncoderInfo, sender: Sender<VideoEncoderPoolRequest>) -> Self {
        Self {
            id: nanoid::nanoid!(),
            controller_id: None,
            info,
            sender,
            codec: None,
            number_of_cores: None,
            max_payload_size: None,
            sent: false,
            callback: None,
        }
    }
}

impl VideoEncoderImpl for ReactiveVideoEncoder {
    unsafe fn init_encode(
        &mut self,
        codec_settings: *const ffi::ArcasCxxVideoCodec,
        number_of_cores: i32,
        max_payload_size: usize,
    ) -> i32 {
        let codec = libwebrtc_sys::ffi::create_arcas_video_codec_from_cxx(codec_settings);
        self.codec = Some(codec);
        self.number_of_cores = Some(max(
            number_of_cores * ENCODER_THREADS_MULTIPLER as i32,
            MAX_ENCODER_THREADS,
        ));
        self.max_payload_size = Some(max_payload_size);
        0
    }

    fn register_encode_complete_callback(
        &mut self,
        callback: UniquePtr<ArcasEncodedImageCallback>,
    ) -> i32 {
        self.callback = Some(callback);
        0
    }

    fn release(&mut self) -> i32 {
        if let Some(ref controller_id) = self.controller_id {
            match self.sender.send(VideoEncoderPoolRequest::Release {
                id: self.id.clone(),
                controller_id: controller_id.clone(),
            }) {
                Ok(_) => {}
                Err(e) => {
                    error!("Error sending release request to encoder pool: {:?}", e);
                }
            }
        };
        0
    }

    unsafe fn encode(
        &mut self,
        _frame: &ffi::ArcasCxxVideoFrame,
        _frame_types: *const cxx::CxxVector<ffi::ArcasCxxVideoFrameType>,
    ) -> i32 {
        0
    }

    fn set_rates(&mut self, rate: UniquePtr<ArcasVideoEncoderRateControlParameters>) {
        if self.sent {
            // TODO: Implement some rate updating based on individual needs of the encoders.
            // A more realistic (but generates less load) implementation could switch between
            // different encoders which have varying rates.
            return;
        }
        self.sent = true;

        let codec = ok_or_return!(self
            .codec
            .take()
            .ok_or_else(|| WebRTCError::UnexpectedError("no codec".into())));

        let codec_desc = VideoCodecDescription::create_from_codec(&codec);
        let mut hasher = DefaultHasher::new();
        codec_desc.hash(&mut hasher);
        let controller_id = hasher.finish().to_string();
        self.controller_id = Some(controller_id.clone());

        let request = VideoEncoderPoolRequest::Create {
            id: self.id.clone(),
            controller_id,
            rate,
            codec,
            number_of_cores: self.number_of_cores.take().unwrap(),
            max_payload_size: self.max_payload_size.take().unwrap(),
            callback: self.callback.take().unwrap(),
        };

        match self.sender.send(request) {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to send encoder request: {:?}", err.to_string());
            }
        }
    }

    fn on_packet_loss_rate_update(&mut self, _packet_loss_rate: f32) {}

    fn on_rtt_update(&mut self, _rtt: i64) {}

    fn on_loss_notification(&mut self, _loss_notification: ffi::ArcasVideoEncoderLossNotification) {
    }

    fn get_encoder_info(&self) -> ArcasVideoEncoderInfo {
        self.info.clone()
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::{select, unbounded};

    use tokio::{sync::mpsc::channel, time::sleep};

    use crate::{
        factory::Factory,
        peer_connection::PeerConnectionConfig,
        peer_connection_observer::ObserverSenders,
        raw_video_frame_producer::{GStreamerRawFrameProducer, RawFrameProducer},
        video_codec::VideoCodec,
        video_encoder_pool::VideoEncoderPool,
        video_track_source::VideoTrackSource,
    };

    #[tokio::test]
    async fn test_reactive_video_encoder_factory() {
        pretty_env_logger::init();
        let api_factory = Factory::new();
        let api_factory2 = Factory::new();
        let config = PeerConnectionConfig::default();
        let config2 = PeerConnectionConfig::default();
        let (_encode_pool, pool_tx) = VideoEncoderPool::create().unwrap();

        let (ice_tx, mut ice_rx) = channel(100);
        let (ice_tx2, mut ice_rx2) = channel(100);

        let pc_factory_passthrough = api_factory
            .create_peer_connection_factory_reactive(pool_tx.clone())
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

        let codec = VideoCodec::vp9_default();
        let mut raw_frames = GStreamerRawFrameProducer::default_pipeline(&codec).unwrap();
        let raw_frames_rx = raw_frames.start().unwrap();

        let (cancel_push_tx, cancel_push_rx) = unbounded::<()>();
        std::thread::spawn(move || loop {
            select! {
                recv(cancel_push_rx) -> _ => {
                    return;
                },
                recv(raw_frames_rx) -> frame_result => {
                    let frame = frame_result.unwrap();
                    source_writer.push_raw_frame(frame).unwrap();
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
