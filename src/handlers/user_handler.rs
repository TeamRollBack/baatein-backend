use std::sync::Arc;
use axum::{extract::State, http::StatusCode, Json};

use crate::{repositories::user_repo::User, AppState};

pub async fn add_user(
    State(shared_state): State<Arc<AppState>>,
    Json(user): Json<User>,
) -> (StatusCode, Json<String>) {
    if shared_state.user_repo.add_user(user).await {
        (
            StatusCode::CREATED,
            Json("User created successfully".to_string()),
        )
    } else {
        (StatusCode::CONFLICT, Json("user already exists".to_string()))
    }
}
