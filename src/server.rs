use axum::{
    body::StreamBody,
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::{Html, IntoResponse, Json, Response},
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::{fs::read_to_string, sync};

pub struct AppState {
    pub raw_tx: sync::broadcast::Sender<String>,
    pub markdown_tx: sync::broadcast::Sender<String>,
    pub markdown: Mutex<String>,
}

impl AppState {
    pub fn new(
        raw_tx: sync::broadcast::Sender<String>,
        markdown_tx: sync::broadcast::Sender<String>,
    ) -> Self {
        let markdown = Mutex::new(String::new());

        AppState {
            raw_tx,
            markdown,
            markdown_tx,
        }
    }
}

pub async fn get_root() -> Html<String> {
    Html(read_to_string("static/html/index.html").await.unwrap())
}

pub async fn refresh_file(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> Response {
    ws.on_upgrade(|socket| modify_md_file_state(socket, state))
}

pub async fn modify_md_file_state(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut reciever) = socket.split();

    let mut rx = state.markdown_tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(rendered) = rx.recv().await {
            if sender.send(Message::Text(rendered)).await.is_err() {
                break;
            }
        }
    });

    let mut recieve_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(raw_markdown))) = reciever.next().await {
            let _ = state.raw_tx.send(raw_markdown.clone());
            {
                *state.markdown.lock().unwrap() = raw_markdown;
            }
        }
    });

    tokio::select! {
           _ = (&mut send_task) => recieve_task.abort(),
           _ = (&mut recieve_task) => send_task.abort(),
    };
}

#[derive(Deserialize, Debug)]
pub struct PDFDownloadRequest {
    html: String,
    css: String,
}

pub async fn download_current_markdown(
    Json(html_json_payload): Json<PDFDownloadRequest>,
) -> impl IntoResponse {
    println!("{:?}", html_json_payload);

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
            StyleType::Dark => include_str!("../static/templates/pdf.css"),
            StyleType::Light => include_str!("../static/templates/lightpdf.css"),
        }
    } else {
        include_str!("../static/templates/pdf.css")
    }
    .to_string();

    let html = format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><link href=\"https://pvinis.github.io/iosevka-webfont/3.4.1/iosevka.css\" rel=\"stylesheet\"/><link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/katex@0.16.7/dist/katex.min.css\" integrity=\"sha384-3UiQGuEI4TTMaFmGIZumfRPtfKQ3trwQE2JgosJxCnGmQpL/lJdjpcHkaaFwHlcI\" crossorigin=\"anonymous\"/><title>StudyBuddyDownload</title></head><body><div>{}</div></body></html>",html_json_payload.html);

    let js = include_str!("../static/templates/pdf.js").to_string();

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
        .await;

    match api_response {
        Ok(response) => {
            let stream = response.bytes_stream();
            let body = StreamBody::new(stream);

            let headers = [
                (axum::http::header::CONTENT_TYPE, "application/pdf"),
                (
                    axum::http::header::CONTENT_DISPOSITION,
                    "attachment; filename=StudyBuddyDownload.pdf",
                ),
            ];

            Ok((headers, body))
        }
        Err(err) => {
            //TODO better error handling
            return Err((
                axum::http::StatusCode::NOT_FOUND,
                format!("Error performing conversion {}", err),
            ));
        }
    }
}
