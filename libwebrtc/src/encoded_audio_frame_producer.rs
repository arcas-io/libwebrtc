use std::thread;

use crate::error::Result;
use bytes::Bytes;
use crossbeam_channel::{select, Receiver, Sender};

const OPUS_ENC_PIPELINE: &str = "\
    audiotestsrc wave={} ! \
    audio/x-raw,format=S16LE,channels={},rate={} ! \
    opusenc frame-size=10 \
";

pub struct GStreamerOpusAudioFrameProducer {
    channels: usize,
    sample_rate_hz: i32,
    gstreamer_waveform: usize,
    cancel_tx: Option<Sender<()>>,
}

pub trait EncodedAudioFrameProducer {
    fn start(&mut self) -> Result<Receiver<Bytes>>;
    fn cancel(&mut self);
}

impl GStreamerOpusAudioFrameProducer {
    pub fn new(channels: usize, sample_rate_hz: i32, gstreamer_waveform: usize) -> Self {
        Self {
            channels,
            sample_rate_hz,
            gstreamer_waveform,
            cancel_tx: None,
        }
    }

    pub fn start(&mut self) -> Result<Receiver<Bytes>> {
        let (tx, rx) = crossbeam_channel::unbounded::<Bytes>();
        let encoded_rx = media_pipeline::create_and_start_appsink_pipeline(OPUS_ENC_PIPELINE)?;
        let (cancel_tx, cancel_rx) = crossbeam_channel::bounded::<()>(0);
        self.cancel_tx = Some(cancel_tx);
        thread::spawn(move || loop {
            select! {
                recv(encoded_rx) -> result => {
                    if let Ok(res) = result {
                        let _ = tx.try_send(res.freeze());
                    };
                }

                recv(cancel_rx) -> _cancel => {
                    break;
                }
            }
        });
        Ok(rx)
    }

    pub fn cancel(&mut self) {
        self.cancel_tx.iter_mut().for_each(|tx| {
            let _ = tx.send(());
        });
    }
}
