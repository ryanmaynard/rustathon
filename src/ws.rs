use axum::{extract::ws::{Message, WebSocket, WebSocketUpgrade}, response::IntoResponse, Router, routing::get};
use std::sync::Arc;
use tokio::sync::broadcast;

async fn ws_handler(ws: WebSocketUpgrade, Extension(tx): Extension<Arc<broadcast::Sender<String>>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx))
}

async fn handle_socket(socket: WebSocket, tx: Arc<broadcast::Sender<String>>) {
    // TODO: Implement WebSocket handling
    unimplemented!()
}

pub fn create_ws_routes() -> Router {
    Router::new().route("/ws", get(ws_handler))
}
