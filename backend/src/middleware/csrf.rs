use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn csrf_protect(req: Request<Body>, next: Next) -> Response {
    // This is a simplified placeholder CSRF check:
    let csrf_header = req.headers().get("x-csrf-token");

    if let Some(token) = csrf_header {
        if token == "secure-static-token" {
            // ✅ Valid token
            next.run(req).await
        } else {
            // ❌ Invalid token
            Response::builder()
                .status(StatusCode::FORBIDDEN)
                .body(Body::from("Invalid CSRF token"))
                .unwrap()
        }
    } else {
        // ❌ No token
        Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("Missing CSRF token"))
            .unwrap()
    }
}
