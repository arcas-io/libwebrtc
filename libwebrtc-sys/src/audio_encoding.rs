use std::slice::from_raw_parts;

use cxx::UniquePtr;

use self::ffi::{ArcasAudioCodecInfo, ArcasSdpAudioFormat};

#[cxx::bridge]
pub mod ffi {

    // include!("include/rtc_buffer.h");
    // type BufferUint8 = crate::rtc_buffer::ffi::BufferUint8;
    #[derive(Debug, Clone)]
    #[repr(u32)]
    enum ArcasCxxAudioCodecType {
        kOther = 0,
        kOpus = 1,
        kIsac = 2,
        kPcmA = 3,
        kPcmU = 4,
        kG722 = 5,
        kIlbc = 6,
        kMaxLoggedAudioCodecTypes,
    }

    #[derive(Debug, Clone)]
    struct ArcasAudioEncodedInfoLeaf {
        pub encoded_bytes: usize,
        pub encoded_timestamp: u32,
        pub payload_type: i32,
        pub send_even_if_empty: bool,
        pub speech: bool,
        pub encoder_type: ArcasCxxAudioCodecType,
    }

    #[derive(Debug, Default, Clone)]
    struct ArcasAudioCodecInfo {
        pub sample_rate: i32,
        pub num_channels: usize,
        pub default_bitrate_bps: i32,
        pub min_bitrate_bps: i32,
        pub max_bitrate_bps: i32,
        pub allow_comfort_noise: bool,
        pub supports_network_adaptation: bool,
    }

    #[derive(Debug, Clone)]
    struct ArcasSdpAudioFormat {
        pub name: String,
        pub clockrate_hz: i32,
        pub num_channels: usize,
        pub parameters: Vec<String>,
    }

    #[derive(Debug, Clone)]
    struct ArcasAudioCodecSpec {
        pub format: ArcasSdpAudioFormat,
        pub info: ArcasAudioCodecInfo,
    }

    unsafe extern "C++" {
        include!("include/audio_encoding.h");
        include!("include/rtc_buffer.h");
        type BufferUint8 = crate::rtc_buffer::ffi::BufferUint8;
        type ArcasCxxAudioCodecType;
        type ArcasAudioEncoder;
        type ArcasRustAudioEncoderFactory;

        fn create_audio_encoder(proxy: Box<AudioEncoderProxy>) -> UniquePtr<ArcasAudioEncoder>;
    }

    extern "Rust" {
        #[rust_name = "AudioEncoderProxy"]
        type ArcasRustAudioEncoder;

        unsafe fn encode_impl(
            self: &mut AudioEncoderProxy,
            rtp_timestamp: u32,
            audio_data: *const i16,
            audio_data_size: usize,
            encoded: UniquePtr<BufferUint8>,
        ) -> ArcasAudioEncodedInfoLeaf;

        unsafe fn sample_rate_hz(self: &AudioEncoderProxy) -> i32;

        unsafe fn num_channels(self: &AudioEncoderProxy) -> usize;

        unsafe fn num_10ms_frames_in_next_packet(self: &AudioEncoderProxy) -> usize;

        unsafe fn max_10ms_frames_in_a_packet(self: &AudioEncoderProxy) -> usize;

        unsafe fn get_target_bitrate(self: &AudioEncoderProxy) -> i32;

        unsafe fn reset(self: &mut AudioEncoderProxy);

    }

    extern "Rust" {
        #[rust_name = "AudioEncoderFactoryProxy"]
        type ArcasRustAudioEncoderFactory;

        unsafe fn get_supported_formats(
            self: &AudioEncoderFactoryProxy,
        ) -> Vec<ArcasAudioCodecSpec>;
        unsafe fn query_audio_encoder(
            self: &AudioEncoderFactoryProxy,
            format: &ArcasSdpAudioFormat,
        ) -> ArcasAudioCodecInfo;
        unsafe fn make_audio_encoder(
            self: &mut AudioEncoderFactoryProxy,
            payload_type: i32,
            format: &ArcasSdpAudioFormat,
        ) -> UniquePtr<ArcasAudioEncoder>;

    }

    unsafe extern "C++" {
        include!("include/peerconnection_factory_config.h");
        type ArcasPeerConnectionFactoryConfig = crate::peerconnection_factory_config::ffi::ArcasPeerConnectionFactoryConfig;

        fn set_audio_encoder_factory(
            self: Pin<&mut ArcasPeerConnectionFactoryConfig>,
            factory: Box<AudioEncoderFactoryProxy>,
        );
    }
}

pub trait AudioEncoderImpl {
    unsafe fn encode_impl(
        &mut self,
        rtp_timestamp: u32,
        _audio_data: &[i16],
        encoded: cxx::UniquePtr<ffi::BufferUint8>,
    ) -> ffi::ArcasAudioEncodedInfoLeaf;

    unsafe fn sample_rate_hz(&self) -> i32;

    unsafe fn num_channels(&self) -> usize;

    unsafe fn num_10ms_frames_in_next_packet(&self) -> usize;

    unsafe fn max_10ms_frames_in_a_packet(&self) -> usize;

    unsafe fn get_target_bitrate(&self) -> i32;

    unsafe fn reset(&self);
}
pub struct AudioEncoderProxy {
    encoder: Box<dyn AudioEncoderImpl>,
}

impl AudioEncoderProxy {
    pub fn new(encoder: Box<dyn AudioEncoderImpl>) -> Self {
        Self { encoder }
    }

    unsafe fn encode_impl(
        &mut self,
        rtp_timestamp: u32,
        audio_data: *const i16,
        audio_data_size: usize,
        encoded: cxx::UniquePtr<ffi::BufferUint8>,
    ) -> ffi::ArcasAudioEncodedInfoLeaf {
        let audio_data = from_raw_parts(audio_data, audio_data_size);
        self.encoder.encode_impl(rtp_timestamp, audio_data, encoded)
    }

    unsafe fn sample_rate_hz(&self) -> i32 {
        self.encoder.sample_rate_hz()
    }

    unsafe fn num_channels(&self) -> usize {
        self.encoder.num_channels()
    }

    unsafe fn num_10ms_frames_in_next_packet(&self) -> usize {
        self.encoder.num_10ms_frames_in_next_packet()
    }

    unsafe fn max_10ms_frames_in_a_packet(&self) -> usize {
        self.encoder.max_10ms_frames_in_a_packet()
    }

    unsafe fn get_target_bitrate(&self) -> i32 {
        self.encoder.get_target_bitrate()
    }

    unsafe fn reset(&mut self) {
        self.encoder.reset()
    }
}

pub trait AudioEncoderFactoryImpl {
    unsafe fn get_supported_formats(&self) -> Vec<ffi::ArcasAudioCodecSpec>;
    unsafe fn query_audio_encoder(
        &self,
        format: &ArcasSdpAudioFormat,
    ) -> Option<ffi::ArcasAudioCodecInfo>;
    unsafe fn make_audio_encoder(
        &mut self,
        payload_type: i32,
        format: &ArcasSdpAudioFormat,
    ) -> UniquePtr<ffi::ArcasAudioEncoder>;
}

pub struct AudioEncoderFactoryProxy {
    api: Box<dyn AudioEncoderFactoryImpl>,
}

impl AudioEncoderFactoryProxy {
    pub fn new(api: Box<dyn AudioEncoderFactoryImpl>) -> Self {
        Self { api }
    }

    pub unsafe fn get_supported_formats(&self) -> Vec<ffi::ArcasAudioCodecSpec> {
        self.api.get_supported_formats()
    }

    pub unsafe fn query_audio_encoder(&self, format: &ArcasSdpAudioFormat) -> ArcasAudioCodecInfo {
        self.api
            .query_audio_encoder(format)
            .unwrap_or(ArcasAudioCodecInfo::default())
    }

    pub unsafe fn make_audio_encoder(
        &mut self,
        payload_type: i32,
        format: &ArcasSdpAudioFormat,
    ) -> UniquePtr<ffi::ArcasAudioEncoder> {
        self.api.make_audio_encoder(payload_type, format)
    }
}
