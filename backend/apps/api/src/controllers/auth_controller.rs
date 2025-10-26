use std::sync::Arc;

use axum::{
    Json,
    extract::State,
};
use chrono::{
    Duration,
    Utc,
};
use domain::{
    models::UserEntity,
    ports::domain::AbstractUserService,
    value_objects::Username,
};
use jsonwebtoken::{
    EncodingKey,
    Header,
    encode,
};
use serde::{
    Deserialize,
    Serialize,
};
use telegram_authorizer::TelegramUser;
use utoipa_axum::{
    router::OpenApiRouter,
    routes,
};

use crate::{
    api_errors::ApiError,
    dto::auth_dto::LoginResponse,
};

#[derive(Clone)]
pub struct ApiAuthController {
    pub user_service: Arc<dyn AbstractUserService>,
    pub jwt_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: u32,
    exp: usize,
    iat: usize,
}

impl ApiAuthController {
    pub fn new(user_service: Arc<dyn AbstractUserService>, jwt_secret: String) -> Self {
        Self {
            user_service,
            jwt_secret,
        }
    }

    pub fn router(self, jwt_token: &str) -> OpenApiRouter {
        OpenApiRouter::new()
            .routes(routes!(login))
            .with_state(Arc::new(self))
            .layer(telegram_authorizer::AuthorizationLayer::new_embedded(
                jwt_token,
            ))
    }
}

#[utoipa::path(
    post,
    path = "/login",
    tag = "auth",
    description = "Обменивает tma токен на JWT токен приложения, с метаинформацией о пользователе",
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 401, description = "Unauthorized", body = String)
    )
)]
#[axum_macros::debug_handler]
#[tracing::instrument(skip(controller), err(Debug), ret)]
async fn login(
    State(controller): State<Arc<ApiAuthController>>,
    telegram_user: TelegramUser,
) -> Result<Json<LoginResponse>, ApiError> {
    let user = match controller
        .user_service
        .get_user_by_tg_id(telegram_user.id as i64)
        .await
    {
        Ok(user) => user,
        Err(_) => {
            let username = Username::new(
                telegram_user
                    .username
                    .unwrap_or_else(|| format!("user_{}", telegram_user.id)),
            )
            .map_err(ApiError::from)?;

            let new_user = UserEntity {
                user_id: 0,
                tg_id: telegram_user.id as i64,
                chat_id: telegram_user.id as i64,
                username,
                first_name: Some(telegram_user.first_name),
                second_name: telegram_user.last_name,
            };

            controller.user_service.create_user(&new_user).await?
        },
    };

    // Создаем JWT токен
    let now = Utc::now();
    let expires_at = now + Duration::hours(24); // Токен действителен 24 часа
    let expires_in = 24 * 60 * 60; // В секундах

    let claims = Claims {
        sub: user.user_id,
        exp: expires_at.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    // Генерируем токен
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(controller.jwt_secret.as_bytes()),
    )
    .map_err(|e| ApiError::InternalError(format!("Failed to create token: {e}")))?;

    Ok(Json(LoginResponse {
        access_token: token,
        token_type: "Bearer".to_string(),
        expires_in,
    }))
}
