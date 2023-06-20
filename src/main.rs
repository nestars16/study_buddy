use serde::{Deserialize,Serialize};
use axum::{
    Router,Server,
    extract::{ws::{WebSocket,Message}, WebSocketUpgrade},
    response::{Html, Json, Response, IntoResponse},
    routing::{get,post},
    body::StreamBody,
};
use tokio::fs::read_to_string;
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    //server::start();
    
    dotenv::dotenv().ok(); 

    let router = Router::new()
        .route("/", get(get_root))
        .route("/refresh", get(refresh_file))
        .route("/download",post(download_current_markdown))
        .nest_service("/static", ServeDir::new("static"));

    let server = Server::bind(&"0.0.0.0:0".parse().unwrap()).serve(router.into_make_service());

    println!("Listening on: {:?}", server.local_addr());

    let address_string = format!("http://{}", server.local_addr().to_string());

    open::that(address_string)?;

    server.await.unwrap();

    Ok(())

}

async fn get_root() -> Html<String> {
    Html(read_to_string("static/html/index.html").await.unwrap())
}

async fn refresh_file(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(modify_md_file_state)
}

async fn modify_md_file_state(mut socket: WebSocket) {
    while let Some(new_md_file_state) = socket.recv().await {

        let new_md_file_state = if let Ok(file_state) = new_md_file_state {
            file_state
        } else {
            return;
        };

        if let Message::Text(file_state) = new_md_file_state {
            if socket.send(Message::Text(study_buddy::parse_markdown_file(&file_state).await)).await.is_err() {
                return;
            }
        }
    }
}

#[derive(Deserialize,Debug)]
struct PDFDownloadRequest {
    html : String,
    css : String,
}

async fn download_current_markdown(Json(html_json_payload): Json<PDFDownloadRequest>) -> impl IntoResponse{

    println!("{:?}",html_json_payload);

    #[derive(Serialize,Debug, Default)]
    struct ApiRequest {
        html : String,
        css : String,
    }

    enum StyleType{
        Light,
        Dark,
    };

    impl TryFrom<&str> for StyleType {
        type Error = String;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "dark" =>  Ok(StyleType::Dark),
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
    }else {
        include_str!("../static/templates/pdf.css")
    }.to_string();


    let html = format!("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\"><link href=\"https://pvinis.github.io/iosevka-webfont/3.4.1/iosevka.css\" rel=\"stylesheet\"/><link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/katex@0.16.7/dist/katex.min.css\" integrity=\"sha384-3UiQGuEI4TTMaFmGIZumfRPtfKQ3trwQE2JgosJxCnGmQpL/lJdjpcHkaaFwHlcI\" crossorigin=\"anonymous\"/><title>StudyBuddyDownload</title></head><body><div>{}</div></body></html>",html_json_payload.html);
     
    let api_request = ApiRequest{html, css};
    let api_url = "https://api.pdfendpoint.com/v1/convert";
    let mut headers = reqwest::header::HeaderMap::new();
    let auth_key = std::env::var("PDF_API_KEY").expect("PDF_API_KEY must be set");
    let auth_string = format!("Bearer {}", auth_key);

    let auth = reqwest::header::HeaderValue::from_str(&auth_string).expect("Auth key is valid ASCII");
    let content_type = reqwest::header::HeaderValue::from_static("application/json");
    headers.insert(reqwest::header::CONTENT_TYPE,content_type);
    headers.insert(reqwest::header::AUTHORIZATION,auth);

    //Make POST request

    let client = reqwest::Client::new();

    let api_response = client
        .post(api_url)
        .headers(headers)
        .json(&api_request)
        .send().await;

    match api_response {
        Ok(response) => {
           
            let stream = response.bytes_stream();
            let body = StreamBody::new(stream);

            let headers = [
                (axum::http::header::CONTENT_TYPE, "application/pdf"),
                (axum::http::header::CONTENT_DISPOSITION,
                 "attachment; filename=StudyBuddyDownload.pdf"
                ),
            ];

            Ok((headers,body))
        }
        Err(err) => {
            //TODO better error handling
            return Err((axum::http::StatusCode::NOT_FOUND, format!("Error performing conversion {}", err)));
        }
    }

}



