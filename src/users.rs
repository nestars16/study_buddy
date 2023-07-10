use postgrest::Postgrest;
use axum::{
    Json,
    response::IntoResponse
};

use axum_typed_multipart::
{TryFromMultipart,
TypedMultipart
};
use serde::{Serialize,Deserialize};

#[derive(Serialize, TryFromMultipart)]
pub struct SentUser {
    email : String,
    password: String,
}

#[derive(Deserialize,Debug)]
pub struct User {
    id : uuid::Uuid,
    email: String,
    password : String,
}

pub struct UserCreationResponse {
    success : bool,
    reason : Option<String>,
}


impl UserCreationResponse {

}

impl IntoResponse for UserCreationResponse {
    fn into_response(self) -> axum::response::Response {

        let status_code = match self.success {
            false => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            true => axum::http::StatusCode::OK
        };

        let reason = match self.reason {
            Some(reason) => reason,
            None => "Request sucessful".to_string()
        };

        (status_code,reason).into_response()
    }
}

#[axum_macros::debug_handler]
pub async fn create_user(TypedMultipart(SentUser{ email, password }): TypedMultipart<SentUser>) -> Result<UserCreationResponse, crate::ReqwestWrapper> {

    let client = 
        Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").
        insert_header("apikey", std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"));

    let users_with_this_email : Vec<User> = 
        client
        .from("users")
        .eq("email", email)
        .execute()
        .await?.
        json()
        .await?;

    if !users_with_this_email.is_empty() {
        
    } else {

    }
}

pub async fn log_in(TypedMultipart(SentUser{ email, password }): TypedMultipart<SentUser>) -> impl IntoResponse{

}

