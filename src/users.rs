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


pub enum UserCreationError {
    EmailAlreadyInUse
}

pub enum UserLogInError {
    NoMatchingRecord,
    IncorrectPasswordOrEmail,
}

pub enum UserRequestResponse<ErrorT>{
    Success,
    Fail(ErrorT)
}


impl IntoResponse for UserRequestResponse<UserCreationError>{
    fn into_response(self) -> axum::response::Response {

        use axum::http::StatusCode;

        match self {
            Self::Success => (StatusCode::CREATED, "Created_user").into_response(),
            Self::Fail(error) => {
                
                let error_message = match error {
                    UserCreationError::EmailAlreadyInUse => "This email is already in use"
                };

                (StatusCode::INTERNAL_SERVER_ERROR,error_message).into_response()
            }
        }
    }
}

impl IntoResponse for UserRequestResponse<UserLogInError> {

    fn into_response(self) -> axum::response::Response {

        use axum::http::StatusCode;

        match self {
            Self::Success => (StatusCode::OK, "Logged in User").into_response(),
            Self::Fail(error) => {
                
                let error_message = match error {
                    UserLogInError::NoMatchingRecord => "No matching user with provided email",
                    UserLogInError::IncorrectPasswordOrEmail => "Incorrect Password or Email",
                };

                (StatusCode::INTERNAL_SERVER_ERROR,error_message).into_response()
            }
        }
    }
}


//TODO add button delay on frontend
#[axum_macros::debug_handler]
pub async fn create_user(Json(user_payload): Json<SentUser>) -> Result<UserRequestResponse<UserCreationError>, crate::ReqwestWrapper> {

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

        client
        .from("users")
        .insert(new_user)
        .execute()
        .await?
        .error_for_status()?;

        Ok(UserRequestResponse::Success)

    } else {
        Ok(UserRequestResponse::Fail(UserCreationError::EmailAlreadyInUse))
    }

}

pub async fn log_in(Json(user_payload) : Json<SentUser>) -> Result<UserRequestResponse<UserLogInError>, crate::ReqwestWrapper>{

    let client = 
        Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").
        insert_header("apikey", std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"));

    let users_with_this_email : Vec<User> = 
        client
        .from("users")
        .eq("email", user_payload.email)
        .execute()
        .await?
        .error_for_status()?
        .json()
        .await?;

    if let Some(user_with_email) = users_with_this_email.first() {
        //                                                 Yes i feel bad 
        if verify(user_payload.password, &user_with_email.password).unwrap() {
            Ok(UserRequestResponse::Success)
        } else {
            Ok(UserRequestResponse::Fail(UserLogInError::IncorrectPasswordOrEmail))
        }

    } else {
        Ok(UserRequestResponse::Fail(UserLogInError::NoMatchingRecord))
    }

}
