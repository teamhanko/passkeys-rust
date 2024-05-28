use axum::Router;
use http::{header::{AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method};

// use http::{header::{HeaderValue, ACCEPT, AUTHORIZATION, CONTENT_TYPE}, Method};

use crate::controller::{get_info_handler, login_handler, hello_world_handler, logout_handler, start_registration_handler, finalize_registration_handler};
use axum::routing::{get, post};

use tower_http::cors::CorsLayer;

mod controller;
mod model;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_credentials(true);

    let app = Router::new()
        .route("/", get(hello_world_handler))
        .route("/info", get(get_info_handler))
        .route("/login", post(login_handler))
        .route("/logout", post(logout_handler))
        .route("/passkeys/start-registration", post(start_registration_handler))
        .route("/passkeys/finalize-registration", post(finalize_registration_handler))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("Server running on port 8080");

    axum::serve(listener, app).await.unwrap();
}
