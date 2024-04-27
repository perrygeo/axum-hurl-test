use tokio::net::TcpListener;

use axum_hurl_testing::make_app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = make_app();
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    eprintln!("listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;
    unreachable!();
}
