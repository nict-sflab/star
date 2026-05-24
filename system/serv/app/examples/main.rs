use star_app::{all_prefix, build_server};
use std::env;
use std::thread;
use tiny_http::{Header, Response, Server as TinyServer};

const HTML_TEMPLATE: &str = include_str!("html_template.html");

fn render_html(title: &str, headline: &str, body: &str) -> String {
    HTML_TEMPLATE
        .replace("{{title}}", title)
        .replace("{{headline}}", headline)
        .replace("{{body}}", body)
}

fn spawn_upstream(bind_addr: &str) -> (String, thread::JoinHandle<()>) {
    let server = TinyServer::http(bind_addr).expect("bind upstream");
    let addr = server
        .server_addr()
        .to_ip()
        .expect("upstream should bind to IP")
        .to_string();
    let handle = thread::spawn(move || {
        while let Ok(request) = server.recv() {
            let response = Response::from_string("OK");
            let _ = request.respond(response);
        }
    });
    (addr, handle)
}

fn spawn_html_server(bind_addr: &str) -> (String, thread::JoinHandle<()>) {
    let server = TinyServer::http(bind_addr).expect("bind html server");
    let addr = server
        .server_addr()
        .to_ip()
        .expect("html server should bind to IP")
        .to_string();
    let handle = thread::spawn(move || {
        while let Ok(request) = server.recv() {
            let body = render_html(
                "tiny_http HTML",
                "Hello from tiny_http",
                "This response is HTML.",
            );
            let mut response = Response::from_string(body);
            let header = Header::from_bytes(b"Content-Type", b"text/html; charset=utf-8")
                .expect("build content-type header");
            response.add_header(header);
            let _ = request.respond(response);
        }
    });
    (addr, handle)
}

fn main() {
    let api_addr = env_or_default("STAR_API_ADDR", "0.0.0.0:8080");
    let proxy_addr = env_or_default("STAR_PROXY_ADDR", "0.0.0.0:18081");
    let upstream_bind_addr = env_or_default("STAR_EXAMPLE_UPSTREAM_ADDR", "127.0.0.1:3000");
    let html_bind_addr = env_or_default("STAR_EXAMPLE_HTML_ADDR", "127.0.0.1:3001");

    let (upstream_addr, _upstream_handle) = spawn_upstream(&upstream_bind_addr);
    let (_html_addr, _html_handle) = spawn_html_server(&html_bind_addr);
    let server = build_server(
        &api_addr,
        &proxy_addr,
        &upstream_addr,
        all_prefix(1),
        1,
    );
    server.unwrap().run_forever();
}

fn env_or_default(name: &str, default: &str) -> String {
    env::var(name).unwrap_or_else(|_| default.to_string())
}
