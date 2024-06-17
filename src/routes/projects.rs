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
struct Project {
    id: Uuid,
    name: String,
    description: String,
}

type Database = Arc<Mutex<Vec<Project>>>;

async fn list_projects(db: Database) -> Json<Vec<Project>> {
    let db = db.lock().await;
    Json(db.clone())
}

async fn create_project(db: Database, Json(new_project): Json<Project>) -> Json<Project> {
    let mut db = db.lock().await;
    let project = Project {
        id: Uuid::new_v4(),
        ..new_project
    };
    db.push(project.clone());
    Json(project)
}

async fn get_project(db: Database, Path(project_id): Path<Uuid>) -> Option<Json<Project>> {
    let db = db.lock().await;
    db.iter().find(|project| project.id == project_id).cloned().map(Json)
}

async fn update_project(db: Database, Path(project_id): Path<Uuid>, Json(updated_project): Json<Project>) -> Option<Json<Project>> {
    let mut db = db.lock().await;
    if let Some(project) = db.iter_mut().find(|project| project.id == project_id) {
        project.name = updated_project.name.clone();
        project.description = updated_project.description.clone();
        Some(Json(project.clone()))
    } else {
        None
    }
}

async fn delete_project(db: Database, Path(project_id): Path<Uuid>) -> Option<Json<()>> {
    let mut db = db.lock().await;
    if db.iter().any(|project| project.id == project_id) {
        db.retain(|project| project.id != project_id);
        Some(Json(()))
    } else {
        None
    }
}

pub fn create_project_routes() -> Router {
    let db: Database = Arc::new(Mutex::new(vec![]));
    Router::new()
        .route("/projects", get(list_projects).post(create_project))
        .route("/projects/:id", get(get_project).put(update_project).delete(delete_project))
        .layer(axum::AddExtensionLayer::new(db))
}
