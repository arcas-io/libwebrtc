use crate::certificate::SSLCertificate;
use crate::error::Result;
use crate::ice_candidate::P2pIceConfig;
use crate::rtc_base::{RTCNetworkManager, RTCThread};
use crate::sdp::{SDPType, SessionDescription};
use crate::transport::IceTransportFactory;
use cxx::UniquePtr;
use libwebrtc_sys::async_dns_resolver_factory::ffi::create_arcas_cxx_async_dns_resolver_factory;
use libwebrtc_sys::candidate::ffi::CandidateComponent;
use libwebrtc_sys::error::ffi::ArcasRTCError;
use libwebrtc_sys::ffi::create_arcas_session_description;
use libwebrtc_sys::ice_transport::ffi::create_arcas_cxx_ice_transport_factory;
use libwebrtc_sys::pc::jsep_api::ffi::to_cxx;
use libwebrtc_sys::pc::jsep_api::ffi::ArcasCxxSSLHandshakeError;
use libwebrtc_sys::pc::jsep_api::ffi::DataChannelTransportInterface;
use libwebrtc_sys::pc::jsep_api::ffi::RtpTransportInternal;
use libwebrtc_sys::pc::jsep_api::ffi::SdpType;
use libwebrtc_sys::pc::jsep_api::ffi::{
    create_arcas_cxx_port_allocator, create_arcas_jsep_transport_controller,
    create_arcas_jsep_transport_controller_config, create_buffer_with_data, get_transport_name,
    init_port_alloc, is_writable, send_rtp_packet, set_rtp_params, ArcasCandidateWrapper,
    ArcasJsepTransportController, ArcasJsepTransportControllerConfig, ArcasSSLCertificate,
    CopyOnWriteBuffer, PortAllocator,
};
use libwebrtc_sys::pc::jsep_api::{ArcasRustDTLSHandshakeErrorHandler, OnTransportChanged};
use libwebrtc_sys::pc::jsep_api::{
    ArcasRustJsepRTCPHandler, ArcasRustJsepTransportControllerObserver,
};
use libwebrtc_sys::rtc_base::certificates::ffi::create_arcas_key_params_rsa;
use libwebrtc_sys::rtc_base::certificates::ffi::create_arcas_rtc_certificate;
use libwebrtc_sys::rtc_base::certificates::ffi::create_arcas_ssl_identity_with_key_params;
use libwebrtc_sys::rtc_base::certificates::ffi::create_arcas_ssl_identity_with_key_type;
use libwebrtc_sys::rtc_base::certificates::ffi::KeyType;
use std::borrow::Borrow;
use std::ops::Deref;

pub struct TransportController {
    inner: UniquePtr<ArcasJsepTransportController>,
    // port_alloc: UniquePtr<PortAllocator>,
}
pub struct TransportControllerConfig {
    inner: UniquePtr<ArcasJsepTransportControllerConfig>,
}
fn err_ptr_2_result(err_ptr: UniquePtr<ArcasRTCError>) -> Result<()> {
    if err_ptr.is_null() {
        Ok(())
    } else if err_ptr.ok() {
        Ok(())
    } else {
        Err(err_ptr.into())
    }
}
impl TransportController {
    pub fn new(
        config: TransportControllerConfig,
        network_manager: &mut RTCNetworkManager,
        network_thread: &mut RTCThread,
        min_port: i32,
        max_port: i32,
    ) -> Self {
        let net_man = unsafe { network_manager.inner.pin_mut().get_unchecked_mut() };
        let mut port_alloc = unsafe { create_arcas_cxx_port_allocator(net_man) };
        port_alloc.pin_mut().set_port_range(min_port, max_port);
        init_port_alloc(port_alloc.pin_mut(), network_thread.inner.pin_mut());
        let async_dns_resolver_factory = create_arcas_cxx_async_dns_resolver_factory();
        let net_thr = unsafe { network_thread.inner.pin_mut().get_unchecked_mut() };
        let inner = unsafe {
            create_arcas_jsep_transport_controller(
                net_thr,
                port_alloc,
                async_dns_resolver_factory.into_raw(),
                config.inner,
            )
        };
        return Self { inner };
    }
    pub fn set_local_description(&mut self, sdp: &SessionDescription) -> Result<()> {
        let typ = sdp.get_type();
        // let copy = create_arcas_session_description(typ.into(), sdp.to_string());
        // if copy.ok {
        let err_ptr: UniquePtr<ArcasRTCError> = self
            .inner
            .pin_mut()
            // .set_local_description(typ.into(), copy.session);
            .set_local_description(typ.into(), sdp.cxx_sdp.deref());
        err_ptr_2_result(err_ptr)
        // } else {
        //     Err(copy.error.into())
        // }
    }
    pub fn set_remote_description(&mut self, sdp: &SessionDescription) -> Result<()> {
        let typ = sdp.get_type();
        // let copy = create_arcas_session_description(typ.into(), sdp.to_string());
        // if copy.ok {
        let err_ptr: UniquePtr<ArcasRTCError> = self
            .inner
            .pin_mut()
            .set_remote_description(typ.into(), sdp.cxx_sdp.deref());
        // .set_remote_description(typ.into(), copy.session);
        err_ptr_2_result(err_ptr)
        // } else {
        //     Err(copy.error.into())
        // }
    }
    pub fn set_local_certificate(&mut self, certificate: SSLCertificate) {
        self.inner
            .pin_mut()
            .set_local_certificate(certificate.inner);
    }
    pub fn set_ice_config(&mut self, ice_config: P2pIceConfig) {
        self.inner.pin_mut().set_ice_config(ice_config.cxx_ptr);
    }
    pub fn add_remote_candidates(
        &mut self,
        mid: String,
        candidates: Vec<ArcasCandidateWrapper>,
    ) -> Result<()> {
        let err_ptr = self.inner.pin_mut().add_remote_candidates(mid, candidates);
        err_ptr_2_result(err_ptr)
    }
    pub fn maybe_start_gathering(&mut self) {
        self.inner.pin_mut().maybe_start_gathering();
    }
}

impl TransportControllerConfig {
    pub fn new() -> TransportControllerConfig {
        TransportControllerConfig {
            inner: create_arcas_jsep_transport_controller_config(),
        }
    }
    fn set_transport_observer(&mut self, callback: Box<OnTransportChanged>) {
        let cb = ArcasRustJsepTransportControllerObserver::new(callback);
        let cb = Box::new(cb);
        let cb = to_cxx(cb);
        self.inner.pin_mut().set_transport_observer(cb);
    }

    fn set_rtcp_handler(
        &mut self,
        callback: Box<dyn Fn(&libwebrtc_sys::pc::jsep_api::ffi::CopyOnWriteBuffer, i64)>,
    ) {
        let cb = ArcasRustJsepRTCPHandler::new(callback);
        let cb = Box::new(cb);
        self.inner.pin_mut().set_rtcp_handler(cb);
    }
    fn set_dtls_handshake_error_handler(
        &mut self,
        callback: Box<dyn Fn(ArcasCxxSSLHandshakeError)>,
    ) {
        let cb = ArcasRustDTLSHandshakeErrorHandler::new(callback);
        let cb = Box::new(cb);
        self.inner.pin_mut().set_dtls_handshake_error_handler(cb);
    }
    fn set_ice_transport_factory(&mut self, factory: IceTransportFactory) {
        self.inner
            .pin_mut()
            .set_ice_transport_factory(factory.inner);
    }
}

impl Default for TransportControllerConfig {
    fn default() -> Self {
        Self::new()
    }
}

// impl Vec<libwebrtc_sys::pc::jsep_api::ArcasRustDTLSHandshakeErrorHandler> {}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::certificate::{KeyParams, SSLCertificate, SSLIdentity};
    use crate::transport::IceTransportFactory;
    use libwebrtc_sys::pc::jsep_api::ffi::ArcasJsepTransportControllerObserver;
    use libwebrtc_sys::pc::jsep_api::{
        ArcasRustDTLSHandshakeErrorHandler, ArcasRustJsepRTCPHandler,
        ArcasRustJsepTransportControllerObserver,
    };
    use local_ip_address::local_ip;
    use std::ops::DerefMut;
    use std::pin::Pin;
    use std::sync::mpsc::channel;
    use std::{thread, time};

    fn some_callback(
        txt: String,
        _a: *mut libwebrtc_sys::pc::jsep_api::ffi::RtpTransportInternal,
        _b: UniquePtr<libwebrtc_sys::pc::jsep_api::ffi::ArcasDTLSTransport>,
        _c: *mut libwebrtc_sys::pc::jsep_api::ffi::DataChannelTransportInterface,
    ) -> bool {
        println!("called back: {}", txt);
        true
    }
    fn different_callback(_b: &CopyOnWriteBuffer, _i: i64) {}
    fn a_third(_e: ArcasCxxSSLHandshakeError) {}
    fn offer_text(fingerprint: String) -> String {
        return format!(
            r#"v=0
o=- 1078046247596315931 2 IN IP4 {}
s=-
t=0 0
a=group:BUNDLE 0
a=extmap-allow-mixed
a=msid-semantic: WMS 0
m=video 3340 TCP/TLS/RTP/SAVPF 96 97 98 99 100
c=IN IP4 0.0.0.0
a=rtcp:9 IN IP4 0.0.0.0
a=ice-ufrag:UUEB
a=ice-pwd:oBEHxMxyOKOvLLu13/fDqA68
a=ice-options:trickle
a=fingerprint:{}
a=setup:actpass
a=mid:0
a=extmap:1 urn:ietf:params:rtp-hdrext:toffset
a=extmap:2 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:3 urn:3gpp:video-orientation
a=extmap:4 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=extmap:5 http://www.webrtc.org/experiments/rtp-hdrext/playout-delay
a=extmap:6 http://www.webrtc.org/experiments/rtp-hdrext/video-content-type
a=extmap:7 http://www.webrtc.org/experiments/rtp-hdrext/video-timing
a=extmap:8 http://www.webrtc.org/experiments/rtp-hdrext/color-space
a=extmap:9 urn:ietf:params:rtp-hdrext:sdes:mid
a=extmap:10 urn:ietf:params:rtp-hdrext:sdes:rtp-stream-id
a=extmap:11 urn:ietf:params:rtp-hdrext:sdes:repaired-rtp-stream-id
a=sendrecv
a=msid:0 Testlabel
a=rtcp-mux
a=rtcp-rsize
a=rtpmap:96 VP8/90000
a=rtcp-fb:96 goog-remb
a=rtcp-fb:96 transport-cc
a=rtcp-fb:96 ccm fir
a=rtcp-fb:96 nack
a=rtcp-fb:96 nack pli
a=rtpmap:97 rtx/90000
a=fmtp:97 apt=96
a=rtpmap:98 red/90000
a=rtpmap:99 rtx/90000
a=fmtp:99 apt=98
a=rtpmap:100 ulpfec/90000
a=ssrc-group:FID 3901065077 2566872679
a=ssrc:3901065077 cname:uNQfOR05i3pJxaq1
a=ssrc:3901065077 msid:0 Testlabel
a=ssrc:3901065077 mslabel:0
a=ssrc:3901065077 label:Testlabel
a=ssrc:2566872679 cname:uNQfOR05i3pJxaq1
a=ssrc:2566872679 msid:0 Testlabel
a=ssrc:2566872679 mslabel:0
a=ssrc:2566872679 label:Testlabel
"#,
            local_ip().unwrap(),
            fingerprint
        );
    }
    fn answer_text(fingerprint: String) -> String {
        return format!(
            r#"v=0
o=- 1078046247596315931 2 IN IP4 {}
s=-
t=0 0
a=extmap-allow-mixed
a=msid-semantic: WMS 0
m=video 3341 TCP/TLS/RTP/SAVPF 96 97 98 99 100
c=IN IP4 0.0.0.0
a=rtcp:9 IN IP4 0.0.0.0
a=ice-ufrag:UUEB
a=ice-pwd:oBEHxMxyOKOvLLu13/fDqA68
a=ice-options:trickle
a=fingerprint:{}
a=setup:passive
a=mid:0
a=extmap:1 urn:ietf:params:rtp-hdrext:toffset
a=extmap:2 http://www.webrtc.org/experiments/rtp-hdrext/abs-send-time
a=extmap:3 urn:3gpp:video-orientation
a=extmap:4 http://www.ietf.org/id/draft-holmer-rmcat-transport-wide-cc-extensions-01
a=extmap:5 http://www.webrtc.org/experiments/rtp-hdrext/playout-delay
a=extmap:6 http://www.webrtc.org/experiments/rtp-hdrext/video-content-type
a=extmap:7 http://www.webrtc.org/experiments/rtp-hdrext/video-timing
a=extmap:8 http://www.webrtc.org/experiments/rtp-hdrext/color-space
a=extmap:9 urn:ietf:params:rtp-hdrext:sdes:mid
a=extmap:10 urn:ietf:params:rtp-hdrext:sdes:rtp-stream-id
a=extmap:11 urn:ietf:params:rtp-hdrext:sdes:repaired-rtp-stream-id
a=sendrecv
a=msid:0 Testlabel
a=rtcp-mux
a=rtcp-rsize
a=rtpmap:96 VP8/90000
a=rtcp-fb:96 goog-remb
a=rtcp-fb:96 transport-cc
a=rtcp-fb:96 ccm fir
a=rtcp-fb:96 nack
a=rtcp-fb:96 nack pli
a=rtpmap:97 rtx/90000
a=fmtp:97 apt=96
a=rtpmap:98 red/90000
a=rtpmap:99 rtx/90000
a=fmtp:99 apt=98
a=rtpmap:100 ulpfec/90000
a=ssrc-group:FID 3901065077 2566872679
a=ssrc:3901065077 cname:uNQfOR05i3pJxaq1
a=ssrc:3901065077 msid:0 Testlabel
a=ssrc:3901065077 mslabel:0
a=ssrc:3901065077 label:Testlabel
a=ssrc:2566872679 cname:uNQfOR05i3pJxaq1
a=ssrc:2566872679 msid:0 Testlabel
a=ssrc:2566872679 mslabel:0
a=ssrc:2566872679 label:Testlabel
"#,
            local_ip().unwrap(),
            fingerprint
        );
    }

    #[test]
    fn test_transport_controller() {
        let mut net_man = RTCNetworkManager::default();
        let mut net_thread = RTCThread::with_socket_server();
        net_thread.set_name("test_transport_controller's network thread".into());
        net_thread.start();

        let mut tc1 =
            TransportController::new(plain_config(), &mut net_man, &mut net_thread, 3340, 3340);
        let parms = KeyParams::new(KeyType::KT_RSA);
        let ident = SSLIdentity::new("common name".to_string(), parms);
        let cert1 = SSLCertificate::new(ident);
        let finger1 = cert1.get_fingerprint();

        tc1.set_local_certificate(cert1);

        let mut tc2 =
            TransportController::new(plain_config(), &mut net_man, &mut net_thread, 3341, 3341);
        let parms = KeyParams::new(KeyType::KT_RSA);
        let ident2 = SSLIdentity::new("other common name".to_string(), parms);
        let cert2 = SSLCertificate::new(ident2);
        let fing2 = cert2.get_fingerprint();
        tc2.set_local_certificate(cert2);

        let sdp1 = offer_text(finger1);
        let offer = SessionDescription::new(SDPType::Offer, sdp1);
        assert!(offer.is_ok());
        let offer = offer.unwrap();
        let remote_offer = offer.copy_to_remote().unwrap();

        let err = tc1.set_local_description(&offer);
        assert!(err.is_ok());

        let err = tc2.set_remote_description(&remote_offer);
        assert!(err.is_ok());

        let sdp2_text = answer_text(fing2);
        let sdp2 = SessionDescription::new(SDPType::Answer, sdp2_text.clone());
        let sdp2 = sdp2.unwrap();
        let err = tc2.set_local_description(&sdp2);
        assert!(err.is_ok());

        let err = tc1.set_remote_description(&sdp2);
        assert!(err.is_ok());

        let mut ice_cfg = P2pIceConfig::default();
        ice_cfg.set_presume_writable_when_fully_relayed(true);
        tc1.set_ice_config(ice_cfg);
        ice_cfg = P2pIceConfig::default();
        ice_cfg.set_presume_writable_when_fully_relayed(true);
        tc2.set_ice_config(ice_cfg);
        let ip = local_ip().unwrap().to_string();

        let mut cand2 = ArcasCandidateWrapper::default();
        cand2.set_address(ip.clone() + ":3341");
        cand2.set_component(CandidateComponent::Rtp);
        cand2.set_protocol("tcp".into());
        let mut cand1 = ArcasCandidateWrapper::default();
        cand1.set_address(ip + ":3340");
        cand1.set_component(CandidateComponent::Rtp);
        cand1.set_protocol("tcp".into());

        let err = tc1.add_remote_candidates("0".to_string(), vec![cand2]);
        assert!(err.is_ok());
        let err = tc2.add_remote_candidates("0".to_string(), vec![cand1]);
        assert!(err.is_ok());

        let mut transport = tc1.inner.get_srtp_transport("0".into());
        let transport_name = unsafe { get_transport_name(&*transport, net_thread.inner.pin_mut()) };
        assert_eq!(transport_name, "0".to_string());

        let mut transport_pin = unsafe { Pin::new_unchecked(&mut *transport) };
        set_rtp_params(
            transport_pin,
            9,
            "012345678901234567890123456789".into(),
            8,
            "901234567890123456789012345678".into(),
            vec![],
            net_thread.inner.pin_mut(),
        );

        let mut transport2 = tc2.inner.get_srtp_transport("0".into());
        let transport2_name =
            unsafe { get_transport_name(&*transport2, net_thread.inner.pin_mut()) };
        assert_eq!(transport2_name, "0".to_string());

        let mut transport2_pin = unsafe { Pin::new_unchecked(&mut *transport2) };
        set_rtp_params(
            transport2_pin,
            9,
            "012345678901234567890123456789".into(),
            8,
            "901234567890123456789012345678".into(),
            vec![],
            net_thread.inner.pin_mut(),
        );

        tc1.maybe_start_gathering();
        tc2.maybe_start_gathering();
        /*
        for i in 1..9 {
            let ready = unsafe { is_writable(&*transport) };
            if ready {
                break;
            }
            assert!(i < 9);
            thread::sleep(time::Duration::from_secs(i));
        }
        //The first two bytes: \0x807 (that's the digit 7) are a message header
        let mut cow = create_buffer_with_data(b"\x80701234567890");
        transport_pin = unsafe { Pin::new_unchecked(&mut *transport) };
        let mut sent = send_rtp_packet(transport_pin, cow.pin_mut(), net_thread.inner.pin_mut());
        assert!(sent);
         */
    }
    fn plain_config() -> TransportControllerConfig {
        let mut cfg = TransportControllerConfig::default();
        cfg.set_transport_observer(Box::new(some_callback));
        cfg.set_rtcp_handler(Box::new(different_callback));
        cfg.set_ice_transport_factory(IceTransportFactory::default());
        cfg.set_dtls_handshake_error_handler(Box::new(a_third));
        cfg
    }
}
