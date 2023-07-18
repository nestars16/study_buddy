use axum::{
    routing::{get, post},
    Router, Server,
};
use tower_http::services::ServeDir;
use study_buddy::users;

//TODO Users and eventual file navigation
//TODO vim editor settings for textarea possibly

//TODO make light mode and night mode modals
//TODO possible async problem with the parsing of markdown??
//TODO better button delay on frontend
//TODO add timeout to connections
#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let router = Router::new()
        .route("/", get(study_buddy::server::get_root))
        .route("/refresh", get(study_buddy::server::refresh_file))
        .route(
            "/download",
            post(study_buddy::server::download_current_markdown),
        )
        .route("/create_user", post(users::create_user))
        .route("/log_in", post(users::log_in))
        .route("/log_out", post(users::log_out))
        .route("/create_post", post(users::create_post))
        .route("/save", post(users::save_post))
        .nest_service("/static", ServeDir::new("static"));

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
