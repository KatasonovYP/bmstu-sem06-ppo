use std::time::Duration;

use axum::{
    body::Body,
    http::{
        header,
        Request,
        StatusCode,
    },
    middleware::Next,
    response::Response,
};

pub async fn tma_middleware(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let tma_init_data = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("tma "))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    let exp_in = Duration::from_secs(24 * 60 * 60);
    let bot_token = std::env::var("TELOXIDE_TOKEN").unwrap();

    let result = tma_init_data::validate(tma_init_data, bot_token.as_str(), exp_in);

    tracing::trace!("auth result: {result:?}");

    let response = next.run(request).await;

    Ok(response)
}
