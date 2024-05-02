use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};

use crate::{
    repositories::{
        chat_repo::{ChatRequest, Participants},
        message_repo::{Message, MessageRequest},
    },
    AppState,
};

pub async fn create_chat(
    State(shared_state): State<Arc<AppState>>,
    Json(participants): Json<Participants>,
) -> (StatusCode, Json<String>) {
    shared_state.chat_repo.create_chat(participants).await;

    (StatusCode::OK, Json("chat created".to_string()))
}

pub async fn send_message(
    State(shared_state): State<Arc<AppState>>,
    Json(message_request): Json<MessageRequest>,
) -> (StatusCode, Json<String>) {
    let message = Message {
        sender: message_request.sender,
        message: message_request.message,
    };
    let participants = Participants {
        p1: message_request.sender,
        p2: message_request.reciever,
    };
    let msg_id = shared_state.message_repo.create_message(message).await;
    shared_state.chat_repo.add_msg(participants, msg_id).await;

    (StatusCode::OK, Json("message sent".to_string()))
}

pub async fn get_chats(
    State(shared_state): State<Arc<AppState>>,
    Json(chat_req): Json<ChatRequest>
) -> (StatusCode, Json<Vec<Message>>) {

    let participants = Participants {
        p1: shared_state.user_repo.get_user_oid(chat_req.u1).await,
        p2: shared_state.user_repo.get_user_oid(chat_req.u2).await
    };

    let chats = shared_state.chat_repo.get_chats(participants).await;
    let mut messages: Vec<Message> = Vec::new();

    for chat in chats {
        messages.push(shared_state.message_repo.get_msg_by_id(chat).await);
    }

    (StatusCode::OK, Json(messages))
}
