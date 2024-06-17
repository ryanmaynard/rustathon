use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use crate::{config::Config, db::establish_connection, auth::create_auth_routes, routes::{create_project_routes, create_task_routes, create_ws_routes}};
use tokio::sync::broadcast;
use std::sync::Arc;
use tower_http::services::ServeDir;

mod auth;
mod config;
mod db;
mod errors;
mod middleware;
mod models;
mod routes;
mod ws;

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    let connection = establish_connection();

    let (tx, _rx) = broadcast::channel(100);
    let tx = Arc::new(tx);

    let app = Router::new()
        .merge(create_auth_routes())
        .merge(create_project_routes())
        .merge(create_task_routes())
        .merge(create_ws_routes())
        .nest("/static", tower_http::services::ServeDir::new("static"))
        .route("/", get(|| async { "Welcome to Rustathon!" }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
