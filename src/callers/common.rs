pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, utoipa::ToSchema)]
    pub struct TestResult {
        pub message: String,
    }
}

pub mod endpoint {
    use super::*;
    use axum::{Extension, Json, http::StatusCode};

    /// Endpoint to hit the root
    /// basic handler that responds with a static string
    #[utoipa::path(
        get,
        path = super::super::endpoints::ROOT,
        responses(
            (status = 200, description = "Test", body = &str),
        )
    )]
    pub async fn root() -> &'static str {
        "Hello, World!"
    }

    /// Endpoint to do a database ping
    #[utoipa::path(
        get,
        path = super::super::endpoints::DBTEST,
        responses(
            (status = 200, description = "Successful ping of the db", body = super::response::TestResult),
            (status = 400, description = "Failure in pinging the db", body = super::response::TestResult)
        )
    )]
    pub async fn db_ping(
        Extension(pool): Extension<sqlx::PgPool>,
    ) -> (StatusCode, Json<response::TestResult>) {
        match sqlx::query("SELECT 1").execute(&pool).await {
            Ok(_) => {
                let tr = response::TestResult {
                    message: String::from("This works"),
                };
                (StatusCode::OK, Json(tr))
            }
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(response::TestResult {
                    message: e.to_string(),
                }),
            ),
        }
    }
}
