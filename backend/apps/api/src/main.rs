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
    ServiceExt,
    extract::Request,
    middleware,
};
use controllers::{
    ApiActiveController,
    ApiAuthController,
    ApiNotificationController,
    ApiUserController,
};
use http::{
    HeaderName, HeaderValue, Method
};
use middlewares::jwt_middleware::JwtAuth;
use shaku::HasComponent;
use tower_http::cors::CorsLayer;
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
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let settings = Settings::new().unwrap();
    let jwt_token = settings.clone().telegram_bot_token;
    let module = di_domain_module(settings.clone()).await;

    let user_controller = ApiUserController::new(module.resolve());
    let auth_controller = ApiAuthController::new(module.resolve(), jwt_token.clone());
    let active_controller = ApiActiveController::new(module.resolve());
    let notification_controller = ApiNotificationController::new(module.resolve());

    #[derive(utoipa::OpenApi)]
    #[openapi(
        modifiers(&SecurityAddon),
        tags(
            (name = "active", description = "Todo items management API")
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
    let jwt_auth = Arc::new(JwtAuth::new(jwt_token.clone()));
    let protected_router = OpenApiRouter::new()
        .nest("/actives", active_controller.router())
        .nest("/notifications", notification_controller.router())
        .nest("/users", user_controller.router())
        .layer(middleware::from_fn_with_state(
            jwt_auth.clone(),
            middlewares::jwt_middleware::jwt_middleware,
        ));
    let auth_router = OpenApiRouter::new().nest("/auth", auth_controller.router(&jwt_token));

    let (router, api): (axum::Router, OpenApi) =
        OpenApiRouter::with_openapi(<ApiDoc as utoipa::OpenApi>::openapi())
            .nest("/api/v1", protected_router)
            .nest("/api/v1", auth_router)
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

    let router = router
        .layer(cors)
        .merge(SwaggerUi::new("/swagger/").url("/api-docs/openapi.json", api));

    // let router = NormalizePathLayer::trim_trailing_slash().layer(router);

    let addr = SocketAddr::from(([0, 0, 0, 0], settings.api_server_port));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    tracing::info!("Server runs on http://{addr}");

    axum::serve(listener, ServiceExt::<Request>::into_make_service(router))
        .await
        .unwrap();
}
