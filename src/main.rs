use std::sync::Arc;

use axum::{http::StatusCode, routing::{get, post}, Router};
use handlers::user_handler::add_user;
use repositories::user_repo::UserRepo;

mod handlers;
mod repositories;

struct AppState {
    user_repo: UserRepo,
}

async fn say_hello() -> (StatusCode, String) {
    (StatusCode::OK, "Hello World!".to_string())
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        user_repo: UserRepo::init().await.unwrap(),
    });

    // build our application with a single route
    let app = Router::new()
        .route("/", get(say_hello))
        .route("/user/add", post(add_user))
        .with_state(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
