use axum::{
    Json,
    response::IntoResponse
};

use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
pub struct User {
    email : String,
    password: String,
}

pub async fn create_user(Json(user_payload) : Json<User>) -> impl IntoResponse{
    ()
}

pub async fn log_in(Json(user_payload) : Json<User>) -> impl IntoResponse{
    ()
}

