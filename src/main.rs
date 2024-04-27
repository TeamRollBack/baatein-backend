use std::sync::Arc;

use axum::{http::StatusCode, routing::{get, post}, Router};
use handlers::{chat_handler::{create_chat, send_message}, message_handler::create_message, user_handler::add_user};
use repositories::{chat_repo::ChatRepo, message_repo::MessageRepo, user_repo::UserRepo};

mod handlers;
mod repositories;
mod db;

struct AppState {
    user_repo: UserRepo,
    message_repo: MessageRepo,
    chat_repo: ChatRepo,
}

async fn say_hello() -> (StatusCode, String) {
    (StatusCode::OK, "Hello World!".to_string())
}

#[tokio::main]
async fn main() {

    let baatein_db = db::DB::init().await.unwrap();

    let shared_state = Arc::new(AppState {
        user_repo: UserRepo::init(baatein_db.clone()).await.unwrap(),
        message_repo: MessageRepo::init(baatein_db.clone()).await.unwrap(),
        chat_repo: ChatRepo::init(baatein_db.clone()).await.unwrap(),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(say_hello))
        .route("/user/add", post(add_user))
        .route("/msg/create", post(create_message))
        .route("/chat/create", post(create_chat))
        .route("/sendmsg", post(send_message))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
