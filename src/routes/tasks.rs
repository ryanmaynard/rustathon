use axum::{
    extract::{Path, Query},
    Json, Router,
    routing::{get, post, put, delete}
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Task {
    id: Uuid,
    name: String,
    description: String,
    project_id: Uuid,
}

type Database = Arc<Mutex<Vec<Task>>>;

async fn list_tasks(db: Database) -> Json<Vec<Task>> {
    let db = db.lock().await;
    Json(db.clone())
}

async fn create_task(db: Database, Json(new_task): Json<Task>) -> Json<Task> {
    let mut db = db.lock().await;
    let task = Task {
        id: Uuid::new_v4(),
        ..new_task
    };
    db.push(task.clone());
    Json(task)
}

async fn get_task(db: Database, Path(task_id): Path<Uuid>) -> Option<Json<Task>> {
    let db = db.lock().await;
    db.iter().find(|task| task.id == task_id).cloned().map(Json)
}

async fn update_task(db: Database, Path(task_id): Path<Uuid>, Json(updated_task): Json<Task>) -> Option<Json<Task>> {
    let mut db = db.lock().await;
    if let Some(task) = db.iter_mut().find(|task| task.id == task_id) {
        task.name = updated_task.name.clone();
        task.description = updated_task.description.clone();
        task.project_id = updated_task.project_id;
        Some(Json(task.clone()))
    } else {
        None
    }
}

async fn delete_task(db: Database, Path(task_id): Path<Uuid>) -> Option<Json<()>> {
    let mut db = db.lock().await;
    if db.iter().any(|task| task.id == task_id) {
        db.retain(|task| task.id != task_id);
        Some(Json(()))
    } else {
        None
    }
}

pub fn create_task_routes() -> Router {
    let db: Database = Arc::new(Mutex::new(vec![]));
    Router::new()
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", get(get_task).put(update_task).delete(delete_task))
        .layer(axum::AddExtensionLayer::new(db))
}
