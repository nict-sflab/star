use std::sync::Arc;

use pingora_web::{
    Handler, Middleware, PingoraHttpRequest, PingoraWebHttpResponse, StatusCode, WebError,
};

#[derive(Clone, Default)]
pub struct Cors;

impl Cors {
    fn apply_headers(
        res: &mut PingoraWebHttpResponse,
        allow_methods: Option<&str>,
        allow_headers: Option<&str>,
    ) {
        res.set_header("access-control-allow-origin", "*");
        res.set_header(
            "access-control-allow-methods",
            allow_methods.unwrap_or("GET, POST, OPTIONS"),
        );
        res.set_header(
            "access-control-allow-headers",
            allow_headers.unwrap_or("*"),
        );
        res.set_header("access-control-max-age", "86400");
    }

    fn cors_values(req: &PingoraHttpRequest) -> (Option<String>, Option<String>) {
        let allow_methods = req
            .headers()
            .get("access-control-request-method")
            .and_then(|v| v.to_str().ok())
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string());
        let allow_headers = req
            .headers()
            .get("access-control-request-headers")
            .and_then(|v| v.to_str().ok())
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string());
        (allow_methods, allow_headers)
    }
}

#[async_trait::async_trait]
impl Middleware for Cors {
    async fn handle(
        &self,
        req: PingoraHttpRequest,
        next: Arc<dyn Handler>,
    ) -> Result<PingoraWebHttpResponse, WebError> {
        let (allow_methods, allow_headers) = Self::cors_values(&req);
        if req.method().as_str() == "OPTIONS" {
            let mut res = PingoraWebHttpResponse::empty(StatusCode::NO_CONTENT);
            Self::apply_headers(&mut res, allow_methods.as_deref(), allow_headers.as_deref());
            return Ok(res);
        }

        let mut res = next.handle(req).await?;
        Self::apply_headers(&mut res, allow_methods.as_deref(), allow_headers.as_deref());
        Ok(res)
    }
}
