use study_buddy::get_parsed_markdown_in_folder;
use axum::routing::get;
use axum::{Server,Router};
use axum::response::{Html,Response,IntoResponse,Json};
use tokio::fs::read_to_string;

#[tokio::main]
async fn main() -> std::io::Result<()> {

    //server::start();
    let router = Router::new().route(
        "/",
        get(get_root)
        )
        .route("/index.js", get(get_js))
        .route("/styles.css", get(get_css))
        .route("/api/md", get(get_md_files));

    let server = Server::bind(&"0.0.0.0:0".parse().unwrap())
                .serve(router.into_make_service());

    println!("Listening on: {:?}", server.local_addr());

    let address_string = format!("http://{}",server.local_addr().to_string());

    open::that(address_string)?;

    server.await.unwrap();

    Ok(())
}

pub async fn get_root() -> Html<String>{

    Html(read_to_string("html/index.html").await.unwrap())

}

pub async fn get_static_file(path_to_file : &str, content_type: &str) -> Response<String> {

    let markup = read_to_string(path_to_file).await.unwrap();

    Response::builder()
            .header("content-type", format!("{content_type};charset=utf8"))
            .body(markup)
            .unwrap()

}

pub async fn get_js() -> Response<String>{

    get_static_file("js/index.js","application/javascript").await
}


pub async fn get_css() -> Response<String>{

    get_static_file("css/styles.css", "text/css").await
}

pub async fn get_md_files() -> impl IntoResponse {

    let payload : Vec<_> = get_parsed_markdown_in_folder("./md_files").await.unwrap_or(Vec::new());

    Json(payload)
}
