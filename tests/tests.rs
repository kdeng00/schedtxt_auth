use schedtxt_auth;
use schedtxt_auth::callers;
use schedtxt_auth::db;
use schedtxt_auth::init;

mod db_mgr {
    use std::str::FromStr;

    pub const LIMIT: usize = 6;

    pub async fn get_pool() -> Result<sqlx::PgPool, sqlx::Error> {
        let tm_db_url = schedtxt_models::envy::environment::get_db_url().value;
        let tm_options = sqlx::postgres::PgConnectOptions::from_str(&tm_db_url).unwrap();
        sqlx::PgPool::connect_with(tm_options).await
    }

    pub async fn generate_db_name() -> String {
        let db_name =
            get_database_name().await.unwrap() + &"_" + &uuid::Uuid::new_v4().to_string()[..LIMIT];
        db_name
    }

    pub async fn connect_to_db(db_name: &str) -> Result<sqlx::PgPool, sqlx::Error> {
        let db_url = schedtxt_models::envy::environment::get_db_url().value;
        let options = sqlx::postgres::PgConnectOptions::from_str(&db_url)?.database(db_name);
        sqlx::PgPool::connect_with(options).await
    }

    pub async fn create_database(
        template_pool: &sqlx::PgPool,
        db_name: &str,
    ) -> Result<(), sqlx::Error> {
        let create_query = format!("CREATE DATABASE {}", db_name);
        match sqlx::query(sqlx::AssertSqlSafe(create_query))
            .execute(template_pool)
            .await
        {
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
        sqlx::query(sqlx::AssertSqlSafe(drop_query))
            .execute(template_pool)
            .await?;
        Ok(())
    }

    pub async fn get_database_name() -> Result<String, Box<dyn std::error::Error>> {
        let database_url = schedtxt_models::envy::environment::get_db_url().value;

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

    /// Function to call update password endpoint
    pub async fn update_password(
        app: &axum::Router,
        user_id: &uuid::Uuid,
        current_password: &str,
        updated_password: &str,
        confirmed_password: &str,
    ) -> Result<axum::response::Response, axum::http::Error> {
        let payload = serde_json::json!({
            "user_id": user_id,
            "current_password": current_password,
            "updated_password": updated_password,
            "confirmed_password": confirmed_password,
        });
        match axum::http::Request::builder()
            .method(axum::http::Method::PATCH)
            .uri(super::callers::endpoints::UPDATE_PASSWORD)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(payload.to_string()))
        {
            Ok(req) => match app.clone().oneshot(req).await {
                Ok(resp) => Ok(resp),
                Err(err) => Err(axum::http::Error::from(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Function to call update name of user endpoint
    pub async fn update_name_of_user(
        app: &axum::Router,
        user_id: &uuid::Uuid,
        firstname: &str,
        lastname: &str,
    ) -> Result<axum::response::Response, axum::http::Error> {
        let payload = serde_json::json!({
            "user_id": user_id,
            "firstname": firstname,
            "lastname": lastname,
        });
        match axum::http::Request::builder()
            .method(axum::http::Method::PATCH)
            .uri(super::callers::endpoints::UPDATE_USER_NAME)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::from(payload.to_string()))
        {
            Ok(req) => match app.clone().oneshot(req).await {
                Ok(resp) => Ok(resp),
                Err(err) => Err(axum::http::Error::from(err)),
            },
            Err(err) => Err(err),
        }
    }

    /// Function to call get user profile endpoint
    pub async fn get_user_profile(
        app: &axum::Router,
        user_id: &uuid::Uuid,
    ) -> Result<axum::response::Response, axum::http::Error> {
        let url = super::util::format_url_with_value(
            schedtxt_auth::callers::endpoints::GET_USER_PROFILE,
            user_id,
        );
        match axum::http::Request::builder()
            .method(axum::http::Method::GET)
            .uri(url)
            .header(axum::http::header::CONTENT_TYPE, "application/json")
            .body(axum::body::Body::empty())
        {
            Ok(req) => match app.clone().oneshot(req).await {
                Ok(resp) => Ok(resp),
                Err(err) => Err(axum::http::Error::from(err)),
            },
            Err(err) => Err(err),
        }
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
/// Test updated user password
const TEST_UPDATED_PASSWORD: &str = "3cnf29ry8q27i3yrc928qi37ryndxc2198q7yd9xzq12837e";

/// Test updated user firstname
const TEST_UPDATED_FIRSTNAME: &str = "Kuoth";
/// Test updated user lastname
const TEST_UPDATED_LASTNAME: &str = "Wech";

/// Test service username
const TEST_SERVICE_USERNAME: &str = "swoon";
/// Test service passphrase
const TEST_SERVICE_PASSPHRASE: &str = "4n5cf349tfy34w857ty39wq45nfdq23";
/// Test updated service user passphrase
const TEST_SERVICE_UPDATED_PASSPHRASE: &str = "3487ncfyth934287fcrty32487fry32in7";

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

    pub fn format_url_with_value(endpoint: &str, value: &uuid::Uuid) -> String {
        let last = endpoint.len() - 5;
        format!("{}/{value}", &endpoint[0..last])
    }
}

mod flow {
    use super::callers;
    use super::requests;
    use super::util;

    pub async fn register_user(
        app: &axum::Router,
    ) -> Result<schedtxt_models::user::User, std::io::Error> {
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
    ) -> Result<schedtxt_models::token::LoginResult, std::io::Error> {
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
    ) -> Result<schedtxt_models::user::ServiceUser, std::io::Error> {
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
    ) -> Result<schedtxt_models::token::LoginResult, std::io::Error> {
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
    ) -> Result<schedtxt_models::token::LoginResult, std::io::Error> {
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

    pub async fn update_password(
        app: &axum::Router,
        user_id: &uuid::Uuid,
        current_password: &str,
        updated_password: &str,
        confirmed_password: &str,
    ) -> Result<uuid::Uuid, std::io::Error> {
        match requests::update_password(
            app,
            user_id,
            current_password,
            updated_password,
            confirmed_password,
        )
        .await
        {
            Ok(response) => {
                if axum::http::StatusCode::OK != response.status() {
                    Err(std::io::Error::other(format!(
                        "Status code is off {:?}",
                        response.status()
                    )))
                } else {
                    match util::convert_response::<callers::login::response::UpdatePasswordResponse>(
                        response,
                    )
                    .await
                    {
                        Ok(response) => {
                            if response.data.len() > 0 {
                                let id = response.data[0].clone();
                                Ok(id)
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

    pub async fn update_name_of_user(
        app: &axum::Router,
        user_id: &uuid::Uuid,
        firstname: &str,
        lastname: &str,
    ) -> Result<schedtxt_models::user::User, std::io::Error> {
        match requests::update_name_of_user(app, user_id, firstname, lastname).await {
            Ok(response) => {
                if axum::http::StatusCode::OK != response.status() {
                    Err(std::io::Error::other(format!(
                        "Status code is off {:?}",
                        response.status()
                    )))
                } else {
                    match util::convert_response::<callers::login::response::UserUpdateNameResponse>(
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

#[tokio::test]
async fn test_update_password() {
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

                match flow::update_password(
                    &app,
                    &user.id,
                    TEST_PASSWORD,
                    TEST_UPDATED_PASSWORD,
                    TEST_UPDATED_PASSWORD,
                )
                .await
                {
                    Ok(id) => {
                        assert_eq!(id, user.id, "Ids do not match");
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
        Ok(service_user) => {
            assert_eq!(
                false,
                service_user.id.is_nil(),
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

                    match flow::update_password(
                        &app,
                        &service_user.id,
                        TEST_SERVICE_PASSPHRASE,
                        TEST_SERVICE_UPDATED_PASSPHRASE,
                        TEST_SERVICE_UPDATED_PASSPHRASE,
                    )
                    .await
                    {
                        Ok(id) => {
                            assert_eq!(id, service_user.id, "Ids do not match");
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

#[tokio::test]
async fn test_update_name_of_password() {
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

                match flow::update_name_of_user(
                    &app,
                    &user.id,
                    TEST_UPDATED_FIRSTNAME,
                    TEST_UPDATED_LASTNAME,
                )
                .await
                {
                    Ok(user) => {
                        assert_eq!(
                            TEST_UPDATED_FIRSTNAME, user.firstname,
                            "Firstname do not match"
                        );
                        assert_eq!(
                            TEST_UPDATED_LASTNAME, user.lastname,
                            "Lastname do not match"
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

    match db_mgr::drop_database(&tm_pool, &db_name).await {
        Ok(()) => {}
        Err(err) => {
            assert!(false, "Error: {err:?}");
        }
    }
}

#[tokio::test]
async fn test_get_user_profile() {
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

    let user_id = match flow::register_user(&app).await {
        Ok(user) => match flow::login_user(&app, &user.username, TEST_PASSWORD).await {
            Ok(login_result) => {
                assert_eq!(
                    false,
                    login_result.access_token.is_empty(),
                    "Access token is empty when it should not be"
                );
                login_result.user_id
            }
            Err(err) => {
                assert!(false, "Error: {err:?}");
                uuid::Uuid::nil()
            }
        },
        Err(err) => {
            assert!(false, "Error: {err:?}");
            uuid::Uuid::nil()
        }
    };

    assert_eq!(false, user_id.is_nil(), "User Id should not be empty");

    match requests::get_user_profile(&app, &user_id).await {
        Ok(response) => {
            match util::convert_response::<
                schedtxt_auth::callers::login::response::GetUserProfileResponse,
            >(response)
            .await
            {
                Ok(response) => {
                    assert_eq!(false, response.data.is_empty(), "No User Profile found");
                    let user_profile = &response.data[0];
                    assert_eq!(
                        TEST_USERNAME, user_profile.username,
                        "Username does not match"
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
