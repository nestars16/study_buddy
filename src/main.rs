use axum::{
    error_handling::HandleErrorLayer,
    http::StatusCode,
    middleware,
    routing::{delete, get, post, put},
    BoxError, Router, Server,
};
use std::{sync::Arc, time::Duration};
use study_buddy::users;
use tokio::sync::Mutex;
use tower::{
    buffer::BufferLayer, limit::rate::RateLimitLayer, timeout::TimeoutLayer, ServiceBuilder,
};
use tower_cookies::CookieManagerLayer;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};
use tracing::{info, log::warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

//TODO add text search to document_titles
//TODO syntax highlighting for code blocks

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "study_buddy=debug,tower_http=debug,axum::rejection=trace,sqlx=info".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app_state = Arc::new(Mutex::new(study_buddy::server::AppState::new().await));

    let auth_needed_routes = Router::new()
        .route("/log_out", post(users::log_out))
        .route("/create_document", post(users::create_document))
        .route("/save", put(users::save_document))
        .route("/fetch_documents", get(users::fetch_posts))
        .route("/fetch_content", get(users::fetch_post_content))
        .route("/delete_document", delete(users::delete_document))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            users::mw_user_ctx_resolver,
        ));

    let router = Router::new()
        .route_service("/", ServeFile::new("static/html/index.html"))
        .route("/refresh", get(study_buddy::server::refresh_file))
        .route(
            "/download",
            post(study_buddy::server::download_current_markdown),
        )
        .route("/create_user", post(users::create_user))
        .route("/log_in", post(users::log_in))
        .route_service("/recovery", ServeFile::new("static/html/recovery.html"))
        .route("/send_recovery", post(users::send_password_recovery_email))
        .route("/try_recovery_code", post(users::try_recovery_code))
        .merge(auth_needed_routes)
        .nest_service("/static", ServeDir::new("static"))
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(10, Duration::from_secs(1)))
                .layer(TimeoutLayer::new(Duration::from_secs(60)))
                .layer(CookieManagerLayer::new()),
        )
        .with_state(app_state)
        .fallback_service(ServeFile::new("static/html/not_found.html"));

    let quit_sig = async {
        _ = tokio::signal::ctrl_c().await;
        warn!("Initiating graceful shutdown");
    };

    let address = &"0.0.0.0:8080"
        .parse()
        .expect("Address is guaranteed to be valid");

    info!("Listening on: {:?}", address);

    let server = Server::bind(address)
        .serve(router.into_make_service())
        .with_graceful_shutdown(quit_sig);

    server.await.unwrap();

    Ok(())
}
