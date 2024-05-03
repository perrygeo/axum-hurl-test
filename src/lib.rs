use axum::response::Html;
use axum::routing::get;
use axum::Router;

pub fn make_app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/2", get(handler2))
}

/// Response will be Content-Type: text/html
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

/// Response will be Content-Type: text/plain
async fn handler2() -> &'static str {
    "Yo, World!"
}
