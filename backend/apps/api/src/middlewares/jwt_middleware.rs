// src/middlewares/jwt_middleware.rs
use std::sync::Arc;

use axum::{
    body::Body,
    extract::State,
    http::{
        Request,
        StatusCode,
    },
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{
    Algorithm,
    DecodingKey,
    Validation,
    decode,
};
use secrecy::{ExposeSecret, SecretString};
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u32,
    pub exp: usize,
    pub iat: usize,
}

pub struct JwtAuth {
    pub jwt_secret: SecretString,
}

impl JwtAuth {
    pub fn new(jwt_secret: SecretString) -> Self {
        Self { jwt_secret }
    }
}

#[tracing::instrument(skip(jwt_auth), err(Debug), ret)]
pub async fn jwt_middleware(
    State(jwt_auth): State<Arc<JwtAuth>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Получаем токен из заголовка авторизации
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Проверяем формат токена
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Проверяем валидность токена
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_auth.jwt_secret.expose_secret().as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Добавляем user_id в запрос для дальнейшего использования
    request.extensions_mut().insert(token_data.claims.sub);

    let response = next.run(request).await;
    Ok(response)
}
