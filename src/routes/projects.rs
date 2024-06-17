use axum::{extract::{Path, Query}, Extension, Json, Router, routing::{get, post, put, delete}};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct Project {
    id: Uuid,
    name: String,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Task {
    id: Uuid,
    name: String,
    description: String,
    project_id: Uuid,
}

async fn list_projects() -> Json<Vec<Project>> {
    // TODO: Implement list projects
    unimplemented!()
}

async fn create_project(Json(project): Json<Project>) -> Json<Project> {
    // TODO: Implement create project
    unimplemented!()
}

async fn update_project(Path(id): Path<Uuid>, Json(project): Json<Project>) -> Json<Project> {
    // TODO: Implement update project
    unimplemented!()
}

async fn delete_project(Path(id): Path<Uuid>) -> Json<()> {
    // TODO: Implement delete project
    unimplemented!()
}

async fn list_tasks(Query(params): Query<Uuid>) -> Json<Vec<Task>> {
    // TODO: Implement list tasks
    unimplemented!()
}

async fn create_task(Json(task): Json<Task>) -> Json<Task> {
    // TODO: Implement create task
    unimplemented!()
}

async fn update_task(Path(id): Path<Uuid>, Json(task): Json<Task>) -> Json<Task> {
    // TODO: Implement update task
    unimplemented!()
}

async fn delete_task(Path(id): Path<Uuid>) -> Json<()> {
    // TODO: Implement delete task
    unimplemented!()
}

pub fn create_project_routes() -> Router {
    Router::new()
        .route("/projects", get(list_projects).post(create_project))
        .route("/projects/:id", put(update_project).delete(delete_project))
        .route("/tasks", get(list_tasks).post(create_task))
        .route("/tasks/:id", put(update_task).delete(delete_task))
}
