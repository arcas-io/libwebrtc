#[allow(unreachable_patterns)]
#[cxx::bridge]
pub mod ffi {

    #[derive(Debug)]
    #[namespace = "rtc"]
    #[repr(u32)]
    enum KeyType {
        KT_RSA,
        KT_ECDSA,
        KT_LAST,
        KT_DEFAULT = 1,
    }

    #[derive(Debug)]
    struct ArcasRTCCertificatePEM {
        private_key: String,
        certificate: String,
    }

    #[derive(Debug)]
    #[namespace = "rtc"]
    #[repr(u32)]
    enum SSLRole {
        SSL_CLIENT,
        SSL_SERVER,
    }

    #[derive(Debug)]
    #[namespace = "rtc"]
    #[repr(u32)]
    enum SSLMode {
        SSL_MODE_TLS,
        SSL_MODE_DTLS,
    }

    unsafe extern "C++" {
        include!("include/rtc_base/certificates.h");

        #[namespace = "rtc"]
        type SSLIdentity;
        #[namespace = "rtc"]
        type KeyType;
        #[namespace = "rtc"]
        type RTCCertificate;
        type ArcasKeyParams;
        type ArcasSSLCertificate;
        #[namespace = "rtc"]
        type SSLMode;
        #[namespace = "rtc"]
        type SSLRole;

        fn create_arcas_key_params_rsa() -> UniquePtr<ArcasKeyParams>;
        fn create_arcas_key_params_ecdsa() -> UniquePtr<ArcasKeyParams>;
        fn create_arcas_ssl_identity_with_key_params(
            common_name: String,
            key_params: UniquePtr<ArcasKeyParams>,
        ) -> UniquePtr<SSLIdentity>;
        fn create_arcas_ssl_identity_with_key_type(
            common_name: String,
            key_type: KeyType,
        ) -> UniquePtr<SSLIdentity>;

        fn create_arcas_rtc_certificate(
            identity: UniquePtr<SSLIdentity>,
        ) -> UniquePtr<ArcasSSLCertificate>;

        fn create_arcas_rtc_certificate_from_pem(
            private_key: String,
            certificate: String,
        ) -> UniquePtr<ArcasSSLCertificate>;

        fn get_fingerprint(self: &ArcasSSLCertificate) -> String;

        // ArcasSSLCertificate
        fn to_pem(self: &ArcasSSLCertificate) -> ArcasRTCCertificatePEM;
        fn get_fingerprint_data(self: &ArcasSSLCertificate) -> Vec<u8>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certificate_create() {
        let key_params = ffi::create_arcas_key_params_ecdsa();
        let identity =
            ffi::create_arcas_ssl_identity_with_key_params("arcas".to_string(), key_params);
        let cert = ffi::create_arcas_rtc_certificate(identity);
        let pem = cert.to_pem();
        assert!(!pem.private_key.is_empty());
        assert!(!pem.certificate.is_empty());
    }
}
