use bytes::Bytes;
use crossbeam_channel::Receiver;
use libwebrtc_sys::audio_encoding::ffi::{ArcasAudioEncodedInfoLeaf, ArcasCxxAudioCodecType};
use libwebrtc_sys::audio_encoding::AudioEncoderImpl;

pub struct SharedAudioEncoder {
    _id: u64,
    _payload_type: i32,
    _sample_rate_hz: i32,
    _num_channels: usize,
    _max_10ms_frames_in_a_packet: usize,
    _target_bitrate: i32,
    rx: Receiver<Bytes>,
}

impl SharedAudioEncoder {
    pub fn new(
        id: u64,
        payload_type: i32,
        sample_rate_hz: i32,
        num_channels: usize,
        target_bitrate: i32,
        rx: Receiver<Bytes>,
    ) -> Self {
        Self {
            _id: id,
            _payload_type: payload_type,
            _sample_rate_hz: sample_rate_hz,
            _num_channels: num_channels,
            _target_bitrate: target_bitrate,
            _max_10ms_frames_in_a_packet: 1,
            rx,
        }
    }
}

impl AudioEncoderImpl for SharedAudioEncoder {
    unsafe fn encode_impl(
        &mut self,
        rtp_timestamp: u32,
        _audio_data: &[i16],
        mut encoded: cxx::UniquePtr<libwebrtc_sys::audio_encoding::ffi::BufferUint8>,
    ) -> ArcasAudioEncodedInfoLeaf {
        let result_buf = match encoded.as_mut() {
            Some(x) => x,
            None => {
                return ArcasAudioEncodedInfoLeaf {
                    encoded_bytes: 0,
                    encoded_timestamp: rtp_timestamp,
                    encoder_type: ArcasCxxAudioCodecType::kOpus,
                    payload_type: self._payload_type,
                    send_even_if_empty: false,
                    speech: true,
                }
            }
        };
        match self.rx.recv_timeout(std::time::Duration::from_millis(100)) {
            Ok(buf) => {
                result_buf.append_data(&buf);
                ArcasAudioEncodedInfoLeaf {
                    encoded_bytes: buf.len(),
                    encoded_timestamp: rtp_timestamp,
                    encoder_type: ArcasCxxAudioCodecType::kOpus,
                    payload_type: self._payload_type,
                    send_even_if_empty: false,
                    speech: true,
                }
            }
            Err(_) => ArcasAudioEncodedInfoLeaf {
                encoded_bytes: 0,
                encoded_timestamp: rtp_timestamp,
                encoder_type: ArcasCxxAudioCodecType::kOpus,
                payload_type: self._payload_type,
                send_even_if_empty: false,
                speech: true,
            },
        }
    }

    unsafe fn sample_rate_hz(&self) -> i32 {
        self._sample_rate_hz
    }

    unsafe fn num_channels(&self) -> usize {
        self._num_channels
    }

    unsafe fn num_10ms_frames_in_next_packet(&self) -> usize {
        1
    }

    unsafe fn max_10ms_frames_in_a_packet(&self) -> usize {
        self._max_10ms_frames_in_a_packet
    }

    unsafe fn get_target_bitrate(&self) -> i32 {
        self._target_bitrate
    }

    unsafe fn reset(&self) {}
}
