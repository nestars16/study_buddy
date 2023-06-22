use axum::{
    routing::{get, post},
    Router, Server,
};
use std::sync::Arc;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let (raw_tx, mut raw_rx): (
        tokio::sync::broadcast::Sender<String>,
        tokio::sync::broadcast::Receiver<String>,
    ) = tokio::sync::broadcast::channel(100);

    let (html_tx, _) = tokio::sync::broadcast::channel(100);

    let app_state = Arc::new(study_buddy::server::AppState::new(raw_tx, html_tx.clone()));

    tokio::task::spawn(async move {
        loop {
            if let Ok(raw_markdown) = raw_rx.recv().await {
                let _ = html_tx.send(study_buddy::parse_markdown_file(&raw_markdown));
            }
        }
    });

    let router = Router::new()
        .route("/", get(study_buddy::server::get_root))
        .route("/refresh", get(study_buddy::server::refresh_file))
        .route(
            "/download",
            post(study_buddy::server::download_current_markdown),
        )
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    let server = Server::bind(
        &"0.0.0.0:0"
            .parse()
            .expect("Address is guaranteed to be valid"),
    )
    .serve(router.into_make_service());

    println!("Listening on: {:?}", server.local_addr());

    let address_string = format!("http://{}", server.local_addr().to_string());

    open::that(address_string)?;

    server.await.unwrap();

    Ok(())
}
