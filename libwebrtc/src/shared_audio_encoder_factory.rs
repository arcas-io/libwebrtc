use std::sync::{atomic::AtomicBool, Arc};

use libwebrtc_sys::audio_encoding::{
    ffi::{
        create_audio_encoder, ArcasAudioCodecInfo, ArcasAudioCodecSpec, ArcasAudioEncoder,
        ArcasSdpAudioFormat,
    },
    AudioEncoderFactoryImpl, AudioEncoderProxy,
};

use crate::audio_encoder_pool::AudioEncoderPool;

struct SharedAudioEncoderFactory {
    pool: Arc<AudioEncoderPool>,
    started: AtomicBool,
    supported_formats: Vec<ArcasAudioCodecSpec>,
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
        self.supported_formats
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
            .unwrap_or_else(cxx::UniquePtr::null)
    }
}
