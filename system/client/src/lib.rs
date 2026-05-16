use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::URL_SAFE};
use log::debug;
use openssl::ssl::{ExtensionContext, SslConnector, SslMethod, SslVerifyMode};
use pingora::apps::ServerApp;
use pingora::protocols::Stream;
use pingora::protocols::tls::SslStream;
use pingora::server::{Server, ShutdownWatch};
use pingora::services::listening::Service;
use rand::rngs::OsRng;
use star_core::{client::User, secure_channel::client::SecureChannelClient};
use std::env;
use std::error::Error;
use std::fmt;
use std::io;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::net::TcpStream;

pub const STAR_TLS_EXTENSION_TYPE: u16 = 65280;
const MAX_HEADER_SIZE: usize = 64 * 1024;

type BoxError = Box<dyn Error + Send + Sync>;

trait ProxyIo: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T> ProxyIo for T where T: AsyncRead + AsyncWrite + Unpin + Send {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Scheme {
    Http,
    Https,
}

impl Scheme {
    fn default_port(self) -> u16 {
        match self {
            Scheme::Http => 80,
            Scheme::Https => 443,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetRule {
    scheme: Option<Scheme>,
    host: String,
    port: Option<u16>,
}

impl TargetRule {
    pub fn parse_list(input: &str) -> Vec<Self> {
        input
            .split(',')
            .filter_map(|part| Self::parse(part.trim()))
            .collect()
    }

    pub fn parse(input: &str) -> Option<Self> {
        if input.is_empty() {
            return None;
        }

        let (scheme, rest) = if let Some(rest) = input.strip_prefix("https://") {
            (Some(Scheme::Https), rest)
        } else if let Some(rest) = input.strip_prefix("http://") {
            (Some(Scheme::Http), rest)
        } else {
            (None, input)
        };

        let authority = rest.split('/').next().unwrap_or(rest);
        let (host, port) = parse_authority(authority, scheme.map(Scheme::default_port))?;
        let port = if authority_has_explicit_port(authority) {
            Some(port)
        } else {
            None
        };

        Some(Self { scheme, host, port })
    }

    fn matches(&self, target: &UpstreamTarget) -> bool {
        self.scheme.is_none_or(|scheme| scheme == target.scheme)
            && self.host == normalize_host(&target.host)
            && self.port.is_none_or(|port| port == target.port)
    }
}

#[derive(Clone, Debug)]
pub struct ProxyConfig {
    pub listen_addr: String,
    pub api_base: String,
    pub star_targets: Vec<TargetRule>,
    pub insecure_tls: bool,
    pub max_count: u64,
}

impl ProxyConfig {
    pub fn from_env() -> Self {
        let targets = env::var("STAR_TARGETS")
            .unwrap_or_else(|_| "https://localhost:8081,https://127.0.0.1:8081".to_string());

        Self {
            listen_addr: env::var("STAR_CLIENT_PROXY_LISTEN")
                .unwrap_or_else(|_| "127.0.0.1:18080".to_string()),
            api_base: env::var("STAR_API_BASE")
                .unwrap_or_else(|_| "http://127.0.0.1:8080".to_string()),
            star_targets: TargetRule::parse_list(&targets),
            insecure_tls: truthy_env("STAR_TLS_INSECURE"),
            max_count: env::var("STAR_MAX_COUNT")
                .ok()
                .and_then(|value| value.parse().ok())
                .unwrap_or(10),
        }
    }
}

#[derive(Clone)]
pub struct StarAuthenticator {
    api_base: String,
    max_count: u64,
    inner: Arc<Mutex<StarAuthState>>,
}

struct StarAuthState {
    user: User,
    counter: u64,
    period: u64,
}

impl StarAuthenticator {
    pub fn register(api_base: String, max_count: u64) -> Result<Self, BoxError> {
        let public_key = http_get_text(&format!(
            "{}/star/public_key",
            api_base.trim_end_matches('/')
        ))?;
        let public_key = URL_SAFE.decode(public_key.trim())?;
        let public_key: [u8; 32] = public_key
            .as_slice()
            .try_into()
            .map_err(|_| "public key must be 32 bytes")?;

        let client = SecureChannelClient::new(public_key)
            .map_err(|e| format!("failed to initialize secure channel: {e:?}"))?;
        let mut user = User::new(max_count, client);
        let mut rng = OsRng;
        let (request, shared_key) = user.request_credential(&mut rng);
        let request = URL_SAFE.encode(request);

        let credential_cipher = http_post_text(
            &format!("{}/star/register", api_base.trim_end_matches('/')),
            request.as_bytes(),
        )?;
        let credential_cipher = URL_SAFE.decode(credential_cipher.trim())?;
        user.receive_credential(&credential_cipher, &shared_key)
            .ok_or("failed to receive credential")?;

        Ok(Self {
            api_base,
            max_count,
            inner: Arc::new(Mutex::new(StarAuthState {
                user,
                counter: 0,
                period: current_period(),
            })),
        })
    }

    fn token_request(&self) -> Result<Vec<u8>, BoxError> {
        let mut state = self.inner.lock().map_err(|_| "STAR auth mutex poisoned")?;
        let period = current_period();
        if state.period != period {
            state.period = period;
            state.counter = 0;
        }
        if state.counter >= self.max_count {
            return Err(format!(
                "exceeded {max} STAR token requests for period {period}; wait for the next minute",
                max = self.max_count
            )
            .into());
        }

        let counter = state.counter;
        state.counter += 1;
        let mut rng = OsRng;
        state
            .user
            .request_auth(counter, period, &mut rng)
            .map_err(|e| {
                format!(
                    "failed to build STAR token request via {}: {e:?}",
                    self.api_base
                )
                .into()
            })
    }
}

pub struct StarForwardProxy {
    star_targets: Vec<TargetRule>,
    authenticator: Option<StarAuthenticator>,
    insecure_tls: bool,
}

impl StarForwardProxy {
    pub fn new(config: ProxyConfig) -> Result<Self, BoxError> {
        let authenticator = if config.star_targets.is_empty() {
            None
        } else {
            Some(StarAuthenticator::register(
                config.api_base.clone(),
                config.max_count,
            )?)
        };

        Ok(Self {
            star_targets: config.star_targets,
            authenticator,
            insecure_tls: config.insecure_tls,
        })
    }

    async fn handle(&self, downstream: &mut Stream) -> io::Result<()> {
        let raw = read_header(downstream).await?;
        let request = ParsedRequest::parse(&raw)?;

        if request.method.eq_ignore_ascii_case("CONNECT") {
            return self.handle_connect(downstream, &request).await;
        }

        self.handle_forward(downstream, &request).await
    }

    async fn handle_connect(
        &self,
        downstream: &mut Stream,
        request: &ParsedRequest,
    ) -> io::Result<()> {
        let (host, port) = parse_authority(&request.uri, Some(443))
            .ok_or_else(|| io_error("invalid CONNECT authority"))?;
        let target = UpstreamTarget {
            scheme: Scheme::Https,
            host,
            port,
            authority: request.uri.clone(),
            path: String::new(),
        };

        if self.should_attach_star(&target) {
            write_status(
                downstream,
                421,
                "Misdirected Request",
                "STAR targets must be requested as absolute-form HTTPS so the proxy can create the TLS ClientHello.\n",
            )
            .await?;
            return Ok(());
        }

        let mut upstream = TcpStream::connect((target.host.as_str(), target.port)).await?;
        downstream
            .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
            .await?;
        downstream.flush().await?;
        let _ = tokio::io::copy_bidirectional(&mut **downstream, &mut upstream).await;
        Ok(())
    }

    async fn handle_forward(
        &self,
        downstream: &mut Stream,
        request: &ParsedRequest,
    ) -> io::Result<()> {
        let target = parse_forward_target(request)
            .ok_or_else(|| io_error("request target must be absolute-form or include Host"))?;
        let token = if self.should_attach_star(&target) {
            Some(
                self.authenticator
                    .as_ref()
                    .ok_or_else(|| io_error("STAR authenticator is not configured"))?
                    .token_request()
                    .map_err(|e| io_error(e.to_string()))?,
            )
        } else {
            None
        };

        let mut upstream = self.connect_upstream(&target, token).await?;
        let header = request.forward_header(&target);
        upstream.write_all(&header).await?;

        if let Some(content_length) = request.content_length() {
            write_fixed_body(downstream, &mut *upstream, request.body(), content_length).await?;
            copy_response(&mut *upstream, downstream).await?;
        } else if request.is_chunked() {
            upstream.write_all(request.body()).await?;
            upstream.flush().await?;
            let _ = tokio::io::copy_bidirectional(&mut **downstream, &mut *upstream).await;
        } else {
            upstream.flush().await?;
            copy_response(&mut *upstream, downstream).await?;
        }

        Ok(())
    }

    async fn connect_upstream(
        &self,
        target: &UpstreamTarget,
        star_token: Option<Vec<u8>>,
    ) -> io::Result<Box<dyn ProxyIo>> {
        let tcp = TcpStream::connect((target.host.as_str(), target.port)).await?;
        if target.scheme == Scheme::Http {
            return Ok(Box::new(tcp));
        }

        let mut builder = SslConnector::builder(SslMethod::tls()).map_err(io_other)?;
        if self.insecure_tls {
            builder.set_verify(SslVerifyMode::NONE);
        }
        if let Some(token) = star_token {
            builder
                .add_custom_ext(
                    STAR_TLS_EXTENSION_TYPE,
                    ExtensionContext::TLS_ONLY
                        | ExtensionContext::CLIENT_HELLO
                        | ExtensionContext::TLS1_2_AND_BELOW_ONLY
                        | ExtensionContext::TLS1_3_ONLY,
                    move |_, _, _| Ok(Some(token.clone())),
                    |_, _, _, _| Ok(()),
                )
                .map_err(io_other)?;
        }

        let ssl = builder
            .build()
            .configure()
            .map_err(io_other)?
            .into_ssl(&target.host)
            .map_err(io_other)?;
        let mut tls = SslStream::new(ssl, tcp).map_err(io_other)?;
        tls.connect().await.map_err(io_other)?;
        Ok(Box::new(tls))
    }

    fn should_attach_star(&self, target: &UpstreamTarget) -> bool {
        target.scheme == Scheme::Https && self.star_targets.iter().any(|rule| rule.matches(target))
    }
}

#[async_trait]
impl ServerApp for StarForwardProxy {
    async fn process_new(
        self: &Arc<Self>,
        mut io: Stream,
        _shutdown: &ShutdownWatch,
    ) -> Option<Stream> {
        if let Err(err) = self.handle(&mut io).await {
            debug!("client proxy connection closed: {err}");
        }
        None
    }
}

pub fn build_server(config: ProxyConfig) -> Result<Server, BoxError> {
    let listen_addr = config.listen_addr.clone();
    let proxy = StarForwardProxy::new(config)?;
    let mut server = Server::new(None)?;
    server.bootstrap();

    let mut service = Service::new("STAR client HTTP proxy".to_string(), proxy);
    service.add_tcp(&listen_addr);
    server.add_service(service);
    Ok(server)
}

#[derive(Debug)]
struct ParsedRequest {
    method: String,
    uri: String,
    version: u8,
    headers: Vec<(String, Vec<u8>)>,
    header_len: usize,
    raw: Vec<u8>,
}

impl ParsedRequest {
    fn parse(raw: &[u8]) -> io::Result<Self> {
        let header_len = find_header_end(raw).ok_or_else(|| io_error("missing header end"))?;
        let mut headers = [httparse::EMPTY_HEADER; 96];
        let mut req = httparse::Request::new(&mut headers);
        let status = req.parse(&raw[..header_len]).map_err(io_other)?;
        if !status.is_complete() {
            return Err(io_error("incomplete request header"));
        }

        let headers = req
            .headers
            .iter()
            .map(|header| (header.name.to_string(), header.value.to_vec()))
            .collect();

        Ok(Self {
            method: req.method.unwrap_or("").to_string(),
            uri: req.path.unwrap_or("").to_string(),
            version: req.version.unwrap_or(1),
            headers,
            header_len,
            raw: raw.to_vec(),
        })
    }

    fn body(&self) -> &[u8] {
        &self.raw[self.header_len..]
    }

    fn host(&self) -> Option<String> {
        self.headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("host"))
            .and_then(|(_, value)| String::from_utf8(value.clone()).ok())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
    }

    fn content_length(&self) -> Option<usize> {
        self.headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("content-length"))
            .and_then(|(_, value)| std::str::from_utf8(value).ok())
            .and_then(|value| value.trim().parse().ok())
    }

    fn is_chunked(&self) -> bool {
        self.headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("transfer-encoding"))
            .and_then(|(_, value)| std::str::from_utf8(value).ok())
            .is_some_and(|value| value.to_ascii_lowercase().contains("chunked"))
    }

    fn forward_header(&self, target: &UpstreamTarget) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(
            format!(
                "{} {} HTTP/1.{}\r\n",
                self.method, target.path, self.version
            )
            .as_bytes(),
        );
        out.extend_from_slice(format!("Host: {}\r\n", target.authority).as_bytes());
        out.extend_from_slice(b"Connection: close\r\n");

        for (name, value) in &self.headers {
            if name.eq_ignore_ascii_case("host")
                || name.eq_ignore_ascii_case("connection")
                || name.eq_ignore_ascii_case("proxy-connection")
                || name.eq_ignore_ascii_case("proxy-authorization")
            {
                continue;
            }
            out.extend_from_slice(name.as_bytes());
            out.extend_from_slice(b": ");
            out.extend_from_slice(value);
            out.extend_from_slice(b"\r\n");
        }
        out.extend_from_slice(b"\r\n");
        out
    }
}

#[derive(Debug)]
struct UpstreamTarget {
    scheme: Scheme,
    host: String,
    port: u16,
    authority: String,
    path: String,
}

fn parse_forward_target(request: &ParsedRequest) -> Option<UpstreamTarget> {
    if let Some(target) = parse_absolute_uri(&request.uri) {
        return Some(target);
    }

    let authority = request.host()?;
    let (host, port) = parse_authority(&authority, Some(80))?;
    Some(UpstreamTarget {
        scheme: Scheme::Http,
        host,
        port,
        authority,
        path: request.uri.clone(),
    })
}

fn parse_absolute_uri(uri: &str) -> Option<UpstreamTarget> {
    let (scheme, rest) = if let Some(rest) = uri.strip_prefix("http://") {
        (Scheme::Http, rest)
    } else if let Some(rest) = uri.strip_prefix("https://") {
        (Scheme::Https, rest)
    } else {
        return None;
    };

    let path_start = rest.find(['/', '?']).unwrap_or(rest.len());
    let authority = &rest[..path_start];
    let path = if path_start == rest.len() {
        "/".to_string()
    } else if rest.as_bytes()[path_start] == b'?' {
        format!("/{}", &rest[path_start..])
    } else {
        rest[path_start..].to_string()
    };
    let (host, port) = parse_authority(authority, Some(scheme.default_port()))?;

    Some(UpstreamTarget {
        scheme,
        host,
        port,
        authority: authority.to_string(),
        path,
    })
}

fn parse_authority(authority: &str, default_port: Option<u16>) -> Option<(String, u16)> {
    let authority = authority.rsplit('@').next().unwrap_or(authority);
    if authority.starts_with('[') {
        let end = authority.find(']')?;
        let host = &authority[1..end];
        let port = authority[end + 1..]
            .strip_prefix(':')
            .and_then(|value| value.parse().ok())
            .or(default_port)?;
        return Some((normalize_host(host), port));
    }

    if let Some((host, port)) = authority.rsplit_once(':') {
        if let Ok(port) = port.parse() {
            return Some((normalize_host(host), port));
        }
    }

    Some((normalize_host(authority), default_port?))
}

fn authority_has_explicit_port(authority: &str) -> bool {
    authority
        .rsplit_once(':')
        .is_some_and(|(_, port)| port.parse::<u16>().is_ok())
}

fn normalize_host(host: &str) -> String {
    host.trim_matches(['[', ']'])
        .trim_end_matches('.')
        .to_ascii_lowercase()
}

async fn read_header(stream: &mut Stream) -> io::Result<Vec<u8>> {
    let mut out = Vec::with_capacity(4096);
    let mut buf = [0u8; 2048];
    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Err(io_error("connection closed before request header"));
        }
        out.extend_from_slice(&buf[..n]);
        if find_header_end(&out).is_some() {
            return Ok(out);
        }
        if out.len() > MAX_HEADER_SIZE {
            return Err(io_error("request header too large"));
        }
    }
}

fn find_header_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|pos| pos + 4)
}

async fn write_fixed_body(
    downstream: &mut Stream,
    upstream: &mut dyn ProxyIo,
    buffered: &[u8],
    content_length: usize,
) -> io::Result<()> {
    let already = buffered.len().min(content_length);
    if already > 0 {
        upstream.write_all(&buffered[..already]).await?;
    }

    let mut remaining = content_length - already;
    let mut buf = [0u8; 8192];
    while remaining > 0 {
        let want = remaining.min(buf.len());
        let n = downstream.read(&mut buf[..want]).await?;
        if n == 0 {
            return Err(io_error("downstream closed during request body"));
        }
        upstream.write_all(&buf[..n]).await?;
        remaining -= n;
    }
    upstream.flush().await
}

async fn copy_response(upstream: &mut dyn ProxyIo, downstream: &mut Stream) -> io::Result<()> {
    tokio::io::copy(upstream, &mut **downstream).await?;
    downstream.flush().await
}

async fn write_status(
    stream: &mut Stream,
    status: u16,
    reason: &str,
    body: &str,
) -> io::Result<()> {
    let response = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
    stream.write_all(response.as_bytes()).await?;
    stream.flush().await
}

fn current_period() -> u64 {
    u64::from_be_bytes(constant_time_utils::time::current_period())
}

fn http_get_text(url: &str) -> Result<String, BoxError> {
    Ok(ureq::get(url).call()?.into_string()?)
}

fn http_post_text(url: &str, body: &[u8]) -> Result<String, BoxError> {
    Ok(ureq::post(url).send_bytes(body)?.into_string()?)
}

fn truthy_env(name: &str) -> bool {
    env::var(name)
        .map(|value| {
            let value = value.trim();
            !value.is_empty() && value != "0" && !value.eq_ignore_ascii_case("false")
        })
        .unwrap_or(false)
}

fn io_error(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::Other, message.into())
}

fn io_other(error: impl fmt::Display) -> io::Error {
    io_error(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_star_target_rules() {
        let rules = TargetRule::parse_list("https://api.example.test:8443,localhost:8081");
        let target = parse_absolute_uri("https://api.example.test:8443/v1?q=1").unwrap();
        assert!(rules[0].matches(&target));

        let target = parse_absolute_uri("https://localhost:8081/").unwrap();
        assert!(rules[1].matches(&target));

        let target = parse_absolute_uri("http://localhost:8081/").unwrap();
        assert!(!rules[0].matches(&target));
    }

    #[test]
    fn rewrites_absolute_form_and_strips_proxy_headers() {
        let raw = b"GET https://example.test:8443/a?b=1 HTTP/1.1\r\nHost: proxy-value\r\nProxy-Connection: keep-alive\r\nProxy-Authorization: secret\r\nAccept: */*\r\n\r\n";
        let request = ParsedRequest::parse(raw).unwrap();
        let target = parse_forward_target(&request).unwrap();
        let rewritten = String::from_utf8(request.forward_header(&target)).unwrap();

        assert!(rewritten.starts_with("GET /a?b=1 HTTP/1.1\r\n"));
        assert!(rewritten.contains("Host: example.test:8443\r\n"));
        assert!(rewritten.contains("Connection: close\r\n"));
        assert!(rewritten.contains("Accept: */*\r\n"));
        assert!(
            !rewritten
                .to_ascii_lowercase()
                .contains("proxy-authorization")
        );
        assert!(!rewritten.to_ascii_lowercase().contains("proxy-connection"));
    }

    #[test]
    fn parses_origin_form_as_plain_http() {
        let raw = b"GET /health HTTP/1.1\r\nHost: 127.0.0.1:3000\r\n\r\n";
        let request = ParsedRequest::parse(raw).unwrap();
        let target = parse_forward_target(&request).unwrap();
        assert_eq!(target.scheme, Scheme::Http);
        assert_eq!(target.host, "127.0.0.1");
        assert_eq!(target.port, 3000);
        assert_eq!(target.path, "/health");
    }
}
