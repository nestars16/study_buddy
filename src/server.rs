use axum::{
    extract::{
        ws::{Message, WebSocket},
        WebSocketUpgrade,
    },
    response::{Json, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub async fn new() -> Self {
        AppState {
            pool: PgPoolOptions::new()
                .max_connections(8)
                .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
                .await
                .expect("Failure of creation of AppState is reason enough to crash"),
        }
    }
}

pub async fn refresh_file(ws: WebSocketUpgrade) -> Response {
    info!("Connecting to refresh socket");
    ws.on_upgrade(modify_md_file_state)
}

pub async fn modify_md_file_state(mut socket: WebSocket) {
    while let Some(new_md_file_state) = socket.recv().await {
        let new_md_file_state = if let Ok(file_state) = new_md_file_state {
            file_state
        } else {
            return;
        };

        if let Message::Text(file_state) = new_md_file_state {
            let parse_result =
                tokio::task::spawn_blocking(move || crate::parse_markdown(&file_state))
                    .await
                    .expect("Task cant panic");

            if socket.send(Message::Text(parse_result)).await.is_err() {
                return;
            }
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PDFDownloadRequest {
    html: String,
    css: String,
}

#[derive(Serialize, Deserialize)]
struct DataFields {
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse {
    success: bool,
    data: DataFields,
}

#[axum_macros::debug_handler]
pub async fn download_current_markdown(
    Json(html_json_payload): Json<PDFDownloadRequest>,
) -> Result<Json<ApiResponse>, crate::StudyBuddyError> {
    info!("Fullfilling download pdf request");

    #[derive(Serialize, Debug, Default)]
    struct ApiRequest {
        html: String,
        css: String,
        js: String,
    }

    enum StyleType {
        Light,
        Dark,
    }

    impl TryFrom<&str> for StyleType {
        type Error = String;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "dark" => Ok(StyleType::Dark),
                "light" => Ok(StyleType::Light),
                _ => Err(format!("Style {} not supported", value)),
            }
        }
    }

    let css = StyleType::try_from(html_json_payload.css.as_str());

    let css = if let Ok(style) = css {
        match style {
            StyleType::Dark => include_str!("../templates/pdf.css"),
            StyleType::Light => include_str!("../templates/lightpdf.css"),
        }
    } else {
        include_str!("../templates/pdf.css")
    }
    .to_string();

    let html = format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><link href=\"https://pvinis.github.io/iosevka-webfont/3.4.1/iosevka.css\" rel=\"stylesheet\"/><link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/katex@0.16.7/dist/katex.min.css\" integrity=\"sha384-3UiQGuEI4TTMaFmGIZumfRPtfKQ3trwQE2JgosJxCnGmQpL/lJdjpcHkaaFwHlcI\" crossorigin=\"anonymous\"/><title>StudyBuddyDownload</title></head><body><div>{}</div></body></html>",html_json_payload.html);

    let js = include_str!("../templates/pdf.js").to_string();

    let api_request = ApiRequest { html, css, js };
    let api_url = "https://api.pdfendpoint.com/v1/convert";
    let mut headers = reqwest::header::HeaderMap::new();
    let auth_key = std::env::var("PDF_API_KEY").expect("PDF_API_KEY must be set");
    let auth_string = format!("Bearer {}", auth_key);

    let auth =
        reqwest::header::HeaderValue::from_str(&auth_string).expect("Auth key is valid ASCII");
    let content_type = reqwest::header::HeaderValue::from_static("application/json");
    headers.insert(reqwest::header::CONTENT_TYPE, content_type);
    headers.insert(reqwest::header::AUTHORIZATION, auth);

    //Make POST request
    let client = reqwest::Client::new();
    let api_response = client
        .post(api_url)
        .headers(headers)
        .json(&api_request)
        .send()
        .await?
        .error_for_status()?
        .json::<ApiResponse>()
        .await?;

    Ok(Json(api_response))
}
