use tower_http::services::ServeDir;
use axum::{
    Router,Server,
    routing::{get,post}
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //server::start();
    
    dotenv::dotenv().ok(); 

    let router = Router::new()
        .route("/", get(study_buddy::server::get_root))
        .route("/refresh", get(study_buddy::server::refresh_file))
        .route("/download",post(study_buddy::server::download_current_markdown))
        .nest_service("/static", ServeDir::new("static"));

    let server = Server::bind(&"0.0.0.0:0".parse().unwrap()).serve(router.into_make_service());

    println!("Listening on: {:?}", server.local_addr());

    let address_string = format!("http://{}", server.local_addr().to_string());

    open::that(address_string)?;

    server.await.unwrap();

    Ok(())

}
