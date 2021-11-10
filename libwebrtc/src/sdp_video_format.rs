use std::collections::HashMap;

use libwebrtc_sys::ffi::ArcasSDPVideoFormatWrapper;

use crate::error::{Result, WebRTCError};

/**
 * SDP Video format helper class. Should not be directly constructed.
 */
pub struct SDPVideoFormat {
    cxx: cxx::UniquePtr<ArcasSDPVideoFormatWrapper>,
}

impl std::fmt::Debug for SDPVideoFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SDPVideoFormat")
            .field("value", &self.cxx.to_string())
            .finish()
    }
}

impl SDPVideoFormat {
    pub(crate) fn new_from_cxx(cxx: cxx::UniquePtr<ArcasSDPVideoFormatWrapper>) -> Self {
        Self { cxx }
    }

    pub(crate) fn as_ref(&self) -> Result<&ArcasSDPVideoFormatWrapper> {
        self.cxx
            .as_ref()
            .ok_or_else(|| WebRTCError::CXXUnwrapError("SDPVideoFormat".into()))
    }

    /// Get the name of this SDP video codec (H264, VP9, etc)
    pub fn get_name(&self) -> String {
        self.cxx.get_name()
    }

    /// RTP parameters for this video codec
    pub fn get_parameters(&self) -> HashMap<String, String> {
        let params = self.cxx.get_parameters();

        let mut map = HashMap::new();

        for item in params.iter() {
            map.insert(item.key.clone(), item.value.clone());
        }

        map
    }
}

impl Clone for SDPVideoFormat {
    fn clone(&self) -> Self {
        SDPVideoFormat {
            cxx: self.cxx.clone(),
        }
    }
}

impl ToString for SDPVideoFormat {
    fn to_string(&self) -> String {
        self.cxx.to_string()
    }
}
