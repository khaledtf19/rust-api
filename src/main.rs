use axum::{
    routing::get,
    Router,
};
#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let app = Router::new().route("/", get(|| async  {"Hello"}) );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
