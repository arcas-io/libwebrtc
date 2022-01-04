use std::{
    borrow::BorrowMut,
    sync::{atomic::AtomicBool, Arc},
    thread,
};

use bytes::Bytes;
use crossbeam_channel::{select, Receiver, Sender};
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
