mod app;
mod domain;
mod ports;

use app::{
    adapters::db::{
        connection::establish_connection_pool,
        postgres_active_repository::PostgresActiveRepository,
        postgres_notification_repository::PostgresNotificationRepository,
        postgres_user_repository::PostgresUserRepository,
    },
    controllers::api::{
        active_controller::ActiveController, health_controller::HealthController,
        notification_controller::NotificationController, user_controller::UserController,
    },
};
use axum::{
    body::{Body, Bytes},
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Router,
};
use domain::services::{
    active_service::ActiveService, notification_service::NotificationService,
    user_service::UserService,
};
use std::{net::SocketAddr, sync::Arc, time::Instant};

use http_body_util::BodyExt;
use tracing::{debug, info};

// Middleware для логирования запросов в структурированном формате
async fn log_request(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Извлекаем нужную информацию из запроса
    let path = request.uri().path().to_owned();
    let method = request.method().clone();

    // Логируем заголовки
    let headers = format!("{:#?}", request.headers());

    // Разделяем запрос на части
    let (parts, body) = request.into_parts();

    // Собираем тело запроса
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(e) => {
            debug!("Failed to read request body: {}", e);
            Bytes::new() // Пустые байты если не удалось прочитать
        }
    };

    // Пытаемся преобразовать тело в строку
    let body_str = match String::from_utf8(bytes.clone().to_vec()) {
        Ok(s) => s,
        Err(_) => format!("<binary data: {} bytes>", bytes.len()),
    };

    // Восстанавливаем запрос с тем же телом
    let request = Request::from_parts(parts, Body::from(bytes));

    // Отмечаем начало запроса
    let start = Instant::now();

    // Обрабатываем запрос
    let response = next.run(request).await;

    // Логируем информацию о запросе
    let latency = start.elapsed();
    let status = response.status();

    info!(
        status = status.as_u16(),
        method = method.as_str(),
        path = path,
        latency = format!("{latency:?}"),
        headers = headers,
        body = body_str,
    );

    Ok(response)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        // .json()
        .pretty()
        .with_max_level(tracing::Level::TRACE)
        .with_level(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    info!("logger inited successfully");
    let user_repo = Arc::new(PostgresUserRepository::new(establish_connection_pool()));
    info!("PostgresUserRepository inited successfully");
    let user_service = Arc::new(UserService::new(user_repo));
    info!("UserService inited successfully");
    let user_controller = UserController::new(user_service);
    info!("UserController inited successfully");

    let active_repo = Arc::new(PostgresActiveRepository::new(establish_connection_pool()));
    let active_service = Arc::new(ActiveService::new(active_repo));
    let active_controller = ActiveController::new(active_service);

    let notification_repo = Arc::new(PostgresNotificationRepository::new(
        establish_connection_pool(),
    ));
    let notification_service = Arc::new(NotificationService::new(notification_repo));
    let notification_controller: NotificationController = NotificationController::new(notification_service);

    // Создаем базовый маршрутизатор без middleware
    let app = Router::new()
        .route("/ping/", get(HealthController::get_ping))
        .route("/users/{user_id}/", get(UserController::get_user))
        .route("/users/", post(UserController::create_user))
        .with_state(user_controller)
        .route("/actives/{id}/", get(ActiveController::get_active))
        .route("/actives/", post(ActiveController::create_active))
        .with_state(active_controller)
        .route(
            "/notifications/{id}/",
            get(NotificationController::get_notification),
        )
        .route(
            "/notifications/",
            post(NotificationController::create_notification),
        )
        .with_state(notification_controller);

    // Добавляем middleware для логирования
    let app = app.layer(middleware::from_fn(log_request));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Server runs on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
