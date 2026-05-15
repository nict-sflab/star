use star_app::{all_prefix, build_server};
use std::thread;
use tiny_http::{Header, Response, Server as TinyServer};

const HTML_TEMPLATE: &str = include_str!("html_template.html");

fn render_html(title: &str, headline: &str, body: &str) -> String {
    HTML_TEMPLATE
        .replace("{{title}}", title)
        .replace("{{headline}}", headline)
        .replace("{{body}}", body)
}

fn spawn_upstream() -> (String, thread::JoinHandle<()>) {
    let server = TinyServer::http("127.0.0.1:3000").expect("bind upstream");
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

fn spawn_html_server() -> (String, thread::JoinHandle<()>) {
    let server = TinyServer::http("127.0.0.1:3001").expect("bind html server");
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
    let (upstream_addr, _upstream_handle) = spawn_upstream();
    let (_html_addr, _html_handle) = spawn_html_server();
    let server = build_server(
        "127.0.0.1:8080",
        "127.0.0.1:8081",
        &upstream_addr,
        all_prefix(1),
        1,
    );
    server.unwrap().run_forever();
}
