use postgrest::Postgrest;
use axum::{
    Json,
    extract::Multipart,
    response::IntoResponse
};
use axum_typed_multipart::
{TryFromMultipart,
TypedMultipart
};
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize, TryFromMultipart)]
pub struct User {
    email : String,
    password: String,
}

pub async fn create_user(TypedMultipart(User{ email, password }): TypedMultipart<User>) -> Json<User>{

    let client = Postgrest::new("https://hgioigecbrqawyedynet.supabase.co").
        insert_header("apikey", std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"));

    let users_with_this_email = 
        client
        .from("users")
        .select("*")
        .execute()
        .await;

    println!("{:?}", users_with_this_email.unwrap().text().await);

    Json(User{ email, password})
}

pub async fn log_in(TypedMultipart(User{ email, password }): TypedMultipart<User>) -> impl IntoResponse{

}

