use textsender_auth;
use textsender_auth::callers;
use textsender_auth::db;
use textsender_auth::init;

mod db_mgr {
    use std::str::FromStr;

    pub const LIMIT: usize = 6;

    pub async fn get_pool() -> Result<sqlx::PgPool, sqlx::Error> {
        let tm_db_url = textsender_models::envy::environment::get_db_url()
            .await
            .value;
        let tm_options = sqlx::postgres::PgConnectOptions::from_str(&tm_db_url).unwrap();
        sqlx::PgPool::connect_with(tm_options).await
    }

    pub async fn generate_db_name() -> String {
        let db_name =
            get_database_name().await.unwrap() + &"_" + &uuid::Uuid::new_v4().to_string()[..LIMIT];
        db_name
    }

    pub async fn connect_to_db(db_name: &str) -> Result<sqlx::PgPool, sqlx::Error> {
        let db_url = textsender_models::envy::environment::get_db_url()
            .await
            .value;
        let options = sqlx::postgres::PgConnectOptions::from_str(&db_url)?.database(db_name);
        sqlx::PgPool::connect_with(options).await
    }

    pub async fn create_database(
        template_pool: &sqlx::PgPool,
        db_name: &str,
    ) -> Result<(), sqlx::Error> {
        let create_query = format!("CREATE DATABASE {}", db_name);
        match sqlx::query(&create_query).execute(template_pool).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    // Function to drop a database
    pub async fn drop_database(
        template_pool: &sqlx::PgPool,
        db_name: &str,
    ) -> Result<(), sqlx::Error> {
        let drop_query = format!("DROP DATABASE IF EXISTS {} WITH (FORCE)", db_name);
        sqlx::query(&drop_query).execute(template_pool).await?;
        Ok(())
    }

    pub async fn get_database_name() -> Result<String, Box<dyn std::error::Error>> {
        let database_url = textsender_models::envy::environment::get_db_url()
            .await
            .value;

        let parsed_url = url::Url::parse(&database_url)?;
        if parsed_url.scheme() == "postgres" || parsed_url.scheme() == "postgresql" {
            match parsed_url
                .path_segments()
                .and_then(|segments| segments.last().map(|s| s.to_string()))
            {
                Some(sss) => Ok(sss),
                None => Err("Error parsing".into()),
            }
        } else {
            // Handle other database types if needed
            Err("Error parsing".into())
        }
    }
}

pub mod requests {
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    /// Function to call register user endpoint
    pub async fn register_user(
        app: &axum::Router,
    ) -> Result<axum::response::Response, std::convert::Infallible> {
        let payload = serde_json::json!({
            "username": String::from(super::TEST_USERNAME),
            "password": String::from(super::TEST_PASSWORD),
            "phone_number": String::from(super::TEST_PHONE_NUMBER),
            "firstname": String::from(super::TEST_FIRSTNAME),
            "lastname": String::from(super::TEST_LASTNAME),
        });
        let req = axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri(super::callers::endpoints::REGISTER)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(payload.to_string()))
            .unwrap();

        app.clone().oneshot(req).await
    }

    /// Function to call login user endpoint
    pub async fn login_user(
        app: &axum::Router,
        username: &str,
        password: &str,
    ) -> Result<axum::response::Response, std::convert::Infallible> {
        let payload = serde_json::json!({
            "username": username,
            "password": password,
        });
        let req = axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri(super::callers::endpoints::LOGIN)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(payload.to_string()))
            .unwrap();

        app.clone().oneshot(req).await
    }

    /// Function to call register service user endpoint
    pub async fn register_service_user(
        app: &axum::Router,
    ) -> Result<axum::response::Response, std::convert::Infallible> {
        let payload = serde_json::json!({
            "username": String::from(super::TEST_SERVICE_USERNAME),
            "passphrase": String::from(super::TEST_SERVICE_PASSPHRASE),
        });
        let req = axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri(super::callers::endpoints::REGISTER_SERVICE_USER)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(payload.to_string()))
            .unwrap();

        app.clone().oneshot(req).await
    }

    /// Function to call service user login endpoint
    pub async fn login_service_user(
        app: &axum::Router,
        username: &str,
        passphrase: &str,
    ) -> Result<axum::response::Response, std::convert::Infallible> {
        let payload = serde_json::json!({
            "username": username,
            "passphrase": passphrase,
        });
        let req = axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri(super::callers::endpoints::LOGIN_SERVICE_USER)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(payload.to_string()))
            .unwrap();

        app.clone().oneshot(req).await
    }

    /// Function to call token refresh endpoint
    pub async fn refresh_token(
        app: &axum::Router,
        access_token: &str,
    ) -> Result<axum::response::Response, std::convert::Infallible> {
        let payload = serde_json::json!({
            "access_token": access_token,
        });
        let req = axum::http::Request::builder()
            .method(axum::http::Method::POST)
            .uri(super::callers::endpoints::REFRESH_TOKEN)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(payload.to_string()))
            .unwrap();

        app.clone().oneshot(req).await
    }
}

/// Test user firstname
const TEST_FIRSTNAME: &str = "Billy";
/// Test user lastname
const TEST_LASTNAME: &str = "Bob";
/// Test user username
const TEST_USERNAME: &str = "BillyBob01";
/// Test user password
const TEST_PASSWORD: &str = "923ndcry392qryudx328qrdy328r";
/// Test user phone number
const TEST_PHONE_NUMBER: &str = "+10123456789";

/// Test service username
const TEST_SERVICE_USERNAME: &str = "swoon";
/// Test service passphrase
const TEST_SERVICE_PASSPHRASE: &str = "4n5cf349tfy34w857ty39wq45nfdq23";

mod util {
    pub async fn convert_response<T>(
        response: axum::response::Response,
    ) -> Result<T, std::io::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        match axum::body::to_bytes(response.into_body(), usize::MAX).await {
            Ok(body) => {
                let resp: T = match serde_json::from_slice(&body) {
                    Ok(val) => val,
                    Err(err) => {
                        return Err(std::io::Error::other(err.to_string()));
                    }
                };
                Ok(resp)
            }
            Err(err) => Err(std::io::Error::other(err.to_string())),
        }
    }
}

mod flow {
    use super::callers;
    use super::requests;
    use super::util;

    pub async fn register_user(
        app: &axum::Router,
    ) -> Result<textsender_models::user::User, std::io::Error> {
        match requests::register_user(&app).await {
            Ok(response) => {
                if axum::http::StatusCode::CREATED != response.status() {
                    Err(std::io::Error::other(format!(
                        "Status code is off {:?}",
                        response.status()
                    )))
                } else {
                    match util::convert_response::<callers::register::response::Response>(response)
                        .await
                    {
                        Ok(response) => {
                            if response.data.len() > 0 {
                                let user = response.data[0].clone();
                                Ok(user)
                            } else {
                                Err(std::io::Error::other("No data returned"))
                            }
                        }
                        Err(err) => Err(err),
                    }
                }
            }
            Err(err) => Err(std::io::Error::other(err.to_string())),
        }
    }

    pub async fn login_user(
        app: &axum::Router,
        username: &str,
        password: &str,
    ) -> Result<textsender_models::token::LoginResult, std::io::Error> {
        match requests::login_user(&app, username, password).await {
            Ok(response) => {
                if axum::http::StatusCode::OK != response.status() {
                    Err(std::io::Error::other(format!(
                        "Status code is off {:?}",
                        response.status()
                    )))
                } else {
                    match util::convert_response::<callers::login::response::LoginResponse>(
                        response,
                    )
                    .await
                    {
                        Ok(response) => {
                            if response.data.len() > 0 {
                                let user = response.data[0].clone();
                                Ok(user)
                            } else {
                                Err(std::io::Error::other("No data returned"))
                            }
                        }
                        Err(err) => Err(err),
                    }
                }
            }
            Err(err) => Err(std::io::Error::other(err.to_string())),
        }
    }

    pub async fn register_service_user(
        app: &axum::Router,
    ) -> Result<textsender_models::user::ServiceUser, std::io::Error> {
        match requests::register_service_user(&app).await {
            Ok(response) => {
                if axum::http::StatusCode::CREATED != response.status() {
                    Err(std::io::Error::other(format!(
                        "Status code is off {:?}",
                        response.status()
                    )))
                } else {
                    match util::convert_response::<
                        callers::register::response::RegisterServiceUserResponse,
                    >(response)
                    .await
                    {
                        Ok(response) => {
                            if response.data.len() > 0 {
                                let service_user = response.data[0].clone();
                                Ok(service_user)
                            } else {
                                Err(std::io::Error::other("No data returned"))
                            }
                        }
                        Err(err) => Err(err),
                    }
                }
            }
            Err(err) => Err(std::io::Error::other(err.to_string())),
        }
    }

    pub async fn login_service_user(
        app: &axum::Router,
        username: &str,
        passphrase: &str,
    ) -> Result<textsender_models::token::LoginResult, std::io::Error> {
        match requests::login_service_user(&app, username, passphrase).await {
            Ok(response) => {
                if axum::http::StatusCode::OK != response.status() {
                    Err(std::io::Error::other(format!(
                        "Status code is off {:?}",
                        response.status()
                    )))
                } else {
                    match util::convert_response::<callers::login::response::ServiceUserLoginResponse>(
                        response,
                    )
                    .await
                    {
                        Ok(response) => {
                            if response.data.len() > 0 {
                                let login_result = response.data[0].clone();
                                Ok(login_result)
                            } else {
                                Err(std::io::Error::other("No data returned"))
                            }
                        }
                        Err(err) => Err(err),
                    }
                }
            }
            Err(err) => Err(std::io::Error::other(err.to_string())),
        }
    }

    pub async fn refresh_token(
        app: &axum::Router,
        access_token: &str,
    ) -> Result<textsender_models::token::LoginResult, std::io::Error> {
        match requests::refresh_token(app, access_token).await {
            Ok(response) => {
                if axum::http::StatusCode::OK != response.status() {
                    Err(std::io::Error::other(format!(
                        "Status code is off {:?}",
                        response.status()
                    )))
                } else {
                    match util::convert_response::<callers::login::response::RefreshTokenResponse>(
                        response,
                    )
                    .await
                    {
                        Ok(response) => {
                            if response.data.len() > 0 {
                                let login_result = response.data[0].clone();
                                Ok(login_result)
                            } else {
                                Err(std::io::Error::other("No data returned"))
                            }
                        }
                        Err(err) => Err(err),
                    }
                }
            }
            Err(err) => Err(std::io::Error::other(err.to_string())),
        }
    }
}

#[tokio::test]
async fn test_register_user() {
    let tm_pool = db_mgr::get_pool().await.unwrap();
    let db_name = db_mgr::generate_db_name().await;

    match db_mgr::create_database(&tm_pool, &db_name).await {
        Ok(_) => {
            println!("Success");
        }
        Err(e) => {
            assert!(false, "Error: {:?}", e.to_string());
        }
    }

    let pool = db_mgr::connect_to_db(&db_name).await.unwrap();

    db::init::migrations(&pool).await;

    let app = init::routes().await.layer(axum::Extension(pool));

    match flow::register_user(&app).await {
        Ok(returned_user) => {
            assert_eq!(
                TEST_USERNAME, returned_user.username,
                "Error with returned user"
            );
        }
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }

    match db_mgr::drop_database(&tm_pool, &db_name).await {
        Ok(()) => {}
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }
}

#[tokio::test]
async fn test_login_user() {
    let tm_pool = db_mgr::get_pool().await.unwrap();
    let db_name = db_mgr::generate_db_name().await;

    match db_mgr::create_database(&tm_pool, &db_name).await {
        Ok(_) => {
            println!("Success");
        }
        Err(e) => {
            assert!(false, "Error: {:?}", e.to_string());
        }
    }

    let pool = db_mgr::connect_to_db(&db_name).await.unwrap();

    db::init::migrations(&pool).await;

    let app = init::routes().await.layer(axum::Extension(pool));

    match flow::register_user(&app).await {
        Ok(user) => match flow::login_user(&app, &user.username, TEST_PASSWORD).await {
            Ok(login_result) => {
                assert_eq!(
                    false,
                    login_result.access_token.is_empty(),
                    "Access token is empty when it should not be"
                );
            }
            Err(err) => {
                assert!(false, "Error: {err:?}");
            }
        },
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }

    match db_mgr::drop_database(&tm_pool, &db_name).await {
        Ok(()) => {}
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }
}

#[tokio::test]
async fn test_register_service_user() {
    let tm_pool = db_mgr::get_pool().await.unwrap();
    let db_name = db_mgr::generate_db_name().await;

    match db_mgr::create_database(&tm_pool, &db_name).await {
        Ok(_) => {
            println!("Success");
        }
        Err(e) => {
            assert!(false, "Error: {:?}", e.to_string());
        }
    }

    let pool = db_mgr::connect_to_db(&db_name).await.unwrap();

    db::init::migrations(&pool).await;

    let app = init::routes().await.layer(axum::Extension(pool));

    match requests::register_service_user(&app).await {
        Ok(response) => {
            match util::convert_response::<callers::register::response::RegisterServiceUserResponse>(
                response,
            )
            .await
            {
                Ok(resp) => {
                    assert!(resp.data.len() > 0, "No service user was created");
                    let service_user = &resp.data[0];
                    assert_eq!(
                        TEST_SERVICE_USERNAME, service_user.username,
                        "Service username does not match"
                    );
                }
                Err(err) => {
                    assert!(false, "Error: {err:?}");
                }
            }
        }
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }

    match db_mgr::drop_database(&tm_pool, &db_name).await {
        Ok(()) => {}
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }
}

#[tokio::test]
async fn test_login_service_user() {
    let tm_pool = db_mgr::get_pool().await.unwrap();
    let db_name = db_mgr::generate_db_name().await;

    match db_mgr::create_database(&tm_pool, &db_name).await {
        Ok(_) => {
            println!("Success");
        }
        Err(e) => {
            assert!(false, "Error: {:?}", e.to_string());
        }
    }

    let pool = db_mgr::connect_to_db(&db_name).await.unwrap();

    db::init::migrations(&pool).await;

    let app = init::routes().await.layer(axum::Extension(pool));

    match flow::register_service_user(&app).await {
        Ok(user) => {
            assert_eq!(
                false,
                user.id.is_nil(),
                "The service user id should not be nil"
            );
            match flow::login_service_user(&app, TEST_SERVICE_USERNAME, TEST_SERVICE_PASSPHRASE)
                .await
            {
                Ok(login_result) => {
                    assert_eq!(
                        false,
                        login_result.access_token.is_empty(),
                        "Access token is empty when it should not be"
                    );
                }
                Err(err) => {
                    assert!(false, "Error: {err:?}");
                }
            }
        }
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }

    match db_mgr::drop_database(&tm_pool, &db_name).await {
        Ok(()) => {}
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }
}

#[tokio::test]
async fn test_refresh_token() {
    let tm_pool = db_mgr::get_pool().await.unwrap();
    let db_name = db_mgr::generate_db_name().await;

    match db_mgr::create_database(&tm_pool, &db_name).await {
        Ok(_) => {
            println!("Success");
        }
        Err(e) => {
            assert!(false, "Error: {:?}", e.to_string());
        }
    }

    let pool = db_mgr::connect_to_db(&db_name).await.unwrap();

    db::init::migrations(&pool).await;

    let app = init::routes().await.layer(axum::Extension(pool));

    match flow::register_user(&app).await {
        Ok(user) => match flow::login_user(&app, &user.username, TEST_PASSWORD).await {
            Ok(login_result) => {
                assert_eq!(
                    false,
                    login_result.access_token.is_empty(),
                    "Access token is empty when it should not be"
                );
                match flow::refresh_token(&app, &login_result.access_token).await {
                    Ok(refresh_login_result) => {
                        assert_eq!(
                            false,
                            refresh_login_result.access_token.is_empty(),
                            "Refreshed access token should not be empty"
                        );
                    }
                    Err(err) => {
                        assert!(false, "Error: {err:?}");
                    }
                }
            }
            Err(err) => {
                assert!(false, "Error: {err:?}");
            }
        },
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }

    match flow::register_service_user(&app).await {
        Ok(user) => {
            assert_eq!(
                false,
                user.id.is_nil(),
                "The service user id should not be nil"
            );
            match flow::login_service_user(&app, TEST_SERVICE_USERNAME, TEST_SERVICE_PASSPHRASE)
                .await
            {
                Ok(login_result) => {
                    assert_eq!(
                        false,
                        login_result.access_token.is_empty(),
                        "Access token is empty when it should not be"
                    );

                    match flow::refresh_token(&app, &login_result.access_token).await {
                        Ok(refresh_login_result) => {
                            assert_eq!(
                                false,
                                refresh_login_result.access_token.is_empty(),
                                "Refreshed access token should not be empty"
                            );
                        }
                        Err(err) => {
                            assert!(false, "Error: {err:?}");
                        }
                    }
                }
                Err(err) => {
                    assert!(false, "Error: {err:?}");
                }
            }
        }
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }

    match db_mgr::drop_database(&tm_pool, &db_name).await {
        Ok(()) => {}
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }
}
