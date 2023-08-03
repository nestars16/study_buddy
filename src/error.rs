use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Clone, Debug)]
pub enum StudyBuddySessionError {
    InvalidUserSession,
    NoSessionId,
    LookupFailed,
}

impl IntoResponse for StudyBuddySessionError {
    fn into_response(self) -> Response {
        match self {
            StudyBuddySessionError::InvalidUserSession => {
                (StatusCode::UNAUTHORIZED, "Invalid user session").into_response()
            }
            StudyBuddySessionError::NoSessionId => {
                (StatusCode::UNAUTHORIZED, "No session cookie found").into_response()
            }
            StudyBuddySessionError::LookupFailed => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Lookup Failed").into_response()
            }
        }
    }
}

impl From<StudyBuddySessionError> for StudyBuddyError {
    fn from(value: StudyBuddySessionError) -> Self {
        StudyBuddyError::SessionError(value)
    }
}

#[derive(Debug)]
pub enum StudyBuddyError {
    SessionError(StudyBuddySessionError),
    NoMatchingUserRecord,
    EmailAlreadyInUse,
    IncompleteRequest,
    WrongEmailOrPassword,
    DocumentNotFound,
    ReqwestWrapper(reqwest::Error),
    InvalidEmailAddress,
    SqlxWrapper(sqlx::Error),
}

impl From<reqwest::Error> for StudyBuddyError {
    fn from(value: reqwest::Error) -> Self {
        StudyBuddyError::ReqwestWrapper(value)
    }
}

impl From<sqlx::Error> for StudyBuddyError {
    fn from(value: sqlx::Error) -> Self {
        StudyBuddyError::SqlxWrapper(value)
    }
}

impl IntoResponse for StudyBuddyError {
    fn into_response(self) -> Response {
        match self {
            StudyBuddyError::WrongEmailOrPassword => {
                (StatusCode::UNAUTHORIZED, "Invalid email or password").into_response()
            }
            StudyBuddyError::NoMatchingUserRecord => (
                StatusCode::NOT_FOUND,
                "No matching user with provided email",
            )
                .into_response(),
            StudyBuddyError::EmailAlreadyInUse => {
                (StatusCode::CONFLICT, "Incorrect Password or Email").into_response()
            }
            StudyBuddyError::IncompleteRequest => (
                StatusCode::BAD_REQUEST,
                "Request doesn't contain all of the necessary fields",
            )
                .into_response(),
            StudyBuddyError::ReqwestWrapper(error) => {
                let status_code = match error.status() {
                    Some(code) => code,
                    None => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                };

                (status_code, error.to_string()).into_response()
            }
            StudyBuddyError::DocumentNotFound => {
                (StatusCode::NO_CONTENT, "Document ID isn't valid").into_response()
            }
            StudyBuddyError::SessionError(err) => err.into_response(),
            //Could be better
            StudyBuddyError::SqlxWrapper(error) => {
                (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()).into_response()
            },
            StudyBuddyError::InvalidEmailAddress => {
                (StatusCode::BAD_REQUEST, "Invalid email address").into_response()
            }
        }
    }
}
