use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{repositories::message_repo::Message, AppState};

pub async fn create_message(
    State(shared_state): State<Arc<AppState>>,
    Json(message): Json<Message>,
) -> (StatusCode, Json<String>) {

    let uid = message.sender;
    if shared_state.user_repo.uid_exists(uid).await {
        shared_state.message_repo.create_message(message).await;
        (StatusCode::OK, Json("message created".to_string()))
    } else {
        (StatusCode::CONFLICT, Json("message not created".to_string()))
    }

}
