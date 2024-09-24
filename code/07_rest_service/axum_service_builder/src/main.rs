use axum::{
    http::Method,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tower::{limit::ConcurrencyLimitLayer, ServiceBuilder};
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
};

#[tokio::main]
async fn main() {
    let service = ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any),
        )
        .layer(ConcurrencyLimitLayer::new(100));

    let app = Router::new()
        .route("/", get(handler))
        .layer(service.into_inner());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    const WAR_AND_PEACE: &str = include_str!("../../../03_async/buffered_reader/warandpeace.txt");
    Html(WAR_AND_PEACE)
}
