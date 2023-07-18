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

#[derive(Deserialize, Serialize)]
pub struct UuidJson {
    unique_id: uuid::Uuid,
    post_title : Option<String>
}

#[derive(Serialize, Deserialize,Debug)]
pub struct User {
    id : uuid::Uuid,
    email: String,
    password : String,
    session_id : Option<uuid::Uuid>,
}

#[derive(Serialize, Clone)]
pub struct Document {
    user_id : uuid::Uuid,
    title : String,
    content: String,
    document_id : uuid::Uuid,
}


impl Document {
    fn new(user_id : uuid::Uuid, title : String)  -> Self {
        Document{ user_id, title, content: String::new(), document_id : uuid::Uuid::new_v4()}
    }
}

impl User {
    fn new(email : String, password:  String) -> Self {
        let password = hash(password, DEFAULT_COST).expect("All passwords should hash");
        User{id : uuid::Uuid::new_v4(),email,password, session_id : Some(uuid::Uuid::new_v4()) }
    }
}

impl From<Document> for String {
    fn from(value: Document) -> Self {
        serde_json::to_string(&value).expect("Type is serializable")
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
                match error {
                    UserCreationError::EmailAlreadyInUse => (StatusCode::CONFLICT, "This email is already in use").into_response(),
                    //_ =>  (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong with your request").into_response(),
                }

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
                .await?
                .error_for_status()?;

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

    //Unique id in this case is the session id to invalidate in the database
    client
        .from("users")
        .eq("session_id", user_session_id.unique_id.to_string())
        .update(r#"{ "session_id" : null }"#)
        .execute()
        .await?
        .error_for_status()?
        .text()
        .await?;

    use axum::http::{header,StatusCode};

    let headers = [
        (header::SET_COOKIE, format!("session_id=''; expires=Thu, 01 Jan 1970 00:00:00 GMT")),
    ];

    Ok((StatusCode::OK,headers,"Logged out and invalidated user session").into_response())
}

#[axum_macros::debug_handler]
pub async fn create_post(Json(user_request_info) : Json<UuidJson>)  -> Result<Json<Result<UuidJson, &'static str>>, crate::ReqwestWrapper> {

    let client = Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").
        insert_header("apikey", std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"));

    let user_with_session = client
        .from("users")
        .eq("session_id", user_request_info.unique_id.to_string())
        .execute()
        .await?
        .error_for_status()?
        .json::<Vec<User>>()
        .await?
        .pop();

    if let Some(valid_user)  = user_with_session {

        if let Some(post_title) = user_request_info.post_title {

        let new_document = Document::new(valid_user.id, post_title);

        client
            .from("documents")
            .insert(new_document.clone())
            .execute()
            .await?
            .error_for_status()?;

            return Ok(Json(Ok(UuidJson{unique_id: new_document.document_id , post_title: Some(new_document.title)})));
        } 

        Ok(Json(Err("Request doesn't contain title")))

    } else {

        Ok(Json(Err("Invalid user session")))
    }

}

pub async fn save_post(Json(user_session_id) : Json<UuidJson>)  {

}
