//! Shared, in-memory OpenSSL fixtures for the TLS and STAR benchmarks.

use openssl::ssl::{
    ErrorCode, HandshakeError, MidHandshakeSslStream, SslAcceptorBuilder, SslConnector,
    SslConnectorBuilder, SslFiletype, SslMethod, SslVerifyMode,
};
use std::io::{self, Read, Write};

const CERT_PATH: &str = "examples/certs/server.crt";
const KEY_PATH: &str = "examples/certs/server.key";

pub enum CertificateKind {
    File,
    Chain,
}

pub fn configure_server(builder: &mut SslAcceptorBuilder, kind: CertificateKind) {
    let cert = super::manifest_path(CERT_PATH);
    match kind {
        CertificateKind::File => builder
            .set_certificate_file(cert, SslFiletype::PEM)
            .expect("set TLS certificate"),
        CertificateKind::Chain => builder
            .set_certificate_chain_file(cert)
            .expect("set TLS certificate chain"),
    }
    builder
        .set_private_key_file(super::manifest_path(KEY_PATH), SslFiletype::PEM)
        .expect("set TLS private key");
    builder.check_private_key().expect("check TLS private key");
    builder.set_groups_list("X25519").expect("set TLS groups");
}

pub fn client_connector() -> SslConnectorBuilder {
    let mut builder = SslConnector::builder(SslMethod::tls()).expect("build TLS connector");
    builder.set_verify(SslVerifyMode::NONE);
    builder.set_groups_list("X25519").expect("set TLS groups");
    builder
}

/// Returns the recorded ClientHello and the paused client for optional setup-only validation.
/// No TLS client is ever run in the measured path.
pub fn prepare_client_hello(
    connector: SslConnector,
) -> (Vec<u8>, MidHandshakeSslStream<MemoryStream<Vec<u8>>>) {
    let client = match connector.connect("localhost", MemoryStream::recording(Vec::new())) {
        Err(HandshakeError::WouldBlock(client)) if client.error().code() == ErrorCode::WANT_READ => {
            client
        }
        _ => panic!("TLS client must stop after emitting ClientHello"),
    };
    let hello = client.get_ref().written().to_vec();
    assert!(!hello.is_empty(), "recorded ClientHello is empty");
    (hello, client)
}

/// The reader returns WouldBlock once input is exhausted; writes may be captured or discarded.
/// Borrowed input avoids cloning the same ClientHello on each server-side operation.
#[derive(Debug)]
pub struct MemoryStream<I> {
    input: I,
    offset: usize,
    output: Option<Vec<u8>>,
}

impl<I: AsRef<[u8]>> MemoryStream<I> {
    pub fn new(input: I) -> Self {
        Self { input, offset: 0, output: None }
    }

    pub fn recording(input: I) -> Self {
        Self { input, offset: 0, output: Some(Vec::new()) }
    }

    pub fn consumed_all(&self) -> bool {
        self.offset == self.input.as_ref().len()
    }

    pub fn written(&self) -> &[u8] {
        self.output.as_deref().unwrap_or(&[])
    }
}

impl MemoryStream<Vec<u8>> {
    pub fn set_input(&mut self, input: Vec<u8>) {
        self.input = input;
        self.offset = 0;
    }
}

impl<I: AsRef<[u8]>> Read for MemoryStream<I> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let remaining = &self.input.as_ref()[self.offset..];
        if remaining.is_empty() {
            return Err(io::ErrorKind::WouldBlock.into());
        }
        let len = remaining.len().min(buf.len());
        buf[..len].copy_from_slice(&remaining[..len]);
        self.offset += len;
        Ok(len)
    }
}

impl<I: AsRef<[u8]>> Write for MemoryStream<I> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(output) = &mut self.output {
            output.extend_from_slice(buf);
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
