use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

const READ_BUF_BYTES: usize = 4_096;
const PENDING_BUF_BYTES: usize = 8_192;
const OK_RESPONSE: &[u8] = b"HTTP/1.1 200 OK\r\ncontent-length: 2\r\nconnection: close\r\n\r\nOK";

fn find_headers_end(buf: &[u8]) -> Option<usize> {
    buf.windows(4).position(|window| window == b"\r\n\r\n")
}

fn handle_ok_connection(mut stream: TcpStream) {
    let _ = stream.set_nodelay(true);
    let mut read_buf = [0u8; READ_BUF_BYTES];
    let mut pending = Vec::with_capacity(PENDING_BUF_BYTES);

    loop {
        let read = match stream.read(&mut read_buf) {
            Ok(0) => return,
            Ok(n) => n,
            Err(_) => return,
        };
        pending.extend_from_slice(&read_buf[..read]);

        if find_headers_end(&pending).is_none() {
            continue;
        }

        if stream.write_all(OK_RESPONSE).is_err() {
            return;
        }
        let _ = stream.flush();
        return;
    }
}

pub fn reserve_local_addr() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("reserve local port");
    let addr = listener.local_addr().expect("read reserved local port");
    drop(listener);
    addr.to_string()
}

pub fn spawn_ok_server(
    bind_addr: &str,
    accept_error_label: &'static str,
) -> (String, thread::JoinHandle<()>) {
    let listener = TcpListener::bind(bind_addr)
        .unwrap_or_else(|err| panic!("bind {accept_error_label}: {err}"));
    let addr = listener
        .local_addr()
        .expect("read benchmark HTTP server address")
        .to_string();

    let handle = thread::spawn(move || {
        for incoming in listener.incoming() {
            match incoming {
                Ok(stream) => handle_ok_connection(stream),
                Err(err) => eprintln!("{accept_error_label} accept error: {err}"),
            }
        }
    });

    (addr, handle)
}
