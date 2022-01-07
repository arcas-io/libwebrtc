use std::{sync::Arc, thread};

use crossbeam_channel::{select, Sender};
use libwebrtc_sys::audio_encoding::{
    ffi::{
        create_audio_encoder, ArcasAudioCodecInfo, ArcasAudioCodecSpec, ArcasAudioEncoder,
        ArcasSdpAudioFormat,
    },
    AudioEncoderFactoryImpl, AudioEncoderProxy,
};

use crate::{
    audio_encoder_pool::AudioEncoderPool, encoded_audio_frame_producer::EncodedAudioFrameProducer,
};

impl SharedAudioEncoderFactory {
    pub fn new(
        frame_producer: Option<Box<dyn EncodedAudioFrameProducer>>,
        supported_formats: Vec<ArcasAudioCodecSpec>,
    ) -> Self {
        Self {
            pool: Arc::from(AudioEncoderPool::new()),
            started: false,
            supported_formats,
            frame_producer,
            cancel_tx: None,
        }
    }
}
struct SharedAudioEncoderFactory {
    pool: Arc<AudioEncoderPool>,
    started: bool,
    supported_formats: Vec<ArcasAudioCodecSpec>,
    frame_producer: Option<Box<dyn EncodedAudioFrameProducer>>,
    cancel_tx: Option<Sender<()>>,
}

impl AudioEncoderFactoryImpl for SharedAudioEncoderFactory {
    unsafe fn get_supported_formats(&self) -> Vec<ArcasAudioCodecSpec> {
        self.supported_formats.clone()
    }

    unsafe fn query_audio_encoder(
        &self,
        format: &ArcasSdpAudioFormat,
    ) -> Option<ArcasAudioCodecInfo> {
        self.supported_formats
            .iter()
            .find(|spec| spec.format.name == format.name)
            .map(|spec| spec.info.clone())
    }

    unsafe fn make_audio_encoder(
        &mut self,
        payload_type: i32,
        format: &ArcasSdpAudioFormat,
    ) -> cxx::UniquePtr<ArcasAudioEncoder> {
        let result = self
            .supported_formats
            .iter()
            .find(|spec| {
                spec.format.name == format.name && spec.format.num_channels == format.num_channels
            })
            .map(|spec| {
                let enc = Box::from(self.pool.make_shared_audio_encoder(
                    payload_type,
                    spec.info.sample_rate,
                    spec.format.num_channels,
                    spec.info.default_bitrate_bps,
                ));
                let proxy = AudioEncoderProxy::new(enc);
                create_audio_encoder(Box::from(proxy))
            })
            .unwrap_or_else(cxx::UniquePtr::null);

        if !result.is_null() && !self.started {
            self.started = true;
            let encoded_rx = match self
                .frame_producer
                .as_mut()
                .map(|producer| producer.start().ok())
                .and_then(|x| x)
            {
                Some(x) => x,
                None => return result,
            };
            let (cancel_tx, cancel_rx) = crossbeam_channel::bounded::<()>(1);
            self.cancel_tx = Some(cancel_tx);
            let pool = self.pool.clone();
            thread::spawn(move || loop {
                select! {
                    recv(encoded_rx) -> encoded_buf_res => {
                        if let Ok(buf) = encoded_buf_res {
                            pool.push_encoded_frame(buf);
                        }
                    },
                    recv(cancel_rx) -> _ => {
                        break;
                    },
                }
            });
        }
        result
    }
}

impl Drop for SharedAudioEncoderFactory {
    fn drop(&mut self) {
        self.cancel_tx.as_mut().map(|tx| {
            let _ = tx.send(());
            Some(())
        });
    }
}

#[cfg(test)]
mod tests {
    use libwebrtc_sys::{
        audio_encoding::ffi::{ArcasAudioCodecSpec, ArcasSdpAudioFormat},
        ffi::set_arcas_log_level,
        shared_bridge::ffi::LoggingSeverity,
    };

    use crate::{
        audio_track_source::AudioTrackSource,
        encoded_audio_frame_producer::GStreamerOpusAudioFrameProducer,
        factory::{Factory, FactoryConfig},
        peer_connection::{PeerConnectionConfig, SDPSemantic},
        transceiver::{TransceiverDirection, TransceiverInit},
    };

    use super::SharedAudioEncoderFactory;
    use tokio::spawn;

    #[tokio::test]
    async fn test_create_opus_encoder_factory() {
        // set_arcas_log_level(LoggingSeverity::LS_INFO);
        let arcas_factory = Factory::new();
        let opus_enc_factory = Box::from(SharedAudioEncoderFactory::new(
            Some(Box::from(GStreamerOpusAudioFrameProducer::new(2, 8000, 0))),
            vec![ArcasAudioCodecSpec {
                format: ArcasSdpAudioFormat {
                    name: "opus".to_owned(),
                    num_channels: 2,
                    clockrate_hz: 48000,
                    parameters: vec![],
                },
                info: libwebrtc_sys::audio_encoding::ffi::ArcasAudioCodecInfo {
                    sample_rate: 8000,
                    num_channels: 1,
                    default_bitrate_bps: 64000,
                    min_bitrate_bps: 6000,
                    max_bitrate_bps: 510000,
                    allow_comfort_noise: false,
                    supports_network_adaptation: false,
                },
            }],
        ));
        let pc_factory = arcas_factory
            .create_factory_with_config(FactoryConfig {
                video_encoder_factory: None,
                video_decoder_factory: None,
                /* audio_encoder_factory: Some(opus_enc_factory), */
                audio_encoder_factory: None,
            })
            .unwrap();
        let recvr_factory = arcas_factory
            .create_factory_with_config(FactoryConfig {
                video_encoder_factory: None,
                video_decoder_factory: None,
                audio_encoder_factory: None,
            })
            .unwrap();

        let source = AudioTrackSource::new(1, 8000);
        let source_writer = source.clone();
        spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(10));
            interval.tick().await;
            loop {
                source_writer.push_10ms_zeroed_data();
                interval.tick().await;
            }
        });

        let mut pc = pc_factory
            .create_peer_connection(PeerConnectionConfig {
                sdp_semantics: SDPSemantic::UnifiedPlan,
                ice_servers: vec![],
            })
            .unwrap();
        {
            let audio_track = pc_factory
                .create_audio_track("audio".to_owned(), &source)
                .unwrap();
            pc.add_audio_transceiver(
                TransceiverInit::new(vec!["0".to_owned()], TransceiverDirection::SendOnly),
                audio_track,
            )
            .await
            .unwrap();
        }

        let mut recvr = recvr_factory
            .create_peer_connection(PeerConnectionConfig {
                sdp_semantics: SDPSemantic::UnifiedPlan,
                ice_servers: vec![],
            })
            .unwrap();

        let offer = pc.create_offer().await.unwrap();
        let remote_offer = offer.copy_to_remote().unwrap();
        pc.set_local_description(offer).await.unwrap();
        recvr.set_remote_description(remote_offer).await.unwrap();
        let answer = recvr.create_answer().await.unwrap();
        let remote_answer = answer.copy_to_remote().unwrap();
        recvr.set_local_description(answer).await.unwrap();
        pc.set_remote_description(remote_answer).await.unwrap();

        let mut pc_ice = pc.take_ice_candidate_rx().unwrap();
        let mut recvr_ice = recvr.take_ice_candidate_rx().unwrap();

        let pc_cand = pc_ice.recv().await.unwrap();
        let recvr_cand = recvr_ice.recv().await.unwrap();
        pc.add_ice_candidate(recvr_cand).await.unwrap();
        recvr.add_ice_candidate(pc_cand).await.unwrap();

        let (done_tx, mut done_rx) = tokio::sync::mpsc::channel(1);
        tokio::spawn(async move {
            loop {
                let stats = pc.get_stats().await.unwrap();
                // println!("{:?}", stats.audio_sender_stats);
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        });

        tokio::spawn(async move {
            loop {
                let stats = recvr.get_stats().await.unwrap();
                if !stats.audio_receiver_stats.is_empty() {
                    if let Some(audio_receiver_stats) = stats.audio_receiver_stats.get(0) {
                        println!("{:?}", audio_receiver_stats);
                        if audio_receiver_stats.frames_decoded > 0 {
                            done_tx.send(1).await.unwrap();
                            break;
                        }
                    }
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
            }
        });
        done_rx.recv().await.unwrap();
    }
}
