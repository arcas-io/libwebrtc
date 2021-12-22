use std::sync::Arc;

use crossbeam_channel::Sender;
use cxx::UniquePtr;
use libwebrtc_sys::{
    ffi::{
        create_arcas_peerconnection_factory_config, create_arcas_video_decoder_factory,
        create_arcas_video_encoder_factory, ArcasAPI,
    },
    video_decoding::VideoDecoderFactoryImpl,
    video_encoding::VideoEncoderFactoryImpl,
    VideoDecoderFactoryProxy, VideoEncoderFactoryProxy,
};

use crate::{
    error::Result, passthrough_video_encoder::PassThroughVideoEncoderFactory,
    peer_connection::PeerConnectionFactory, reactive_video_encoder::ReactiveVideoEncoderFactory,
    video_encoder_pool::VideoEncoderPoolRequest,
};

/// Each factory holds the references to the underlying threads that run
/// everything
pub struct Factory {
    cxx: Arc<UniquePtr<ArcasAPI>>,
}

/// FactoryConfig
#[derive(Default)]
pub struct FactoryConfig {
    pub video_encoder_factory: Option<Box<dyn VideoEncoderFactoryImpl>>,
    pub video_decoder_factory: Option<Box<dyn VideoDecoderFactoryImpl>>,
}

impl Factory {
    pub fn new() -> Factory {
        let cxx = libwebrtc_sys::ffi::create_arcas_api();
        Self { cxx: Arc::new(cxx) }
    }

    pub fn create_peer_connection_factory(&self) -> Result<PeerConnectionFactory> {
        let cxx_factory = self.cxx.create_factory();
        Ok(PeerConnectionFactory::new(cxx_factory))
    }

    pub fn create_peer_connection_factory_passthrough(&self) -> Result<PeerConnectionFactory> {
        let video_encoder_factory = create_arcas_video_encoder_factory(Box::new(
            VideoEncoderFactoryProxy::new(Box::new(PassThroughVideoEncoderFactory::new())),
        ));
        let cxx_factory = self
            .cxx
            .create_factory_with_arcas_video_encoder_factory(video_encoder_factory);

        Ok(PeerConnectionFactory::new(cxx_factory))
    }

    pub fn create_peer_connection_factory_reactive(
        &self,
        encoder_pool_request_tx: Sender<VideoEncoderPoolRequest>,
    ) -> Result<PeerConnectionFactory> {
        let video_encoder_factory =
            create_arcas_video_encoder_factory(Box::new(VideoEncoderFactoryProxy::new(Box::new(
                ReactiveVideoEncoderFactory::create(encoder_pool_request_tx)?,
            ))));
        let cxx_factory = self
            .cxx
            .create_factory_with_arcas_video_encoder_factory(video_encoder_factory);

        Ok(PeerConnectionFactory::new(cxx_factory))
    }

    pub fn create_factory_with_config(
        &self,
        config: FactoryConfig,
    ) -> Result<PeerConnectionFactory> {
        let encoder_factory = config
            .video_encoder_factory
            .map(|enc| {
                create_arcas_video_encoder_factory(Box::new(VideoEncoderFactoryProxy::new(enc)))
            })
            .unwrap_or(UniquePtr::null());
        let decoder_factory = config
            .video_decoder_factory
            .map(|dec| {
                create_arcas_video_decoder_factory(Box::new(VideoDecoderFactoryProxy::new(dec)))
            })
            .unwrap_or(UniquePtr::null());
        let config = create_arcas_peerconnection_factory_config(encoder_factory, decoder_factory);
        let cxx_factory = self.cxx.create_factory_with_config(config);
        Ok(PeerConnectionFactory::new(cxx_factory))
    }
}

impl Default for Factory {
    fn default() -> Self {
        Self::new()
    }
}
