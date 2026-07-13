#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = schedtxt_auth::init::app().await;

    // run our app with hyper, listening globally on port 9080
    let url = schedtxt_auth::config::get_full();
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
