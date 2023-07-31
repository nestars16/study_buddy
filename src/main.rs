use tower::{ServiceBuilder,
    timeout::TimeoutLayer,
    buffer::BufferLayer};
use std::time::Duration;
use axum::{
    error_handling::HandleErrorLayer,
    BoxError,
    http::StatusCode,
    middleware,
    routing::{get, post, put},
    Router, Server,
};

use tower::limit::rate::RateLimitLayer;
use study_buddy::users;
use tower_http::services::ServeDir;
use tower_cookies::CookieManagerLayer;

//TODO JAVASCRIPT REFACTORING

//TODO better button delay on frontend
//TODO find a way to not always create a db client every endpoint
//TODO Remember me button
//TODO Forgot your password button


#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    study_buddy::server::AppState::new().await;

    let auth_needed_routes = Router::new()
        .route("/log_out", post(users::log_out)) 
        .route("/create_document", post(users::create_document)) 
        .route("/save", put(users::save_document)) 
        .route("/fetch_documents", get(users::fetch_posts))
        .route("/fetch_content", get(users::fetch_post_content))
        .layer(middleware::from_fn(users::mw_user_ctx_resolver))
        .layer(CookieManagerLayer::new());


    let router = Router::new()
        .route("/", get(study_buddy::server::get_root))
        .route("/refresh", get(study_buddy::server::refresh_file))
        .route(
            "/download",
            post(study_buddy::server::download_current_markdown),
        )
        .route("/create_user", post(users::create_user))
        .route("/log_in", post(users::log_in))
        .merge(auth_needed_routes)
        .nest_service("/static", ServeDir::new("static"))
        .layer(
            ServiceBuilder::new()
             .layer(HandleErrorLayer::new(|err: BoxError| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled error: {}", err),
                )
            }))
            .layer(BufferLayer::new(1024))
            .layer(RateLimitLayer::new(5, Duration::from_secs(1)))
            .layer(TimeoutLayer::new(Duration::from_secs(20)))
        );

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
