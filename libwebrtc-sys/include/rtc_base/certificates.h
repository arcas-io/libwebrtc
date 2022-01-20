#pragma once
#include "rtc_base/rtc_certificate.h"
#include "rtc_base/ssl_identity.h"
#include "rtc_base/ssl_stream_adapter.h"
#include "rust/cxx.h"

struct ArcasRTCCertificatePEM;

class ArcasKeyParams
{
private:
    rtc::KeyParams _params;

public:
    ArcasKeyParams(rtc::KeyParams params)
    : _params(params)
    {
    }

    rtc::KeyParams get_params() const
    {
        return _params;
    }
};

class ArcasSSLCertificate
{
private:
    rtc::scoped_refptr<rtc::RTCCertificate> _certificate;

public:
    ArcasSSLCertificate(rtc::scoped_refptr<rtc::RTCCertificate> certificate)
    : _certificate(certificate)
    {
    }

    rtc::scoped_refptr<rtc::RTCCertificate> get_certificate() const
    {
        return _certificate;
    }

    ArcasRTCCertificatePEM to_pem() const;
};

std::unique_ptr<ArcasKeyParams> create_arcas_key_params_rsa();
std::unique_ptr<ArcasKeyParams> create_arcas_key_params_ecdsa();
std::unique_ptr<rtc::SSLIdentity>
create_arcas_ssl_identity_with_key_params(rust::String common_name,
                                          std::unique_ptr<ArcasKeyParams> key_params);
std::unique_ptr<rtc::SSLIdentity> create_arcas_ssl_identity_with_key_type(rust::String common_name,
                                                                          rtc::KeyType key_type);

std::unique_ptr<ArcasSSLCertificate>
create_arcas_rtc_certificate(std::unique_ptr<rtc::SSLIdentity> identity);

std::unique_ptr<ArcasSSLCertificate>
create_arcas_rtc_certificate_from_pem(rust::String private_key, rust::String certificate);
