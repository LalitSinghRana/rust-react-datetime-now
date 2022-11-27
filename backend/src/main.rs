use axum::{
    http::{HeaderValue, Method},
    response::Html, 
    routing::get, 
    Router, Json};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use chrono::offset::Local;

#[tokio::main]
async fn main() {
    println!("@main : started");

    let app = Router::new()
        .route("/", get(handler))
        .route("/json", get(get_int))
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        );

    let addr = "[::]:8080".parse::<SocketAddr>().unwrap();
    println!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<& 'static str> {
    Html("<h1>Hello, world!!!</h1>")
}

async fn get_int() -> Json<String> {
    let date_time = Local::now().to_string();
    Json(date_time)
}
