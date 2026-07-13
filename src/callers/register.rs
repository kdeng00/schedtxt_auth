use axum::{Json, http::StatusCode};

use crate::hashing;
use crate::repo;

pub mod request {
    use serde::{Deserialize, Serialize};

    #[derive(Default, Deserialize, Serialize, utoipa::ToSchema)]
    pub struct Request {
        pub username: String,
        pub password: String,
        pub phone_number: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub firstname: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub lastname: Option<String>,
    }

    #[derive(Default, Deserialize, Serialize, utoipa::ToSchema)]
    pub struct RegisterServiceUserRequest {
        pub username: String,
        pub passphrase: String,
    }

    impl RegisterServiceUserRequest {
        pub fn is_empty(&self) -> (bool, Option<String>) {
            if self.username.is_empty() && self.passphrase.is_empty() {
                (
                    true,
                    Some(String::from("Username and Passphrase are empty")),
                )
            } else if self.username.is_empty() {
                (true, Some(String::from("Username is empty")))
            } else if self.passphrase.is_empty() {
                (true, Some(String::from("Passphrase is empty")))
            } else {
                (false, None)
            }
        }
    }
}

pub mod response {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize, utoipa::ToSchema)]
    pub struct Response {
        pub message: String,
        pub data: Vec<schedtxt_models::user::User>,
    }

    #[derive(Default, Deserialize, Serialize, utoipa::ToSchema)]
    pub struct RegisterServiceUserResponse {
        pub message: String,
        pub data: Vec<schedtxt_models::user::ServiceUser>,
    }
}

pub fn generate_the_salt() -> (
    argon2::password_hash::SaltString,
    schedtxt_models::user::Salt,
) {
    let salt_string = hashing::generate_salt().unwrap();
    let salt = schedtxt_models::user::Salt::default();
    (salt_string, salt)
}

/// Endpoint to register a user
#[utoipa::path(
    post,
    path = super::endpoints::REGISTER,
    request_body(
        content = request::Request,
        description = "Data required to register",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "User created", body = response::Response),
        (status = 404, description = "User already exists", body = response::Response),
        (status = 500, description = "Issue creating user", body = response::Response)
    )
)]
pub async fn register_user(
    axum::Extension(pool): axum::Extension<sqlx::PgPool>,
    Json(payload): Json<request::Request>,
) -> (StatusCode, Json<response::Response>) {
    let registration_enabled = match is_registration_enabled().await {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error: {err:?}");
            return (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(response::Response {
                    message: String::from("Registration check failed"),
                    data: Vec::new(),
                }),
            );
        }
    };

    if registration_enabled {
        let mut user = schedtxt_models::user::User {
            username: payload.username.clone(),
            password: payload.password.clone(),
            phone_number: payload.phone_number.clone(),
            ..Default::default()
        };

        user.firstname = payload.firstname.unwrap_or_default();
        user.lastname = payload.lastname.unwrap_or_default();

        println!("Checking if user exists");
        match repo::user::exists(&pool, &user.username).await {
            Ok(res) => {
                if res {
                    println!("Already exists");
                    (
                        StatusCode::BAD_REQUEST,
                        Json(response::Response {
                            message: String::from("Error"),
                            data: Vec::new(),
                        }),
                    )
                } else {
                    println!("Good to create");
                    println!("Generate salt string");

                    let (generated_salt, mut salt) = generate_the_salt();
                    println!("Creating salt");
                    salt.id = repo::salt::insert(&pool, &salt).await.unwrap();
                    user.salt_id = salt.id;
                    let hashed_password =
                        hashing::hash_password(&user.password, &generated_salt).unwrap();
                    user.password = hashed_password;

                    println!("Creating user");
                    match repo::user::insert(&pool, &user).await {
                        Ok((id, date_created)) => {
                            user.id = id;
                            user.created = date_created;
                            (
                                StatusCode::CREATED,
                                Json(response::Response {
                                    message: String::from("User created"),
                                    data: vec![user],
                                }),
                            )
                        }
                        Err(err) => (
                            StatusCode::BAD_REQUEST,
                            Json(response::Response {
                                message: err.to_string(),
                                data: vec![user],
                            }),
                        ),
                    }
                }
            }
            Err(err) => (
                StatusCode::BAD_REQUEST,
                Json(response::Response {
                    message: err.to_string(),
                    data: vec![user],
                }),
            ),
        }
    } else {
        (
            axum::http::StatusCode::NOT_ACCEPTABLE,
            Json(response::Response {
                message: String::from("Registration is not enabled"),
                data: Vec::new(),
            }),
        )
    }
}

/// Checks to see if registration is enabled
async fn is_registration_enabled() -> Result<bool, std::io::Error> {
    let key = String::from("ENABLE_REGISTRATION");
    let var = schedtxt_models::envy::environment::get_env(&key);
    let parsed_value = var.value.to_uppercase();

    if parsed_value == "TRUE" {
        Ok(true)
    } else if parsed_value == "FALSE" {
        Ok(false)
    } else {
        Err(std::io::Error::other(
            "Could not determine value of ENABLE_REGISTRATION",
        ))
    }
}

/// Endpoint to register a service user
#[utoipa::path(
    post,
    path = super::endpoints::REGISTER_SERVICE_USER,
    request_body(
        content = request::RegisterServiceUserRequest,
        description = "Data required to register service user",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "Service user created", body = response::RegisterServiceUserResponse),
        (status = 400, description = "Issue creating service user", body = response::RegisterServiceUserResponse),
        (status = 406, description = "Cannot create service user", body = response::RegisterServiceUserResponse),
        (status = 500, description = "Issue creating service user", body = response::RegisterServiceUserResponse),
    )
)]
pub async fn register_service_user(
    axum::Extension(pool): axum::Extension<sqlx::PgPool>,
    Json(payload): Json<request::RegisterServiceUserRequest>,
) -> (
    axum::http::StatusCode,
    axum::Json<response::RegisterServiceUserResponse>,
) {
    let mut resp = response::RegisterServiceUserResponse {
        ..Default::default()
    };

    let registration_enabled = match is_registration_enabled().await {
        Ok(value) => value,
        Err(err) => {
            eprintln!("Error: {err:?}");
            resp.message = String::from("Registration check failed");
            return (axum::http::StatusCode::INTERNAL_SERVER_ERROR, Json(resp));
        }
    };

    let (res, msg) = payload.is_empty();
    if res {
        resp.message = msg.unwrap();
        (axum::http::StatusCode::BAD_REQUEST, axum::Json(resp))
    } else {
        if registration_enabled {
            match repo::service::exists(&pool, &payload.username).await {
                Ok(exists) => {
                    if exists {
                        resp.message = String::from("Invalid");
                        (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            axum::Json(resp),
                        )
                    } else {
                        let (generate_salt, mut salt) = generate_the_salt();
                        salt.id = repo::salt::insert(&pool, &salt).await.unwrap();
                        let mut service_user = schedtxt_models::user::ServiceUser {
                            username: payload.username.clone(),
                            passphrase: hashing::hash_password(&payload.passphrase, &generate_salt)
                                .unwrap(),
                            salt_id: salt.id,
                            ..Default::default()
                        };

                        println!("Creating user");

                        match repo::service::insert(&pool, &service_user).await {
                            Ok((service_user_id, created)) => {
                                resp.message = String::from(super::messages::SUCCESSFUL_MESSAGE);
                                service_user.created = Some(created);
                                service_user.id = service_user_id;
                                resp.data.push(service_user);
                                (axum::http::StatusCode::CREATED, axum::Json(resp))
                            }
                            Err(err) => {
                                resp.message = err.to_string();
                                (
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                    axum::Json(resp),
                                )
                            }
                        }
                    }
                }
                Err(err) => {
                    resp.message = err.to_string();
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        axum::Json(resp),
                    )
                }
            }
        } else {
            resp.message = String::from("Registration is not enabled");
            (axum::http::StatusCode::NOT_ACCEPTABLE, Json(resp))
        }
    }
}
