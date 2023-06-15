use axum::extract::{ws::{WebSocket,Message}, WebSocketUpgrade};
use axum::response::{Html, IntoResponse, Json, Response};
use axum::routing::get;
use axum::{Router, Server};
use tokio::fs::read_to_string;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //server::start();
    
    let router = Router::new()
        .route("/", get(get_root))
        .route("/index.js", get(get_js))
        .route("/styles.css", get(get_css))
        .route("/file", get(get_md_files))
        .route("/refresh", get(refresh_file));

    let server = Server::bind(&"0.0.0.0:0".parse().unwrap()).serve(router.into_make_service());

    println!("Listening on: {:?}", server.local_addr());

    let address_string = format!("http://{}", server.local_addr().to_string());

    open::that(address_string)?;

    server.await.unwrap();

    Ok(())


}

async fn get_root() -> Html<String> {
    Html(read_to_string("html/index.html").await.unwrap())
}

async fn get_static_file(path_to_file: &str, content_type: &str) -> Response<String> {
    let markup = read_to_string(path_to_file).await.unwrap();

    Response::builder()
        .header("content-type", format!("{content_type};charset=utf8"))
        .body(markup)
        .unwrap()
}

async fn get_js() -> Response<String> {
    get_static_file("js/index.js", "application/javascript").await
}

async fn get_css() -> Response<String> {
    get_static_file("css/styles.css", "text/css").await
}

async fn get_md_files() -> impl IntoResponse {
   Json(tokio::fs::read_to_string("md_files/titles.md").await.unwrap())
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
