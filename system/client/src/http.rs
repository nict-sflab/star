use std::io;

use tokio::io::{AsyncRead, AsyncReadExt};

const MAX_HEADER_SIZE: usize = 64 * 1024;

pub(crate) const CORS_PREFLIGHT_RESPONSE: &[u8] = b"HTTP/1.1 204 No Content\r\n\
Access-Control-Allow-Origin: *\r\n\
Access-Control-Allow-Methods: GET, POST, PUT, PATCH, DELETE, OPTIONS\r\n\
Access-Control-Allow-Headers: *\r\n\
Access-Control-Max-Age: 86400\r\n\
Content-Length: 0\r\n\
Connection: close\r\n\
\r\n";

pub(crate) struct HttpHead {
    bytes: Vec<u8>,
}

impl HttpHead {
    pub(crate) async fn read_from<R>(stream: &mut R) -> io::Result<Self>
    where
        R: AsyncRead + Unpin,
    {
        let mut bytes = Vec::with_capacity(4096);
        let mut buf = [0u8; 2048];

        loop {
            let n = stream.read(&mut buf).await?;
            if n == 0 {
                return Err(io::Error::other("connection closed before header"));
            }

            bytes.extend_from_slice(&buf[..n]);
            if header_end(&bytes).is_some() {
                return Ok(Self { bytes });
            }
            if bytes.len() > MAX_HEADER_SIZE {
                return Err(io::Error::other("header too large"));
            }
        }
    }

    pub(crate) fn is_cors_preflight(&self) -> bool {
        if !self.bytes.starts_with(b"OPTIONS ") {
            return false;
        }

        let end = header_end(&self.bytes).unwrap();
        let Ok(head) = std::str::from_utf8(&self.bytes[..end]) else {
            return false;
        };

        head.lines().any(|line| {
            line.split_once(':')
                .is_some_and(|(name, _)| name.eq_ignore_ascii_case("access-control-request-method"))
        })
    }

    pub(crate) fn close_connection(&mut self) {
        self.rewrite_headers(
            &["connection:", "proxy-connection:", "keep-alive:"],
            &["Connection: close"],
        );
    }

    pub(crate) fn allow_cors(&mut self) {
        self.rewrite_headers(
            &[
                "connection:",
                "proxy-connection:",
                "keep-alive:",
                "access-control-allow-origin:",
                "access-control-allow-methods:",
                "access-control-allow-headers:",
                "access-control-allow-credentials:",
                "access-control-expose-headers:",
                "access-control-max-age:",
            ],
            &[
                "Access-Control-Allow-Origin: *",
                "Access-Control-Allow-Methods: GET, POST, PUT, PATCH, DELETE, OPTIONS",
                "Access-Control-Allow-Headers: *",
                "Access-Control-Expose-Headers: *",
                "Connection: close",
            ],
        );
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    fn rewrite_headers(&mut self, remove: &[&str], add: &[&str]) {
        let Some(end) = header_end(&self.bytes) else {
            return;
        };
        let Ok(head) = std::str::from_utf8(&self.bytes[..end - 4]) else {
            return;
        };

        let mut rewritten = Vec::with_capacity(self.bytes.len() + 256);
        for (index, line) in head.split("\r\n").enumerate() {
            if index != 0 {
                let lower = line.to_ascii_lowercase();
                if remove.iter().any(|name| lower.starts_with(name)) {
                    continue;
                }
            }

            rewritten.extend_from_slice(line.as_bytes());
            rewritten.extend_from_slice(b"\r\n");
        }

        for line in add {
            rewritten.extend_from_slice(line.as_bytes());
            rewritten.extend_from_slice(b"\r\n");
        }

        rewritten.extend_from_slice(b"\r\n");
        rewritten.extend_from_slice(&self.bytes[end..]);
        self.bytes = rewritten;
    }
}

fn header_end(bytes: &[u8]) -> Option<usize> {
    bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .map(|position| position + 4)
}
