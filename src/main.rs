use axum::{
    extract::{Path, State},
    routing::{get, post},
    Router,
};
use std::{env, net::SocketAddr, sync::Arc};

struct AppState {
    uptime_kuma_base_url: url::Url,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        uptime_kuma_base_url: url::Url::parse(&env::var("UPTIME_KUMA_BASE_URL").unwrap()).unwrap(),
    });
    // build our application with a route
    let app = Router::new()
        .route("/health", get(health))
        .route("/*path", post(default_route))
        .with_state(shared_state);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health() -> String {
    "{\"status\":\"OK\"}".into()
}

async fn default_route(State(state): State<Arc<AppState>>, Path(path): Path<String>) -> String {
    let url = state.uptime_kuma_base_url.join(&path.to_string()).unwrap();
    reqwest::get(url).await.unwrap().text().await.unwrap()
}
