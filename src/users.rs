use axum::{
    extract::Query,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use postgrest::Postgrest;

use crate::StudyBuddyError;
use axum_extra::extract::cookie::CookieJar;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SentUser {
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UuidJson {
    unique_id: uuid::Uuid,
    text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: uuid::Uuid,
    email: String,
    password: String,
    session_id: Option<uuid::Uuid>,
}

#[derive(Serialize, Clone, Debug)]
pub struct Document {
    user_id: uuid::Uuid,
    title: String,
    content: String,
    document_id: uuid::Uuid,
}

impl Document {
    fn new(user_id: uuid::Uuid, title: String) -> Self {
        Document {
            user_id,
            title,
            content: String::new(),
            document_id: uuid::Uuid::new_v4(),
        }
    }
}

impl User {
    fn new(email: String, password: String) -> Self {
        let password = hash(password, DEFAULT_COST).expect("All passwords should hash");
        User {
            id: uuid::Uuid::new_v4(),
            email,
            password,
            session_id: Some(uuid::Uuid::new_v4()),
        }
    }

    async fn validate_user<T>(
        client: &Postgrest,
        table_name: &str,
        (field, equal): (&str, &str),
    ) -> Result<T, StudyBuddyError>
    where
        T: DeserializeOwned,
    {
        Ok(client
            .from(table_name)
            .eq(field, equal)
            .execute()
            .await?
            .error_for_status()?
            .json()
            .await?)
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

#[axum_macros::debug_handler]
pub async fn create_user(Json(user_payload): Json<SentUser>) -> Result<Response, StudyBuddyError> {
    let client = Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").insert_header(
        "apikey",
        std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"),
    );

    if User::validate_user::<Vec<User>>(&client, "users", ("email", &user_payload.email))
        .await?
        .first()
        .is_some()
    {
        return Err(StudyBuddyError::EmailAlreadyInUse);
    }

    let new_user = User::new(user_payload.email, user_payload.password);
    let session_id = new_user
        .session_id
        .expect("Newly created user has guaranteed session_id")
        .to_string()
        .clone();

    client
        .from("users")
        .insert(new_user)
        .execute()
        .await?
        .error_for_status()?;

    let headers = [(header::SET_COOKIE, format!("session_id={}", session_id))];

    Ok((
        StatusCode::CREATED,
        headers,
        "Created user and instantied user session",
    )
        .into_response())
}

#[axum_macros::debug_handler]
pub async fn log_in(Json(user_payload): Json<SentUser>) -> Result<Response, StudyBuddyError> {
    let client = Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").insert_header(
        "apikey",
        std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"),
    );

    let mut user_with_email =
        User::validate_user::<Vec<User>>(&client, "users", ("email", &user_payload.email))
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| StudyBuddyError::NoMatchingUserRecord)?;

    if !verify(user_payload.password, &user_with_email.password).unwrap() {
        return Err(StudyBuddyError::WrongEmailOrPassword);
    }

    let new_session_id = uuid::Uuid::new_v4();
    user_with_email.session_id = Some(new_session_id);

    let update_param_string = format!("{{ \"session_id\" : \"{}\"}}", new_session_id);

    client
        .from("users")
        .eq("id", user_with_email.id.to_string())
        .update(update_param_string)
        .execute()
        .await?
        .error_for_status()?;

    let headers = [(header::SET_COOKIE, format!("session_id={}", new_session_id))];

    Ok((
        StatusCode::OK,
        headers,
        "Created user and instantied user session",
    )
        .into_response())
}

pub async fn log_out(Json(user_session_id): Json<UuidJson>) -> Result<Response, StudyBuddyError> {
    let client = Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").insert_header(
        "apikey",
        std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"),
    );

    //Unique id in this case is the session id to invalidate in the database
    client
        .from("users")
        .eq("session_id", user_session_id.unique_id.to_string())
        .update(r#"{ "session_id" : null }"#)
        .execute()
        .await?
        .error_for_status()?;

    let headers = [(
        header::SET_COOKIE,
        format!("session_id=''; expires=Thu, 01 Jan 1970 00:00:00 GMT"),
    )];

    Ok((
        StatusCode::OK,
        headers,
        "Logged out and invalidated user session",
    )
        .into_response())
}

pub async fn create_post(
    Json(user_request_info): Json<UuidJson>,
) -> Result<Json<UuidJson>, StudyBuddyError> {
    let client = Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").insert_header(
        "apikey",
        std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"),
    );

    let valid_user = User::validate_user::<Vec<User>>(
        &client,
        "users",
        ("session_id", &user_request_info.unique_id.to_string()),
    )
    .await?
    .into_iter()
    .next()
    .ok_or_else(|| StudyBuddyError::NoMatchingUserRecord)?;

    if let Some(post_title) = user_request_info.text {
        let new_document = Document::new(valid_user.id, post_title);

        client
            .from("documents")
            .insert(new_document.clone())
            .execute()
            .await?
            .error_for_status()?;

        return Ok(Json(UuidJson {
            unique_id: new_document.document_id,
            text: Some(new_document.title),
        }));
    } else {
        Err(StudyBuddyError::IncompleteRequest)
    }
}

#[derive(Deserialize, Serialize)]
pub struct DatabaseDocumentRecords {
    document_id: uuid::Uuid,
    title: String,
}

pub async fn fetch_posts(
    jar: CookieJar,
) -> Result<Json<Vec<DatabaseDocumentRecords>>, StudyBuddyError> {

    let user_session_id = if let Some(id) = jar.get("session_id") {
        id.value()
    } else {
        return Err(StudyBuddyError::InvalidUserSession);
    };

    let client = Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").insert_header(
        "apikey",
        std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"),
    );


    let valid_user = User::validate_user::<Vec<User>>(
        &client,
        "users",
        ("session_id", user_session_id),
    )
    .await?
    .into_iter()
    .next()
    .ok_or_else(|| StudyBuddyError::InvalidUserSession)?;

    let user_posts = client
        .from("documents")
        .eq("user_id", valid_user.id.to_string())
        .select("title, document_id")
        .execute()
        .await?
        .error_for_status()?
        .json::<Vec<DatabaseDocumentRecords>>()
        .await?;

    Ok(Json(user_posts))
}

#[derive(Serialize, Deserialize)]
pub struct SavePostRequest {
    user_session_id: uuid::Uuid,
    document_id: uuid::Uuid,
    text: String,
}

pub async fn save_post(
    Json(user_save_request): Json<SavePostRequest>,
) -> Result<Response, StudyBuddyError> {
    let client = Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").insert_header(
        "apikey",
        std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"),
    );

    if User::validate_user::<Vec<User>>(&client, "users", ("session_id", &user_save_request.user_session_id.to_string()))
    .await?
    .first()
    .is_none(){
        return Err(StudyBuddyError::InvalidUserSession)
    };

    let update_string = format!("{{ \"content\" : \"{}\" }}", user_save_request.text);

    client
        .from("documents")
        .eq("document_id", user_save_request.document_id.to_string())
        .update(update_string)
        .execute()
        .await?
        .error_for_status()?;

    Ok((StatusCode::OK, "Post contents saved succesfully").into_response())
}

#[derive(Deserialize)]
pub struct DocumentId {
    document_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct DocumentContent {
    content: String,
}

pub async fn fetch_post_content(
    jar: CookieJar,
    document_id: Query<DocumentId>,
) -> Result<Json<String>, StudyBuddyError> {
    let user_session_id = if let Some(id) = jar.get("session_id") {
        id.value()
    } else {
        return Err(StudyBuddyError::InvalidUserSession);
    };

    let client = Postgrest::new("https://hgioigecbrqawyedynet.supabase.co/rest/v1").insert_header(
        "apikey",
        std::env::var("SUPA_BASE_KEY").expect("Database auth needs to be set"),
    );

    if User::validate_user::<Vec<User>>(&client, "users", ("session_id", user_session_id))
    .await?
    .first()
    .is_none(){
        return Err(StudyBuddyError::InvalidUserSession)
    };

    let document_lookup = document_id.0;

    let doc_contents = client
        .from("documents")
        .eq("document_id", document_lookup.document_id)
        .select("content")
        .execute()
        .await?
        .error_for_status()?
        .json::<Vec<DocumentContent>>()
        .await?
        .pop();

    if let Some(document) = doc_contents {
        Ok(Json(document.content))
    } else {
        Err(StudyBuddyError::DocumentNotFound)
    }
}
