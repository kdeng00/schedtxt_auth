pub mod config;
pub mod db;
pub mod hashing;
pub mod repo;


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    /*
    let app = init::app().await;

    // run our app with hyper, listening globally on port 8001
    let url = config::get_full();
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    */
}
