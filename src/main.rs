use reqwest::header::{HeaderMap,self};
use serde::{Deserialize,Serialize};
use axum::{
    Router,Server,
    extract::{ws::{WebSocket,Message}, WebSocketUpgrade},
    response::{Html, Json, Response, IntoResponse},
    routing::{get,post},
    http::{header,StatusCode}
};
use tokio::fs::read_to_string;
use tower_http::services::ServeDir;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    //server::start();
    
    dotenv::dotenv().ok(); 

    let router = Router::new()
        .route("/", get(get_root))
        .route("/refresh", get(refresh_file))
        .route("/download",post(download_current_markdown))
        .nest_service("/static", ServeDir::new("static"));

    let server = Server::bind(&"0.0.0.0:0".parse().unwrap()).serve(router.into_make_service());

    println!("Listening on: {:?}", server.local_addr());

    let address_string = format!("http://{}", server.local_addr().to_string());

    open::that(address_string)?;

    server.await.unwrap();

    Ok(())

}

async fn get_root() -> Html<String> {
    Html(read_to_string("static/html/index.html").await.unwrap())
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

#[derive(Deserialize,Debug)]
struct PDFDownloadRequest {
    html : String,
    css : String,
}

async fn download_current_markdown(Json(html_json_payload): Json<PDFDownloadRequest>) -> impl IntoResponse{

    println!("{:?}",html_json_payload);

    #[derive(Serialize,Debug, Default)]
    struct ApiRequest {
        html : String,
        css : String,
    }

    enum StyleType{
        Light,
        Dark,
    };

    impl TryFrom<&str> for StyleType {
        type Error = String;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            match value {
                "dark" =>  Ok(StyleType::Dark),
                "light" => Ok(StyleType::Light),
                _ => Err(format!("Style {} not supported", value)),
            }
        }
    }

    let css = StyleType::try_from(html_json_payload.css.as_str());

    let css = if let Ok(style) = css {
        match style {
            StyleType::Dark => include_str!("../static/templates/pdf.css"),
            StyleType::Light => include_str!("../static/templates/lightpdf.css"),
        }
    }else {
        include_str!("../static/templates/pdf.css")
    }.to_string();


    let mut html = include_str!("../static/templates/pdf.html").to_string();
    const CONTENT_START_INDEX : usize = 377;
    html.insert_str(CONTENT_START_INDEX, &html_json_payload.html);

    let api_request = ApiRequest{html, css};
    let api_url = "https://api.pdfendpoint.com/v1/convert";
    let mut headers = HeaderMap::new();
    let auth_key = std::env::var("PDF_API_KEY").expect("PDF_API_KEY must be set");
    let auth_string = format!("Bearer {}", auth_key);

    let auth = header::HeaderValue::from_str(&auth_string).expect("Auth key is valid ASCII");
    let content_type = header::HeaderValue::from_static("application/json");
    headers.insert(header::CONTENT_TYPE,content_type);
    headers.insert(header::AUTHORIZATION,auth);

    //Make POST request

    let client = reqwest::Client::new();

    let api_response = client
        .post(api_url)
        .headers(headers)
        .json(&api_request)
        .send().await;

    match api_response {
        Ok(response) => {

        }
        Err(err) => {
           // return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err)));
        }
    }

}



