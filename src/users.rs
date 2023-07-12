use postgrest::Postgrest;
use axum::{
    Json,
    response::IntoResponse
};

use serde::{Serialize,Deserialize};
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Serialize, Deserialize, Debug)]
pub struct SentUser {
    email : String,
    password: String,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct User {
    id : uuid::Uuid,
    email: String,
    password : String,
}

impl User {
    fn new(email : String, password:  String) -> Self {
        let password = hash(password, DEFAULT_COST).expect("All passwords should hash");
        User{id : uuid::Uuid::new_v4(),email,password}
    }
}

impl From<User> for String {
    fn from(value: User) -> Self {
        dbg!(serde_json::to_string(&value).expect("Type is serializable"))
    }
}


pub enum UserCreationResponseError {
    EmailAlreadyInUse
}

pub enum UserCreationResponse {
    Success,
    Fail(UserCreationResponseError)
}

impl IntoResponse for UserCreationResponse {
    fn into_response(self) -> axum::response::Response {

        use axum::http::StatusCode;

        match self {
            Self::Success => (StatusCode::CREATED, "Created_user").into_response(),
            Self::Fail(error) => {
                
                let error_message = match error {
                    UserCreationResponseError::EmailAlreadyInUse => "This email is already in use"
                };

                (StatusCode::INTERNAL_SERVER_ERROR,error_message).into_response()
            }
        }

    }
}


#[axum_macros::debug_handler]
pub async fn create_user(Json(user_payload): Json<SentUser>) -> Result<UserCreationResponse, crate::ReqwestWrapper> {

    let client = 
        Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").
        insert_header("apikey", std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"));

    let users_with_this_email : Vec<User> = 
        client
        .from("users")
        .eq("email", user_payload.email.clone())
        .execute()
        .await?
        .error_for_status()?
        .json()
        .await?;

    if users_with_this_email.is_empty() {

        let new_user = User::new(user_payload.email, user_payload.password);

        dbg!(client
        .from("users")
        .insert(new_user)
        .execute()
        .await?
        .error_for_status()?
        );

        Ok(UserCreationResponse::Success)

    } else {
        Ok(UserCreationResponse::Fail(UserCreationResponseError::EmailAlreadyInUse))
    }

}

pub async fn log_in(Json(user_payload) : Json<SentUser>) -> impl IntoResponse {
}
