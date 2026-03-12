pub fn response_text(result: Result<ureq::Response, ureq::Error>) -> String {
    match result {
        Ok(resp) => resp.into_string().expect("read response body"),
        Err(ureq::Error::Transport(err)) => panic!("http request failed: {err}"),
        Err(ureq::Error::Status(code, _)) => panic!("http request failed: status {code}"),
    }
}

pub fn get_text(url: &str) -> String {
    response_text(ureq::get(url).call())
}

pub fn post_text(url: &str, body: &[u8]) -> String {
    response_text(ureq::post(url).send_bytes(body))
}
