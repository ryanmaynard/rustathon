use axum::{extract::Extension, Json, Router, routing::post};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use oauth2::{AuthUrl, AuthorizationCode, ClientId, ClientSecret, TokenUrl};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
struct GoogleTokenResponse {
    access_token: String,
    id_token: String,
}

async fn google_oauth_callback(client: Client, code: String) -> Json<GoogleTokenResponse> {
    // Exchange the code for a token
    // TODO: Implement the exchange logic
    unimplemented!()
}

async fn login() -> Json<GoogleTokenResponse> {
    // Redirect to Google OAuth login
    // TODO: Implement the redirection logic
    unimplemented!()
}

async fn register() -> Json<GoogleTokenResponse> {
    // Handle user registration
    // TODO: Implement the registration logic
    unimplemented!()
}

pub fn create_auth_routes() -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .route("/auth/google/callback", post(google_oauth_callback))
}
