use serde_json::Value;
use serde::Deserialize;
use axum::extract::{ws::{WebSocket,Message}, WebSocketUpgrade};
use axum::response::{Html, Json, Response, IntoResponse};
use axum::routing::{get,post};
use axum::{Router, Server};
use tokio::fs::read_to_string;
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    //server::start();
    
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

    let client= reqwest::Client::new();

    let response  = client
        .post("https://api.pdfendpoint.com/v1/convert");

}



