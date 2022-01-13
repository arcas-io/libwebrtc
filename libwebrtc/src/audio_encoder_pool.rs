use bytes::Bytes;
use crossbeam_channel::{SendTimeoutError, Sender};
use dashmap::DashMap;
use libwebrtc_sys::audio_encoding::ffi::{
    ArcasAudioCodecInfo, ArcasAudioCodecSpec, ArcasSdpAudioFormat,
};
use std::sync::atomic::AtomicU64;

use crate::shared_audio_encoder::SharedAudioEncoder;

#[derive(Debug)]
pub struct AudioEncoderPool {
    next_id: AtomicU64,
    senders: DashMap<u64, Sender<Bytes>>,
}

impl AudioEncoderPool {
    pub fn new() -> Self {
        Self {
            next_id: AtomicU64::new(0),
            senders: DashMap::new(),
        }
    }

    pub fn push_encoded_frame(&self, data: Bytes) {
        let mut remove = vec![];
        self.senders.iter_mut().for_each(|mut sender| {
            match sender
                .value_mut()
                .send_timeout(data.clone(), std::time::Duration::from_millis(5))
            {
                Ok(()) => {}
                Err(SendTimeoutError::Disconnected(_)) => remove.push(*sender.key()),
                Err(SendTimeoutError::Timeout(_)) => {}
            }
        });
        remove.iter().for_each(|id| {
            let _ = self.senders.remove(id);
        })
    }

    pub fn make_shared_audio_encoder(
        &self,
        payload_type: i32,
        sample_rate_hz: i32,
        num_channels: usize,
        target_bitrate: i32,
    ) -> SharedAudioEncoder {
        let (tx, rx) = crossbeam_channel::bounded(10);
        let id = self
            .next_id
            .fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        self.senders.insert(id, tx);
        SharedAudioEncoder::new(
            id,
            payload_type,
            sample_rate_hz,
            num_channels,
            target_bitrate,
            rx,
        )
    }
}
