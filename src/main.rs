use axum::{
    routing::{get, post, put},
    Router, Server,
};
use study_buddy::users;
use tower_http::services::ServeDir;

//TODO REFACTORING

//TODO better button delay on frontend
//TODO Rate limiting
//TODO google auth
//TODO find a way to not always create a db client every endpoint
//TODO Remember me button

//TODO possible async problem with the parsing of markdown??
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
        .route("/create_document", post(users::create_post))
        .route("/save", put(users::save_post))
        .route("/fetch_documents", get(users::fetch_posts))
        .route("/fetch_content", get(users::fetch_post_content)) //make this get
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
