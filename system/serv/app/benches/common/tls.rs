use openssl::ssl::{ExtensionContext, SslAlert, SslConnector, SslMethod, SslVerifyMode};
use star_app::tls::STAR_TLS_EXTENSION_TYPE;
use std::fs;
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex, Once};
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
    build_pinned_tls_agent_with_resolver(cert_path, read_timeout, |netloc: &str| {
        use std::net::ToSocketAddrs;
        netloc.to_socket_addrs().map(Iterator::collect)
    })
}

pub fn build_pinned_tls_agent_with_resolver<R>(
    cert_path: &str,
    read_timeout: Duration,
    resolver: R,
) -> ureq::Agent
where
    R: ureq::Resolver + 'static,
{
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
        .resolver(resolver)
        .max_idle_connections(0)
        .max_idle_connections_per_host(0)
        .timeout_connect(Duration::from_secs(2))
        .timeout_read(read_timeout)
        .build()
}

pub struct StarTlsClient {
    connector: SslConnector,
    current_ext: Arc<Mutex<Vec<u8>>>,
}

impl StarTlsClient {
    pub fn new() -> Result<Self, String> {
        let current_ext = Arc::new(Mutex::new(Vec::new()));

        let mut builder = SslConnector::builder(SslMethod::tls()).map_err(|e| e.to_string())?;
        builder.set_verify(SslVerifyMode::NONE);
        builder
            .set_groups_list("X25519")
            .map_err(|e| e.to_string())?;

        let ext_ref = Arc::clone(&current_ext);
        builder
            .add_custom_ext(
                STAR_TLS_EXTENSION_TYPE,
                ExtensionContext::CLIENT_HELLO,
                move |_, _, _| {
                    let ext = ext_ref
                        .lock()
                        .map_err(|_| SslAlert::DECODE_ERROR)?
                        .clone();

                    if ext.is_empty() {
                        return Err(SslAlert::DECODE_ERROR);
                    }

                    Ok(Some(ext))
                },
                |_, _, _, _| Ok(()),
            )
            .map_err(|e| e.to_string())?;

        Ok(Self {
            connector: builder.build(),
            current_ext,
        })
    }

    pub fn get(
        &self,
        proxy_addr: SocketAddr,
        token_request: &[u8],
        read_timeout: Duration,
    ) -> Result<(u16, String), String> {
        {
            let mut current_ext = self
                .current_ext
                .lock()
                .map_err(|_| "STAR TLS extension mutex poisoned".to_string())?;
            current_ext.clear();
            current_ext.extend_from_slice(token_request);
        }

        let stream = TcpStream::connect_timeout(&proxy_addr, Duration::from_secs(2))
            .map_err(|e| e.to_string())?;

        stream
            .set_read_timeout(Some(read_timeout))
            .map_err(|e| e.to_string())?;
        stream
            .set_write_timeout(Some(read_timeout))
            .map_err(|e| e.to_string())?;

        let mut stream = self
            .connector
            .connect("localhost", stream)
            .map_err(|e| format!("tls connect failed: {e}"))?;

        stream
            .write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n")
            .map_err(|e| e.to_string())?;
        stream.flush().map_err(|e| e.to_string())?;

        let mut raw = Vec::new();
        stream.read_to_end(&mut raw).map_err(|e| e.to_string())?;

        let raw = String::from_utf8_lossy(&raw);
        let (head, body) = raw
            .split_once("\r\n\r\n")
            .ok_or_else(|| "invalid HTTP response".to_string())?;

        let status = head
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .ok_or_else(|| "missing HTTP status".to_string())?
            .parse::<u16>()
            .map_err(|e| e.to_string())?;

        Ok((status, body.to_string()))
    }
}

pub fn https_get_with_star_extension(
    proxy_addr: SocketAddr,
    token_request: &[u8],
    read_timeout: Duration,
) -> Result<(u16, String), String> {
    let client = StarTlsClient::new()?;
    client.get(proxy_addr, token_request, read_timeout)
}