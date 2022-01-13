use std::sync::Arc;

use cxx::{SharedPtr, UniquePtr};
use libwebrtc_sys::{
    ffi::{
        audio_transceiver_from_base, create_rtc_configuration, video_transceiver_from_base,
        ArcasAudioReceiverStats, ArcasAudioSenderStats, ArcasICEServer, ArcasMediaType,
        ArcasPeerConnection, ArcasPeerConnectionConfig, ArcasPeerConnectionFactory,
        ArcasRTCConfiguration, ArcasSDPSemantics, ArcasVideoReceiverStats, ArcasVideoSenderStats,
    },
    ArcasRustCreateSessionDescriptionObserver, ArcasRustRTCStatsCollectorCallback,
    ArcasRustSetSessionDescriptionObserver,
};
use parking_lot::Mutex;
use tokio::sync::mpsc::{channel, Receiver};

const STATS_BUFFER_SIZE: usize = 100;

pub type VideoSenderStats = ArcasVideoSenderStats;
pub type VideoReceiverStats = ArcasVideoReceiverStats;
pub type AudioSenderStats = ArcasAudioSenderStats;
pub type AudioReceiverStats = ArcasAudioReceiverStats;

#[derive(Debug)]
pub struct PeerConnectionStats {
    pub video_sender_stats: Vec<VideoSenderStats>,
    pub video_receiver_stats: Vec<VideoReceiverStats>,
    pub audio_sender_stats: Vec<AudioSenderStats>,
    pub audio_receiver_stats: Vec<AudioReceiverStats>,
}

impl PeerConnectionStats {
    pub fn new(
        video_sender_stats: Vec<VideoSenderStats>,
        video_receiver_stats: Vec<VideoReceiverStats>,
        audio_sender_stats: Vec<AudioSenderStats>,
        audio_receiver_stats: Vec<AudioReceiverStats>,
    ) -> Self {
        Self {
            video_sender_stats,
            video_receiver_stats,
            audio_sender_stats,
            audio_receiver_stats,
        }
    }
}

use crate::{
    audio_track::AudioTrack,
    audio_track_source::AudioTrackSource,
    error::{aracs_rtc_error_to_err, Result, WebRTCError},
    ice_candidate::ICECandidate,
    ok_or_return,
    peer_connection_observer::{ConnectionState, PeerConnectionObserver},
    rx_recv_async_or_err,
    sdp::SessionDescription,
    transceiver::{AudioTransceiver, TransceiverInit, VideoTransceiver},
    video_track::VideoTrack,
    video_track_source::VideoTrackSource,
};

pub type ICEServer = ArcasICEServer;

pub enum SDPSemantic {
    PlanB,
    UnifiedPlan,
}

impl From<ArcasSDPSemantics> for SDPSemantic {
    fn from(value: ArcasSDPSemantics) -> Self {
        match value {
            ArcasSDPSemantics::kPlanB => SDPSemantic::PlanB,
            ArcasSDPSemantics::kUnifiedPlan => SDPSemantic::UnifiedPlan,
            _ => SDPSemantic::UnifiedPlan,
        }
    }
}

impl From<SDPSemantic> for ArcasSDPSemantics {
    fn from(value: SDPSemantic) -> Self {
        match value {
            SDPSemantic::PlanB => ArcasSDPSemantics::kPlanB,
            SDPSemantic::UnifiedPlan => ArcasSDPSemantics::kUnifiedPlan,
        }
    }
}

pub struct PeerConnectionConfig {
    pub sdp_semantics: SDPSemantic,
    pub ice_servers: Vec<ICEServer>,
}

impl PeerConnectionConfig {
    pub fn new(sdp_semantics: SDPSemantic, ice_servers: Vec<ICEServer>) -> Self {
        Self {
            sdp_semantics,
            ice_servers,
        }
    }
}

impl From<PeerConnectionConfig> for UniquePtr<ArcasRTCConfiguration> {
    fn from(value: PeerConnectionConfig) -> UniquePtr<ArcasRTCConfiguration> {
        create_rtc_configuration(ArcasPeerConnectionConfig {
            sdp_semantics: value.sdp_semantics.into(),
            ice_servers: value.ice_servers,
        })
    }
}

impl Default for PeerConnectionConfig {
    fn default() -> Self {
        Self {
            sdp_semantics: SDPSemantic::UnifiedPlan,
            ice_servers: vec![ICEServer {
                urls: vec!["stun:stun.l.google.com:19302".to_string()],
                username: "".into(),
                password: "".into(),
            }],
        }
    }
}

/// NOTE: Factories are intended to be used with normal threading models and not tokio tasks.
pub struct PeerConnectionFactory {
    cxx_factory: Arc<UniquePtr<ArcasPeerConnectionFactory>>,
}

impl PeerConnectionFactory {
    pub fn new(cxx_factory: UniquePtr<ArcasPeerConnectionFactory>) -> Self {
        Self {
            cxx_factory: Arc::new(cxx_factory),
        }
    }

    pub fn create_video_track(&self, id: String, source: &VideoTrackSource) -> Result<VideoTrack> {
        let source_ref = source.cxx_ref()?;
        let track = self.cxx_factory.create_video_track(id, source_ref);
        Ok(VideoTrack::new(track))
    }

    pub fn create_audio_track(&self, id: String, source: &AudioTrackSource) -> Result<AudioTrack> {
        let source_ref = source.cxx_ref()?;
        let track = self.cxx_factory.create_audio_track(id, source_ref);
        Ok(AudioTrack::new(track))
    }

    pub fn create_peer_connection(&self, config: PeerConnectionConfig) -> Result<PeerConnection> {
        let mut observer = PeerConnectionObserver::new()?;
        let cxx_pc = unsafe {
            self.cxx_factory
                .create_peer_connection(config.into(), observer.cxx_mut_ptr()?)
        };

        Ok(PeerConnection::new(observer, cxx_pc))
    }
}

/// NOTE: Unlike the factories these peer connection objects are tokio friendly.
///
/// # Thread Safety
///
/// In C++ all calls are redirected to the right thread.  this means we can pass
/// around and share peer connection objects.
pub struct PeerConnection {
    observer: Arc<Mutex<PeerConnectionObserver>>,
    cxx_pc: SharedPtr<ArcasPeerConnection>,
}

impl<'a> PeerConnection {
    pub(crate) fn new(
        observer: PeerConnectionObserver,
        cxx_pc: SharedPtr<ArcasPeerConnection>,
    ) -> Self {
        Self {
            observer: Arc::new(Mutex::new(observer)),
            cxx_pc,
        }
    }

    pub async fn create_offer(&self) -> Result<SessionDescription> {
        let (tx, mut rx) = channel(1);
        let tx_err = tx.clone();

        self.cxx_pc
            .create_offer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
                Box::new(move |session_description| {
                    ok_or_return!(tx.blocking_send(Ok(session_description)));
                }),
                Box::new(move |err| {
                    ok_or_return!(tx_err.blocking_send(Err(WebRTCError::FailedToGenerateSDP(
                        format!("create offer: {:?}", aracs_rtc_error_to_err(err)),
                    ))));
                }),
            )));

        let cxx_sdp_result = rx_recv_async_or_err!(rx)?;
        let cxx_sdp = cxx_sdp_result?;
        Ok(SessionDescription::new_from_cxx(cxx_sdp))
    }

    pub async fn create_answer(&self) -> Result<SessionDescription> {
        let (tx, mut rx) = channel(1);
        let tx_err = tx.clone();

        self.cxx_pc
            .create_answer(Box::new(ArcasRustCreateSessionDescriptionObserver::new(
                Box::new(move |session_description| {
                    ok_or_return!(tx.blocking_send(Ok(session_description)));
                }),
                Box::new(move |err| {
                    ok_or_return!(tx_err.blocking_send(Err(WebRTCError::FailedToGenerateSDP(
                        format!("create answer: {:?}", aracs_rtc_error_to_err(err)),
                    ))));
                }),
            )));

        let cxx_sdp_result = rx_recv_async_or_err!(rx)?;
        let cxx_sdp = cxx_sdp_result?;
        Ok(SessionDescription::new_from_cxx(cxx_sdp))
    }

    pub async fn set_local_description(&self, sdp: SessionDescription) -> Result<()> {
        let cxx_sdp = sdp.take_cxx();
        let (tx, mut rx) = channel(1);
        let tx_err = tx.clone();

        self.cxx_pc.set_local_description(
            Box::new(ArcasRustSetSessionDescriptionObserver::new(
                Box::new(move || {
                    ok_or_return!(tx.blocking_send(Ok(())));
                }),
                Box::new(move |err| {
                    ok_or_return!(
                        tx_err.blocking_send(Err(WebRTCError::FailedToSetSDP(format!(
                            "set local description: {:?}",
                            aracs_rtc_error_to_err(err)
                        ),)))
                    );
                }),
            )),
            cxx_sdp,
        );

        rx_recv_async_or_err!(rx)??;
        Ok(())
    }

    pub async fn set_remote_description(&self, sdp: SessionDescription) -> Result<()> {
        let cxx_sdp = sdp.take_cxx();
        let (tx, mut rx) = channel(1);
        let tx_err = tx.clone();

        self.cxx_pc.set_remote_description(
            Box::new(ArcasRustSetSessionDescriptionObserver::new(
                Box::new(move || {
                    ok_or_return!(tx.blocking_send(Ok(())));
                }),
                Box::new(move |err| {
                    ok_or_return!(
                        tx_err.blocking_send(Err(WebRTCError::FailedToSetSDP(format!(
                            "set remote description: {:?}",
                            aracs_rtc_error_to_err(err)
                        ),)))
                    );
                }),
            )),
            cxx_sdp,
        );

        rx_recv_async_or_err!(rx)??;
        Ok(())
    }

    pub async fn add_video_transceiver(
        &self,
        init: TransceiverInit,
        mut track: VideoTrack,
    ) -> Result<VideoTransceiver> {
        let cxx_track = track.take_cxx()?;
        let cxx_init = init.take_cxx();
        let transceiver = self
            .cxx_pc
            .add_video_transceiver_with_track(cxx_track, cxx_init);

        Ok(VideoTransceiver::new(transceiver))
    }

    pub async fn add_audio_transceiver(
        &self,
        init: TransceiverInit,
        mut track: AudioTrack,
    ) -> Result<AudioTransceiver> {
        let cxx_track = track.take_cxx()?;
        let cxx_init = init.take_cxx();
        let transceiver = self
            .cxx_pc
            .add_audio_transceiver_with_track(cxx_track, cxx_init);
        Ok(AudioTransceiver::new(transceiver))
    }

    pub async fn add_video_track(
        &self,
        stream_ids: Vec<String>,
        mut track: VideoTrack,
    ) -> Result<()> {
        let cxx_track = track.take_cxx()?;
        self.cxx_pc.add_video_track(cxx_track, stream_ids);
        Ok(())
    }

    pub async fn add_audio_track(
        &self,
        stream_ids: Vec<String>,
        mut track: AudioTrack,
    ) -> Result<()> {
        let cxx_track = track.take_cxx()?;
        self.cxx_pc.add_audio_track(cxx_track, stream_ids);
        Ok(())
    }

    pub async fn get_stats(&self) -> Result<PeerConnectionStats> {
        let (tx, mut rx) = channel(STATS_BUFFER_SIZE);
        self.cxx_pc
            .get_stats(Box::new(ArcasRustRTCStatsCollectorCallback::new(Box::new(
                move |video_receiver_stats,
                      audio_receiver_stats,
                      video_sender_stats,
                      audio_sender_stats| {
                    let stats = PeerConnectionStats {
                        video_sender_stats,
                        video_receiver_stats,
                        audio_sender_stats,
                        audio_receiver_stats,
                    };
                    ok_or_return!(tx.blocking_send(stats));
                },
            ))));

        Ok(rx_recv_async_or_err!(rx)?)
    }

    pub async fn add_ice_candidate(&self, candidate: ICECandidate) -> Result<()> {
        let cxx_candidate = candidate.take_cxx();
        self.cxx_pc.add_ice_candidate(cxx_candidate);
        Ok(())
    }

    pub fn get_transceivers(&self) -> (Vec<VideoTransceiver>, Vec<AudioTransceiver>) {
        let cxx_vec = self.cxx_pc.get_transceivers();
        let (mut video, mut audio) = (vec![], vec![]);
        cxx_vec
            .into_iter()
            .for_each(|transceiver| match transceiver.media_type() {
                ArcasMediaType::MEDIA_TYPE_AUDIO => audio.push(AudioTransceiver::new(
                    audio_transceiver_from_base(transceiver),
                )),
                ArcasMediaType::MEDIA_TYPE_VIDEO => video.push(VideoTransceiver::new(
                    video_transceiver_from_base(transceiver),
                )),
                _ => {}
            });
        (video, audio)
    }

    pub fn take_connection_state_rx(&mut self) -> Result<Receiver<ConnectionState>> {
        let mut lock = self.observer.lock();
        lock.take_connection_state_rx()
    }

    pub fn take_ice_candidate_rx(&mut self) -> Result<Receiver<ICECandidate>> {
        let mut lock = self.observer.lock();
        lock.take_ice_candidate_rx()
    }

    pub fn take_video_track_rx(&mut self) -> Result<Receiver<VideoTransceiver>> {
        let mut lock = self.observer.lock();
        lock.take_video_track_rx()
    }
}

impl Drop for PeerConnection {
    fn drop(&mut self) {
        // Stop underlying tasks in libwebrtc from continuing to run on a deallocated object.
        self.cxx_pc.close();
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use tokio::{test, time::sleep};

    use super::*;
    use crate::{
        factory::{Factory, FactoryConfig},
        passthrough_video_decoder_factory::PassthroughVideoDecoderFactory,
        raw_video_frame_producer::{GStreamerRawFrameProducer, RawFrameProducer},
        reactive_video_encoder::ReactiveVideoEncoderFactory,
        video_codec::VideoCodec,
        video_encoder_pool,
    };

    #[test]
    async fn test_drops() {
        // Create some threads to run the peer connections.
        let factory1 = Factory::new();
        {
            let _ = Factory::new();
        }

        {
            let factory = Factory::new();
            let _ = factory.create_peer_connection_factory();
        }

        let pc_factory1 = factory1.create_peer_connection_factory().unwrap();

        {
            pc_factory1
                .create_peer_connection(PeerConnectionConfig::default())
                .unwrap();
        }
    }

    #[tokio::test]
    async fn test_drops_drop_factory() {
        // Create some threads to run the peer connections.
        let factory1 = Factory::new();
        {
            let _ = Factory::new();
        }

        let _pc_factory1 = factory1.create_peer_connection_factory().unwrap();
        // shouldn't panic because pc1 has a reference in C++.
        drop(factory1);
    }

    #[test]
    async fn test_peer_connection_connect() {
        // Create some threads to run the peer connections.
        let factory1 = Factory::new();
        let factory2 = Factory::new();

        let pc_factory1 = factory1.create_peer_connection_factory().unwrap();
        let pc_factory2 = factory2.create_peer_connection_factory().unwrap();

        let mut pc1 = pc_factory1
            .create_peer_connection(PeerConnectionConfig::default())
            .unwrap();

        let mut pc2 = pc_factory2
            .create_peer_connection(PeerConnectionConfig::default())
            .unwrap();

        let (source, source_write) = VideoTrackSource::create();
        let track = pc_factory1
            .create_video_track("test".into(), &source)
            .unwrap();

        let _transceiver = pc1
            .add_video_transceiver(TransceiverInit::default(), track)
            .await
            .unwrap();

        {
            let (video_transceivers, _) = pc1.get_transceivers();
            assert!(video_transceivers.len() == 1);
        }

        let offer = pc1.create_offer().await.unwrap();
        let remote_offer = offer.copy_to_remote().unwrap();
        pc1.set_local_description(offer).await.unwrap();
        pc2.set_remote_description(remote_offer).await.unwrap();
        let answer = pc2.create_answer().await.unwrap();
        let remote_answer = answer.copy_to_remote().unwrap();
        pc2.set_local_description(answer).await.unwrap();
        pc1.set_remote_description(remote_answer).await.unwrap();

        let codec = VideoCodec::vp9_default();
        let mut producer = GStreamerRawFrameProducer::default_pipeline(&codec).unwrap();
        let rx = producer.start().unwrap();

        let mut pc1_ice = pc1.take_ice_candidate_rx().unwrap();
        let mut pc2_ice = pc2.take_ice_candidate_rx().unwrap();

        let pc1_candidate = pc1_ice.recv().await.unwrap();
        let pc2_candidate = pc2_ice.recv().await.unwrap();

        pc1.add_ice_candidate(pc2_candidate).await.unwrap();
        pc2.add_ice_candidate(pc1_candidate).await.unwrap();

        // NOTE: Here we use a thread this is because the rx is blocking and it will jam up tokio
        // if we mix those apis with crossbeam.
        std::thread::spawn(move || {
            while let Ok(frame) = rx.recv() {
                source_write.push_raw_frame(frame).unwrap();
            }
        });

        let (done_tx, mut done_rx) = channel(1);
        tokio::spawn(async move {
            loop {
                let stats = pc2.get_stats().await.unwrap();
                if !stats.video_receiver_stats.is_empty() {
                    if let Some(video_receiver_stats) = stats.video_receiver_stats.get(0) {
                        if video_receiver_stats.frames_decoded > 0 {
                            done_tx.send(1).await.unwrap();
                            break;
                        }
                    }
                }
                sleep(Duration::from_millis(10)).await;
            }
        });
        done_rx.recv().await.unwrap();
    }

    #[test]
    async fn test_create_peer_connection_with_factory_config() {
        use libwebrtc_sys::video_decoding::VideoDecoderFactoryImpl;
        use libwebrtc_sys::video_encoding::VideoEncoderFactoryImpl;
        let (_, enc_tx) = video_encoder_pool::VideoEncoderPool::create().unwrap();
        let video_encoder_factory: Option<Box<dyn VideoEncoderFactoryImpl>> = Some(Box::new(
            ReactiveVideoEncoderFactory::create(enc_tx).unwrap(),
        ));
        let video_decoder_factory: Option<Box<dyn VideoDecoderFactoryImpl>> =
            Some(Box::new(PassthroughVideoDecoderFactory::new()));
        let config = FactoryConfig {
            video_encoder_factory,
            video_decoder_factory,
            audio_encoder_factory: None,
        };
        let api = Factory::new();
        let pc_factory = api.create_factory_with_config(config).unwrap();
        let _ = pc_factory
            .create_peer_connection(PeerConnectionConfig::default())
            .unwrap();
    }
}
