
#include "libwebrtc-sys/include/rtc_base/certificates.h"
#include "libwebrtc-sys/src/rtc_base/certificates.rs.h"

std::unique_ptr<ArcasKeyParams> create_arcas_key_params_rsa()
{
    return std::make_unique<ArcasKeyParams>(rtc::KeyParams::RSA());
}

std::unique_ptr<ArcasKeyParams> create_arcas_key_params_ecdsa()
{
    return std::make_unique<ArcasKeyParams>(rtc::KeyParams::ECDSA());
}

std::unique_ptr<rtc::SSLIdentity>
create_arcas_ssl_identity_with_key_params(rust::String common_name,
                                          std::unique_ptr<ArcasKeyParams> key_params)
{
    return rtc::SSLIdentity::Create(common_name.c_str(), key_params->get_params());
}

std::unique_ptr<rtc::SSLIdentity> create_arcas_ssl_identity_with_key_type(rust::String common_name,
                                                                          rtc::KeyType key_type)
{
    return rtc::SSLIdentity::Create(common_name.c_str(), key_type);
}

std::unique_ptr<ArcasSSLCertificate>
create_arcas_rtc_certificate(std::unique_ptr<rtc::SSLIdentity> identity)
{
    return std::make_unique<ArcasSSLCertificate>(rtc::RTCCertificate::Create(std::move(identity)));
}

std::unique_ptr<ArcasSSLCertificate> create_arcas_rtc_certificate_from_pem(rust::String private_key,
                                                                           rust::String certificate)
{
    rtc::RTCCertificatePEM pem(private_key.c_str(), certificate.c_str());
    return std::make_unique<ArcasSSLCertificate>(rtc::RTCCertificate::FromPEM(pem));
}

ArcasRTCCertificatePEM ArcasSSLCertificate::to_pem() const
{
    auto pem = _certificate->ToPEM();

    return ArcasRTCCertificatePEM{rust::String(pem.private_key().c_str()),
                                  rust::String(pem.certificate().c_str())};
}
