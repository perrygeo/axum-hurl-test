use axum::response::Html;
use axum::routing::get;
use axum::Router;

pub fn make_app() -> Router {
    Router::new()
        .route("/", get(handler))
        .route("/2", get(handler2))
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn handler2() -> &'static str {
    "<h1>Yo, World!</h1>"
}
