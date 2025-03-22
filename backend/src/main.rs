mod ports;
mod app;
mod domain;

use app::{
    adapters::db::{connection::establish_connection_pool, 
    postgres_user_repository::PostgresUserRepository}, 
    controllers::api::user_controller::UserController,
    controllers::api::health_controller::HealthController,
};
use axum::{
    routing::{get, post},
    Router,
};
use domain::services::user_service::UserService;
use std::{net::SocketAddr, sync::Arc};

#[tokio::main]
async fn main() {
    let db_pool = establish_connection_pool();
    let user_repo = Arc::new(PostgresUserRepository::new(db_pool));
    
    let user_service = UserService::new(user_repo);
    let user_controller = UserController::new(user_service);


    let app = Router::new()
        .route("/ping/", get(HealthController::get_ping))
        .route("/users/{id}/", get(UserController::get_user))
        .route("/users/", post(UserController::create_user))
        .with_state(user_controller);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server runs on port {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
