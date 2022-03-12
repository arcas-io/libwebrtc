use cxx::UniquePtr;
use libwebrtc_sys::rtc_base::certificates::ffi::{
    create_arcas_key_params_ecdsa, create_arcas_key_params_rsa, create_arcas_rtc_certificate,
    create_arcas_rtc_certificate_from_pem, create_arcas_ssl_identity_with_key_params,
    ArcasKeyParams, ArcasSSLCertificate, KeyType,
};

pub struct KeyParams {
    inner: UniquePtr<ArcasKeyParams>,
}

pub struct SSLIdentity {
    inner: UniquePtr<libwebrtc_sys::rtc_base::certificates::ffi::SSLIdentity>,
}

pub struct SSLCertificate {
    pub(crate) inner: UniquePtr<ArcasSSLCertificate>,
}

impl KeyParams {
    pub fn new(key_type: KeyType) -> Self {
        if key_type == KeyType::KT_ECDSA {
            Self {
                inner: create_arcas_key_params_ecdsa(),
            }
        } else {
            Self {
                inner: create_arcas_key_params_rsa(),
            }
        }
    }
}

impl SSLIdentity {
    pub fn new(common_name: String, key_params: KeyParams) -> Self {
        Self {
            inner: create_arcas_ssl_identity_with_key_params(common_name, key_params.inner),
        }
    }
}
impl SSLCertificate {
    pub fn new(ident: SSLIdentity) -> Self {
        Self {
            inner: create_arcas_rtc_certificate(ident.inner),
        }
    }
    pub fn get_fingerprint(&self) -> String {
        self.inner.get_fingerprint()
    }
}
