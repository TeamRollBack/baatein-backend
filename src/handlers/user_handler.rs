use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{repositories::user_repo::{self, UserRepo}, AppState};


#[derive(Debug, Serialize, Deserialize)]
pub struct UserRequest {
    username: String,
    first_name: String,
    last_name: String,
    dob: String,
    gender: char,
    phone: String,
    email: String,
    password: String,
}

pub async fn add_user(State(shared_state): State<Arc<AppState>>) -> (StatusCode, String) {

    let u = user_repo::User {
        first_name: "Rohit".to_string(),
        last_name: "Mokashi".to_string(),
        username: "rohitmokashi".to_string(),
        gender: user_repo::Gender::Male,
        dob: "2003-06-12".to_string(),
    };

    shared_state.user_repo.add_user(u).await;
    println!("adlfjdsfjk");

    (StatusCode::OK, "user added".to_string())

}
