mod api_errors;
mod controllers;
mod dto;
mod middlewares;

use std::{
    net::SocketAddr,
    sync::Arc,
};

use adapters::{
    di_domain_module::di_domain_module,
    settings::Settings,
};
use axum::{
    Json,
    ServiceExt,
    extract::Request,
    middleware,
    routing::{
        get,
        options,
    },
};
use controllers::{
    ApiActiveController,
    ApiAuthController,
    ApiHealthController,
    ApiNotificationController,
    ApiUserController,
    global_options_handler,
    not_found_handler,
};
use http::{
    HeaderName,
    HeaderValue,
    Method,
};
use middlewares::jwt_middleware::JwtAuth;
use shaku::HasComponent;
use tower_http::{
    cors::CorsLayer,
    normalize_path::NormalizePathLayer,
    trace::TraceLayer,
};
use tower_layer::Layer;
use utoipa::{
    Modify,
    openapi::{
        OpenApi,
        security::{
            ApiKey,
            ApiKeyValue,
            SecurityScheme,
        },
    },
};
use utoipa_axum::router::OpenApiRouter;

#[derive(utoipa::OpenApi)]
#[openapi(
        info(
            title = "Stocks Tracker API",
            license(
                identifier = "MIT"
            )
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "active", description = "CRUD операции над активами пользователя"),
            (name = "notification", description = "CRUD операции над нотификациями пользователя"),
            (name = "user", description = "CRUD операции над пользователями"),
            (name = "auth", description = "Операции, связанные с авторизацией и аутентификацией"),
        )
    )]
struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "Authorization",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("Authorization"))),
            )
        }
    }
}

#[tokio::main]
async fn main() {
    let settings = Settings::new("config/app.default.yaml").unwrap();
    let jwt_token = settings.clone().telegram_bot_token;
    let module = di_domain_module(settings.clone()).await;

    let user_controller = ApiUserController::new(module.resolve());
    let auth_controller = ApiAuthController::new(module.resolve(), jwt_token.clone());
    let active_controller = ApiActiveController::new(module.resolve());
    let notification_controller = ApiNotificationController::new(module.resolve());
    let health_controller = ApiHealthController::new();

    let jwt_auth = Arc::new(JwtAuth::new(jwt_token.clone()));

    let protected_router = OpenApiRouter::new()
        .nest("/actives", active_controller.router())
        .nest("/notifications", notification_controller.router())
        .nest("/users", user_controller.router())
        .layer(middleware::from_fn_with_state(
            jwt_auth.clone(),
            middlewares::jwt_middleware::jwt_middleware,
        ));

    let public_router = OpenApiRouter::new()
        .nest("/auth", auth_controller.router(&jwt_token))
        .nest("/health", health_controller.router());

    let api_v1_router = OpenApiRouter::new()
        .merge(protected_router)
        .merge(public_router);

    let (app_router, api): (axum::Router, OpenApi) =
        OpenApiRouter::with_openapi(<ApiDoc as utoipa::OpenApi>::openapi())
            .nest("/api/v1", api_v1_router)
            .split_for_parts();

    let origins = settings
        .api_cors_origins
        .iter()
        .filter_map(|origin| HeaderValue::from_str(origin).ok())
        .collect::<Vec<HeaderValue>>();

    let cors = CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ])
        .allow_credentials(true);

    let router = axum::Router::<()>::new()
        .merge(app_router)
        .route("/api/v1", get(|| async { Json(api) }))
        .route("/{*path}", options(global_options_handler))
        .fallback(not_found_handler)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let router = NormalizePathLayer::trim_trailing_slash().layer(router);

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.api_server_port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server runs on http://{addr}");

    axum::serve(listener, ServiceExt::<Request>::into_make_service(router))
        .await
        .unwrap();
}
