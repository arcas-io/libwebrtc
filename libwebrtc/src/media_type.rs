use libwebrtc_sys::ffi::ArcasMediaType;

pub enum MediaType {
    Audio,
    Video,
    Data,
    Unsupported,
}

impl From<ArcasMediaType> for MediaType {
    fn from(t: ArcasMediaType) -> Self {
        match t {
            ArcasMediaType::MEDIA_TYPE_AUDIO => MediaType::Audio,
            ArcasMediaType::MEDIA_TYPE_VIDEO => MediaType::Video,
            ArcasMediaType::MEDIA_TYPE_DATA => MediaType::Data,
            _ => MediaType::Unsupported,
        }
    }
}

impl From<MediaType> for ArcasMediaType {
    fn from(t: MediaType) -> Self {
        match t {
            MediaType::Audio => ArcasMediaType::MEDIA_TYPE_AUDIO,
            MediaType::Video => ArcasMediaType::MEDIA_TYPE_VIDEO,
            MediaType::Data => ArcasMediaType::MEDIA_TYPE_DATA,
            _ => ArcasMediaType::MEDIA_TYPE_UNSUPPORTED,
        }
    }
}
