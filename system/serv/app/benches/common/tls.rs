use std::fs;
use std::sync::{Arc, Once};
use std::time::Duration;
use ureq::rustls::client::danger::{
    HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier,
};
use ureq::rustls::pki_types::pem::PemObject;
use ureq::rustls::pki_types::{CertificateDer, ServerName, UnixTime};

#[derive(Debug)]
struct PinnedServerCertVerifier {
    expected_leaf_der: Vec<u8>,
}

impl ServerCertVerifier for PinnedServerCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, ureq::rustls::Error> {
        if end_entity.as_ref() == self.expected_leaf_der.as_slice() {
            Ok(ServerCertVerified::assertion())
        } else {
            Err(ureq::rustls::Error::General(
                "unexpected TLS server certificate".to_string(),
            ))
        }
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &ureq::rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, ureq::rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &ureq::rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, ureq::rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<ureq::rustls::SignatureScheme> {
        vec![
            ureq::rustls::SignatureScheme::RSA_PKCS1_SHA1,
            ureq::rustls::SignatureScheme::RSA_PKCS1_SHA256,
            ureq::rustls::SignatureScheme::RSA_PKCS1_SHA384,
            ureq::rustls::SignatureScheme::RSA_PKCS1_SHA512,
            ureq::rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            ureq::rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            ureq::rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            ureq::rustls::SignatureScheme::RSA_PSS_SHA256,
            ureq::rustls::SignatureScheme::RSA_PSS_SHA384,
            ureq::rustls::SignatureScheme::RSA_PSS_SHA512,
            ureq::rustls::SignatureScheme::ED25519,
            ureq::rustls::SignatureScheme::ED448,
        ]
    }
}

fn load_first_cert_der(cert_path: &str) -> Vec<u8> {
    let cert_pem = fs::read(cert_path).expect("read TLS cert PEM");
    let mut certs = CertificateDer::pem_slice_iter(&cert_pem)
        .collect::<Result<Vec<_>, _>>()
        .expect("parse TLS cert PEM");
    certs
        .drain(..)
        .next()
        .expect("TLS cert PEM did not contain any certificates")
        .as_ref()
        .to_vec()
}

pub fn ensure_rustls_ring_provider() {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let mut provider = rustls::crypto::ring::default_provider();
        provider.kx_groups = vec![rustls::crypto::ring::kx_group::X25519];
        let _ = provider.install_default();

        let mut ureq_provider = ureq::rustls::crypto::ring::default_provider();
        ureq_provider.kx_groups = vec![ureq::rustls::crypto::ring::kx_group::X25519];
        let _ = ureq_provider.install_default();
    });
}

pub fn build_pinned_tls_agent(cert_path: &str, read_timeout: Duration) -> ureq::Agent {
    ensure_rustls_ring_provider();

    let verifier = PinnedServerCertVerifier {
        expected_leaf_der: load_first_cert_der(cert_path),
    };
    let tls_config = ureq::rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(verifier))
        .with_no_client_auth();

    ureq::AgentBuilder::new()
        .tls_config(Arc::new(tls_config))
        .max_idle_connections(0)
        .max_idle_connections_per_host(0)
        .timeout_connect(Duration::from_secs(2))
        .timeout_read(read_timeout)
        .build()
}
