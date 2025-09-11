use thing_ranker::app_router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app_router = app_router();
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    log::info!("Starting application");
    axum::serve(listener, app_router).await.unwrap();
}
