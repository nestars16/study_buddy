use std::sync::{Arc,Mutex};
use tower_http::services::ServeDir;
use axum::{
    Router,Server,
    routing::{get,post}
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok(); 

    let (tx, _ ) = tokio::sync::broadcast::channel::<String>(1); 
    let raw = Arc::new(Mutex::new(String::new()));
    let rendered = Arc::new(Mutex::new(String::new()));

    let app_state = study_buddy::server::MarkdownState{tx, raw, rendered};

    let router = Router::new()
        .route("/", get(study_buddy::server::get_root))
        .route("/refresh", get(study_buddy::server::refresh_file))
        .route("/download",post(study_buddy::server::download_current_markdown))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(app_state);

    let server = Server::bind(&"0.0.0.0:0".parse().
                              expect("Address is guaranteed to be valid")
                              ).serve(router.into_make_service());

    println!("Listening on: {:?}", server.local_addr());

    let address_string = format!("http://{}", server.local_addr().to_string());

    open::that(address_string)?;

    server.await.unwrap();

    Ok(())
}
