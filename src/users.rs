use postgrest::Postgrest;
use axum::{
    Json,
    response::{Response, IntoResponse}
};

use serde::{Serialize,Deserialize};
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Serialize, Deserialize, Debug)]
pub struct SentUser {
    email : String,
    password: String,
}

#[derive(Deserialize)]
pub struct UuidJson {
    session_id : uuid::Uuid,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct User {
    id : uuid::Uuid,
    email: String,
    password : String,
    session_id : Option<uuid::Uuid>,
}

pub struct Post {
    user_id : uuid::Uuid,
    title : String,
    content: String,
}

impl User {
    fn new(email : String, password:  String) -> Self {
        let password = hash(password, DEFAULT_COST).expect("All passwords should hash");
        User{id : uuid::Uuid::new_v4(),email,password, session_id : Some(uuid::Uuid::new_v4()) }
    }
}

impl From<User> for String {
    fn from(value: User) -> Self {
        serde_json::to_string(&value).expect("Type is serializable")
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
    Success(uuid::Uuid),
    Fail(ErrorT)
}

impl IntoResponse for UserRequestResponse<UserCreationError>{
    fn into_response(self) -> axum::response::Response {

        use axum::http::{StatusCode,header};

        match self {
            Self::Success(id)=>  {

                let headers = [
                    (header::SET_COOKIE, format!("session_id={id}")),
                ];

                (StatusCode::CREATED,headers,"Created user and instantied user session").into_response()    

            }
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

        use axum::http::{StatusCode,header};

        match self {
            Self::Success(id)=>  {

                let headers = [
                    (header::SET_COOKIE, format!("session_id={id}")),
                ];

                (StatusCode::OK,headers,"Created user and instantied user session").into_response()    
            }
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

        let newly_created_user = client
        .from("users")
        .insert(new_user)
        .execute()
        .await?
        .error_for_status()?
        .json::<Vec<User>>()
        .await?
        .pop()
        .expect("Creation must be Successful at this point");

        Ok(UserRequestResponse::Success(newly_created_user.session_id.expect("Newly created user always has session_id")))
    } else {
        Ok(UserRequestResponse::Fail(UserCreationError::EmailAlreadyInUse))
    }

}

pub async fn log_in(Json(user_payload) : Json<SentUser>) -> Result<UserRequestResponse<UserLogInError>, crate::ReqwestWrapper>{

    let client = 
        Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").
        insert_header("apikey", std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"));

    let mut users_with_this_email : Vec<User> = 
        client
        .from("users")
        .eq("email", user_payload.email)
        .execute()
        .await?
        .error_for_status()?
        .json()
        .await?;

    if let Some(mut user_with_email) = users_with_this_email.pop() {
        if verify(user_payload.password, &user_with_email.password).unwrap() {

            user_with_email.session_id = Some(uuid::Uuid::new_v4());

            let update_param_string = format!("{{ \"session_id\" : \"{}\"}}", user_with_email.session_id.expect("session_id is guaranteed to be set"));

            client
                .from("users")
                .eq("id", user_with_email.id.to_string())
                .update(update_param_string)
                .execute()
                .await?;

            Ok(UserRequestResponse::Success(user_with_email.session_id.expect("Session_id guaranteed to be set")))
                
        } else {
            Ok(UserRequestResponse::Fail(UserLogInError::IncorrectPasswordOrEmail))
        }
    } else {
        Ok(UserRequestResponse::Fail(UserLogInError::NoMatchingRecord))
    }
}

#[axum_macros::debug_handler]
pub async fn log_out(Json(user_session_id) : Json<UuidJson>) -> Result<Response, crate::ReqwestWrapper>{

    let client = Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").
        insert_header("apikey", std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"));

    let update_param_string = format!("{{ \"session_id\" : \"NULL\" }}");

    client
        .from("users")
        .eq("session_id", user_session_id.session_id.to_string())
        .update(update_param_string)
        .execute()
        .await?;

    use axum::http::{header,StatusCode};

    let headers = [
        (header::SET_COOKIE, format!("session_id=''; expires=Thu, 01 Jan 1970 00:00:00 GMT")),
    ];

    Ok((StatusCode::OK,headers,"Logged out and invalidated user session").into_response())
}
