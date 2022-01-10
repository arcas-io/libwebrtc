use std::thread;

use crate::error::Result;
use bytes::Bytes;
use crossbeam_channel::{select, Receiver, Sender};

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
}

impl EncodedAudioFrameProducer for GStreamerOpusAudioFrameProducer {
    fn start(&mut self) -> Result<Receiver<Bytes>> {
        let (tx, rx) = crossbeam_channel::unbounded::<Bytes>();
        let encoded_rx = media_pipeline::create_and_start_appsink_pipeline(
            & format!(
                "audiotestsrc wave={} ! audio/x-raw,format=S16LE,channels={},rate={},is-live=true ! opusenc frame-size=10",
                self.gstreamer_waveform,
                self.channels,
                self.sample_rate_hz,
            ),
        )?;
        let (cancel_tx, cancel_rx) = crossbeam_channel::bounded::<()>(1);
        self.cancel_tx = Some(cancel_tx);
        thread::spawn(move || loop {
            select! {
                recv(encoded_rx) -> result => {
                    if let Ok(res) = result {
                        let _ = tx.try_send(res.freeze());
                    }
                }

                recv(cancel_rx) -> _cancel => {
                    break;
                }
            }
        });
        Ok(rx)
    }

    fn cancel(&mut self) {
        if let Some(tx) = self.cancel_tx.as_mut() {
            let _ = tx.send(());
        }
    }
}

impl Drop for GStreamerOpusAudioFrameProducer {
    fn drop(&mut self) {
        self.cancel();
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use crate::encoded_audio_frame_producer::EncodedAudioFrameProducer;

    use super::GStreamerOpusAudioFrameProducer;
    #[test]
    fn test_create_encoded_audio_frame_producer() {
        let mut producer = GStreamerOpusAudioFrameProducer::new(2, 8000, 0);
        let rx = producer.start().unwrap();
        // few of the initial samples seem to be arriving early,
        // but subsequent samples arrive in 10ms intervals
        let _ = rx.recv_timeout(Duration::from_millis(20)).unwrap();
        let _ = rx.recv_timeout(Duration::from_millis(20)).unwrap();
        let _ = rx.recv_timeout(Duration::from_millis(20)).unwrap();
        let start = Instant::now();
        let _ = rx.recv_timeout(Duration::from_millis(20)).unwrap();
        let diff = Instant::now().duration_since(start);
        assert!(diff.gt(&Duration::from_millis(8)));
        assert!(diff.lt(&Duration::from_millis(12)));
    }
}
