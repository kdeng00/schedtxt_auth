pub mod callers;
pub mod config;
pub mod db;
pub mod hashing;
pub mod repo;
pub mod token_stuff;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let app = init::app().await;

    // run our app with hyper, listening globally on port 9080
    let url = config::get_full();
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

mod init {
    use axum::{
        Router,
        routing::{get, post},
    };
    use utoipa::OpenApi;

    use super::callers;
    use callers::common as common_callers;
    use callers::register as register_caller;
    use register_caller::response as register_responses;

    #[derive(utoipa::OpenApi)]
    #[openapi(
        paths(
            common_callers::endpoint::db_ping, common_callers::endpoint::root,
            register_caller::register_user,
            ),
        components(schemas(common_callers::response::TestResult,
                register_responses::Response)),
        tags(
            (name = "TextSender Auth API", description = "Auth API for TextSender API")
            )
    )]
    struct ApiDoc;

    mod cors {
        pub async fn configure_cors() -> tower_http::cors::CorsLayer {
            // Start building the CORS layer with common settings
            let cors = tower_http::cors::CorsLayer::new()
                .allow_methods([
                    axum::http::Method::GET,
                    axum::http::Method::POST,
                    axum::http::Method::PUT,
                    axum::http::Method::DELETE,
                ]) // Specify allowed methods:cite[2]
                .allow_headers([
                    axum::http::header::CONTENT_TYPE,
                    axum::http::header::AUTHORIZATION,
                ]) // Specify allowed headers:cite[2]
                .allow_credentials(true) // If you need to send cookies or authentication headers:cite[2]
                .max_age(std::time::Duration::from_secs(3600)); // Cache the preflight response for 1 hour:cite[2]

            // Dynamically set the allowed origin based on the environment
            match std::env::var(textsender_models::envy::keys::APP_ENV).as_deref() {
                Ok("production") => {
                    let allowed_origins_env =
                        textsender_models::envy::environment::get_allowed_origins().await;
                    match textsender_models::envy::utility::delimitize(&allowed_origins_env) {
                        Ok(alwd) => {
                            let allowed_origins: Vec<axum::http::HeaderValue> = alwd
                                .into_iter()
                                .map(|s| s.parse::<axum::http::HeaderValue>().unwrap())
                                .collect();
                            cors.allow_origin(allowed_origins)
                        }
                        Err(err) => {
                            eprintln!(
                                "Could not parse out allowed origins from env: Error: {err:?}"
                            );
                            std::process::exit(-1);
                        }
                    }
                }
                _ => {
                    // Development (default): Allow localhost origins
                    cors.allow_origin(vec![
                        "http://localhost:4200".parse().unwrap(),
                        "http://127.0.0.1:4200".parse().unwrap(),
                    ])
                }
            }
        }
    }

    pub async fn routes() -> Router {
        // build our application with a route
        Router::new()
            .route(
                callers::endpoints::DBTEST,
                get(callers::common::endpoint::db_ping),
            )
            .route(
                callers::endpoints::ROOT,
                get(callers::common::endpoint::root),
            )
            .route(
                callers::endpoints::REGISTER,
                post(callers::register::register_user),
            )
            .layer(cors::configure_cors().await)
    }

    pub async fn app() -> Router {
        let pool = super::db::init::create_pool()
            .await
            .expect("Failed to create pool");

        super::db::init::migrations(&pool).await;

        routes()
            .await
            .merge(
                utoipa_swagger_ui::SwaggerUi::new("/swagger-ui")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .layer(axum::Extension(pool))
    }
}
