use crate::server::AppState;
use crate::{StudyBuddyError, StudyBuddySessionError};
use async_trait::async_trait;
use axum::{

    response::Html,
    extract::State,
    extract::{FromRequestParts, Query},
    http::{
        request::{Parts, Request},
        StatusCode,
    },
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPool, FromRow};
use std::str::FromStr;
use std::sync::Arc;
use tokio::{sync::Mutex,
fs::read_to_string,
    };
use tower_cookies::{
    cookie::time::{Duration, OffsetDateTime},
    Cookie, Cookies};
use check_if_email_exists::{check_email, CheckEmailInput,Reachable};
use tracing::info;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    Message, AsyncSmtpTransport, AsyncTransport, Address, 
};

#[derive(Serialize, Deserialize, Debug)]
pub struct SentUser {
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct LogInRequest {
    email: String,
    password: String,
    wants_to_be_remembered: bool,
}

#[derive(Deserialize, Serialize)]
pub struct SentDocument {
    unique_id: uuid::Uuid,
    text: String,
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
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

    async fn validate_user<'a ,T>(
        pool: &PgPool,
        query_string : &'a str,
        equal: T
    ) -> Result<Option<User>, StudyBuddyError> 
    where T: sqlx::Type<sqlx::Postgres> + sqlx::Encode<'a, sqlx::Postgres> + std::marker::Send + 'a{

        Ok(sqlx::query_as::<_, User>(query_string)
            .bind(equal)
            .fetch_optional(pool)
            .await?)
    }

    fn create_validate_user_string(comparison_field : &str) -> String {
        format!("SELECT *
                 FROM users
                 WHERE {comparison_field} = $1
                ")
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

#[derive(Clone)]
pub struct UserCtx {
    user_id: uuid::Uuid,
}

impl UserCtx {
    pub fn new(user_id: uuid::Uuid) -> Self {
        UserCtx { user_id }
    }
}

pub async fn mw_user_ctx_resolver<B>(
    State(app_state): State<Arc<Mutex<AppState>>>,
    cookies: Cookies,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StudyBuddySessionError> {
    info!("Attempting to extract UserCtx");
    let session_id = cookies.get("session_id").map(|c| c.value().to_string());


    let ctx_result = match session_id.ok_or(StudyBuddySessionError::NoSessionId) {
        Ok(session_id) => {

            {
                let query_string = User::create_validate_user_string("session_id");
                let session_id = uuid::Uuid::from_str(&session_id).map_err(|_|StudyBuddySessionError::InvalidUserSession)?;
                let pool = &app_state.lock().await.pool;
                User::validate_user::<uuid::Uuid>(
                    pool,
                    &query_string,
                    session_id
                )
                .await
                .map_err(|_|StudyBuddySessionError::LookupFailed)?
                .ok_or(StudyBuddySessionError::InvalidUserSession)
            }
        }

        Err(error) => Err(error),
    };

    let ctx_result = ctx_result.map(|user| UserCtx { user_id: user.id });

    if ctx_result.is_err() && !matches!(ctx_result, Err(StudyBuddySessionError::NoSessionId)) {
        cookies.remove(Cookie::named("session_id"));
        info!("Failed to extract UserCtx");
    }

    req.extensions_mut().insert(ctx_result);

    Ok(next.run(req).await)
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for UserCtx {
    type Rejection = StudyBuddySessionError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, StudyBuddySessionError> {
        parts
            .extensions
            .get::<Result<UserCtx, StudyBuddySessionError>>()
            .ok_or(StudyBuddySessionError::InvalidUserSession)?
            .clone()
    }
}

#[axum_macros::debug_handler]
pub async fn create_user(
    cookies: Cookies,
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(user_payload): Json<SentUser>,
) -> Result<Response, StudyBuddyError> {

    info!("Creating user with credentials {:?}", user_payload);
    let email_to_check = CheckEmailInput::new(user_payload.email.clone()); 
    let result = check_email(&email_to_check).await;

    if !matches!(result.is_reachable, Reachable::Safe | Reachable::Risky) {
        info!("Invalid email address : {}", user_payload.email);
        return Err(StudyBuddyError::InvalidEmailAddress);
    }

    let pool = &app_state.lock().await.pool;

    let query_string = User::create_validate_user_string("email");
    if User::validate_user(pool,&query_string,&user_payload.email).await?.is_some() {
        info!("email address already in use: {}", user_payload.email);
        return Err(StudyBuddyError::EmailAlreadyInUse)
    }

    let new_user = User::new(user_payload.email, user_payload.password);
    let session_id = new_user
        .session_id
        .expect("Newly created user has guaranteed session_id");

    let session_id_string = session_id.to_string().clone();

    sqlx::query!(
        "INSERT INTO users (id, email, password, session_id)
            VALUES ($1, $2, $3, $4)
        ",
        new_user.id,
        new_user.email,
        new_user.password,
        session_id
    )
        .execute(pool)
        .await?;

    cookies.add(Cookie::new("session_id",session_id_string));

    Ok((
        StatusCode::CREATED,
        "Created user and instantied user session"
    )
        .into_response())
}

#[axum_macros::debug_handler]
pub async fn log_in(
    cookies: Cookies,
    State(app_state): State<Arc<Mutex<AppState>>>,
    Json(user_payload): Json<LogInRequest>,
) -> Result<Response, StudyBuddyError> {

    info!("Logging in user {:?}", user_payload);

    let pool = &app_state.lock().await.pool;
    let query_string = User::create_validate_user_string("email");
    let mut user_with_email = 
        User::validate_user(pool, &query_string, &user_payload.email)
        .await?
        .ok_or_else(|| StudyBuddyError::NoMatchingUserRecord)?;

    if !verify(user_payload.password, &user_with_email.password).unwrap() {
        info!("Invalid password for {:?}", user_payload.email);
        return Err(StudyBuddyError::WrongEmailOrPassword);
    }

    let new_session_id = uuid::Uuid::new_v4();
    user_with_email.session_id = Some(new_session_id);

    sqlx::query!(
        "UPDATE users
            SET session_id = $1
            WHERE id = $2
        ",
        new_session_id,
        user_with_email.id
    )
    .execute(pool)
    .await?;

    let mut session_cookie = Cookie::new("session_id",new_session_id.to_string());

    if user_payload.wants_to_be_remembered {
        info!("Added 365 day duration on log in cookie");
        session_cookie.set_expires(OffsetDateTime::now_utc() + Duration::days(365));
    }

    cookies.add(session_cookie);

    Ok((
        StatusCode::OK,
        "Created user and instantied user session",
    )
        .into_response())
}

pub async fn log_out(
    cookies : Cookies,
    State(app_state): State<Arc<Mutex<AppState>>>,
    ctx: UserCtx,
) -> Result<Response, StudyBuddyError> {

    //Unique id in this case is the session id to invalidate in the database
    {
        let pool = &app_state.lock().await.pool;

        sqlx::query!(
            "UPDATE users
            SET session_id = NULL
            WHERE id = $1
            ",
            ctx.user_id
        )
        .execute(pool)
        .await?;
    }

    cookies.remove(Cookie::named("session_id"));
    info!("Logged out user with id {}", ctx.user_id);

    Ok((
        StatusCode::OK,
        "Logged out and invalidated user session",
    )
        .into_response())
}

#[derive(Deserialize)]
pub struct CreateDocumentRequest {
    title: String,
}

pub async fn create_document(
    State(app_state): State<Arc<Mutex<AppState>>>,
    ctx: UserCtx,
    Json(user_request_info): Json<CreateDocumentRequest>,
) -> Result<Json<SentDocument>, StudyBuddyError> {
    let new_document = Document::new(ctx.user_id, user_request_info.title);

    {
        let pool = &app_state.lock().await.pool;

        sqlx::query!(
            "INSERT INTO documents (user_id, title, content, document_id)
             VALUES ($1, $2, $3, $4)
            ",
            ctx.user_id,
            new_document.title,
            new_document.content,
            new_document.document_id
        )
        .execute(pool)
        .await?;
    }

    info!("Created document with title {} and id {}", new_document.title, new_document.document_id);

    Ok(Json(SentDocument {
        unique_id: new_document.document_id,
        text: new_document.title,
    }))
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct DatabaseDocumentRecords {
    document_id: uuid::Uuid,
    title: String,
}

pub async fn fetch_posts(
    State(app_state) : State<Arc<Mutex<AppState>>>,
    ctx: UserCtx,
) -> Result<Json<Vec<DatabaseDocumentRecords>>, StudyBuddyError> {

    let pool = &app_state.lock().await.pool;
    info!("Fetching posts for user {}", ctx.user_id);

    let user_posts = sqlx::query_as::<_,DatabaseDocumentRecords>(
        "SELECT title, document_id
        FROM documents
        WHERE user_id = $1"
        ).bind(ctx.user_id)
        .fetch_all(pool)
        .await?;

    Ok(Json(user_posts))
}

#[derive(Serialize, Deserialize)]
pub struct SavePostRequest {
    document_id: uuid::Uuid,
    text: String,
}

pub async fn save_document(
    State(app_state) : State<Arc<Mutex<AppState>>>,
    _ctx: UserCtx,
    Json(user_save_request): Json<SavePostRequest>,
) -> Result<Response, StudyBuddyError> {

    let pool = &app_state.lock().await.pool;
    info!("Saving document with id {}", user_save_request.document_id);

    sqlx::query!(
        "UPDATE documents
         SET content = $1
         WHERE document_id = $2",
         user_save_request.text,
         user_save_request.document_id
        ).execute(pool)
        .await?;

    Ok((StatusCode::OK, "Post contents saved succesfully").into_response())
}

#[derive(Deserialize)]
pub struct DocumentId {
    document_id: String,
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct DocumentContent {
    content: String,
}

pub async fn fetch_post_content(
    State(app_state) : State<Arc<Mutex<AppState>>>,
    _ctx: UserCtx,
    Query(document_id): Query<DocumentId>,
) -> Result<Json<String>, StudyBuddyError> {

    let doc_id = uuid::Uuid::from_str(&document_id.document_id).map_err(|_|StudyBuddyError::DocumentNotFound)?;

    let pool = &app_state.lock().await.pool;

    let doc_contents = sqlx::query_as::<_, DocumentContent>(
        "SELECT content
        FROM documents
        WHERE document_id = $1
        "
        )
        .bind(doc_id)
        .fetch_optional(pool)
        .await?;

    if let Some(document) = doc_contents {
        Ok(Json(document.content))
    } else {
        Err(StudyBuddyError::DocumentNotFound)
    }
}

pub async fn get_recovery_page() -> Html<String>{
    info!("Serving '/recovery'");
    Html(read_to_string("static/html/recovery.html").await.unwrap())
}

#[derive(Deserialize,Clone)]
pub struct Email{
    email : String
}

impl Email {
    fn new(email : &str) -> Self{
        Email{ email : email.to_string() }
    }

    fn to_mailbox(self) -> Result<Mailbox, StudyBuddyError> {

        let mut email_iterator = self.email.split('@');

        let (Some(name), Some(email)) = (email_iterator.next(), email_iterator.next()) else {
            return Err(StudyBuddyError::InvalidEmailAddress)
        };

        let address = Address::new(name, email).map_err(|_| StudyBuddyError::InvalidEmailAddress)?;

        Ok(Mailbox::new(None, address))  
    }
}

pub async fn send_password_recovery_email(State(app_state): State<Arc<Mutex<AppState>>>, Json(user_email): Json<Email>) -> Result<Response, StudyBuddyError>{

    //let pool = &app_state.lock().await.pool;              
    //let uuid = create_temp_password_in_database(pool, &user_email.email).await?;
    let recovery_email_address = std::env::var("EMAIL").expect("EMAIL should be set");
    let uuid = uuid::Uuid::new_v4();

    let message = Message::builder()
        .from(Email::new(&recovery_email_address).to_mailbox()?)
        .to(user_email.to_mailbox()?)
        .subject("StudyBuddy recovery email")
        .header(ContentType::TEXT_HTML)
        .body(include_str!("../static/html/email_content.html").to_string().replace("$recovery_code$", &uuid.to_string()))
        .expect("All email fields should be valid");

    let credentials = Credentials::new(recovery_email_address, std::env::var("EMAIL_APP_PASSWORD").expect("EMAIL_APP_PASSWORD should be set"));    

    let mailer = AsyncSmtpTransport::<lettre::Tokio1Executor>::relay("smtp.gmail.com")
    .unwrap()
    .credentials(credentials)
    .build();

    match mailer.send(message).await {
        Ok(_) => {
            Ok((StatusCode::OK, "Email sent succesfully".to_string()).into_response())
        },
        Err(err) => Ok((StatusCode::BAD_REQUEST, format!("Email failed to send {:?}", err)).into_response())
    }
}

pub async fn create_temp_password_in_database(pool : &PgPool, sender:  &String) -> Result<uuid::Uuid, StudyBuddyError>{
    
    let query_string = User::create_validate_user_string("email");

    let Some(user_email) = User::validate_user(pool,&query_string,&sender).await? else {
        return Err(StudyBuddyError::NoMatchingUserRecord);
    };

    let user_email = user_email.email;

    let temp_password = uuid::Uuid::new_v4();

    sqlx::query!(
        "INSERT INTO temporary
        VALUES ($1, $2)",
        temp_password,
        user_email
        ).execute(pool).await?;

    Ok(temp_password)
}

#[derive(Deserialize)] 
pub struct PasswordRecoveryRequest{
    code : uuid::Uuid,
    password: String
}

#[derive(FromRow)]
pub struct TemporaryCodeRow {
    corresponding_email : String,
}

pub async fn try_recovery_code(State(app_state) : State<Arc<Mutex<AppState>>>, Json(req) : Json<PasswordRecoveryRequest>) -> Result<Response,StudyBuddyError> {
    

    let pool = &app_state.lock().await.pool;
    let email = sqlx::query_as::<_, TemporaryCodeRow>(
        "SELECT corresponding_email
        FROM temporary
        WHERE temp_password = $1"
        ).bind(&req.code)
        .fetch_optional(pool)
        .await?;

    let Some(valid_email) = email else {
        return Err(StudyBuddyError::InvalidRecoveryCode);
    };

    let valid_email = valid_email.corresponding_email;

    let hashed = hash(req.password, DEFAULT_COST).expect("All passwords should hash");

    sqlx::query!(
        "
        UPDATE users
        SET password = $1
        WHERE email = $2
        ",
        hashed,
        valid_email
        ).execute(pool)
        .await?;

    sqlx::query!(
        "
        DELETE FROM temporary
        WHERE temp_password = $1 
        ",
        req.code
        ).execute(pool)
        .await?;

    Ok((StatusCode::OK, "Successfully reset password").into_response())
}
