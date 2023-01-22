//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-hello-world
//! ```

use axum::{routing::post, Router};
use std::net::SocketAddr;
use std::env;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new().route("/", post(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> String {
    reqwest::get(env::var("UPTIME_KUMA_URL").unwrap())
        .await.unwrap()
        .text()
        .await.unwrap()
}
