mod db;
mod handlers;
mod models;
mod repository;
mod schema;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let db_pool = db::establish_connection_pool();

    let app = Router::new()
        .route("/users", get(handlers::list_users))
        .route("/users", post(handlers::create_user))
        .route("/users/{id}", get(handlers::get_user))
        .route("/users/{id}", put(handlers::update_user))
        .route("/users/{id}", delete(handlers::delete_user))
        .with_state(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server runs on port {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
