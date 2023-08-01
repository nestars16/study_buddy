use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    middleware,
    routing::{get, post, put},
    BoxError, Router, Server,
};
use std::{sync::Arc, time::Duration};
use tower::{buffer::BufferLayer, timeout::TimeoutLayer, ServiceBuilder};

use study_buddy::users;
use tokio::sync::Mutex;
use tower::limit::rate::RateLimitLayer;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

//TODO JAVASCRIPT REFACTORING - use the dialog for modals

//TODO test create user and fetch content


//TODO better button delay on frontend
//TODO Remember me button
//TODO Forgot your password button

//TODO??? maybe websockets problem

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let app_state = Arc::new(Mutex::new(study_buddy::server::AppState::new().await));

    let auth_needed_routes = Router::new()
        .route("/log_out", post(users::log_out))
        .route("/create_document", post(users::create_document))
        .route("/save", put(users::save_document))
        .route("/fetch_documents", get(users::fetch_posts))
        .route("/fetch_content", get(users::fetch_post_content))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            users::mw_user_ctx_resolver,
        ))
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
                .layer(TimeoutLayer::new(Duration::from_secs(20))),
        )
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
