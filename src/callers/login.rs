use crate::hashing;
use crate::repo;
use crate::token_stuff;

pub mod request {
    use serde::Deserialize;
    #[derive(Default, Deserialize, utoipa::ToSchema)]
    pub struct LoginRequest {
        pub username: String,
        pub password: String,
    }

    #[derive(Deserialize, utoipa::ToSchema)]
    pub struct ServiceUserLoginRequest {
        pub username: String,
        pub passphrase: String,
    }

    impl ServiceUserLoginRequest {
        pub fn is_empty(&self) -> (bool, Option<String>) {
            if self.username.is_empty() && self.passphrase.is_empty() {
                (
                    true,
                    Some(String::from("Username and passphrase are empty")),
                )
            } else if self.username.is_empty() {
                (true, Some(String::from("Username is empty")))
            } else if self.username.is_empty() {
                (true, Some(String::from("Passphrase is empty")))
            } else {
                (false, None)
            }
        }
    }

    #[derive(Deserialize, utoipa::ToSchema)]
    pub struct RefreshTokenRequest {
        pub access_token: String,
    }

    #[derive(Deserialize, utoipa::ToSchema)]
    pub struct UpdatePasswordRequest {
        pub user_id: uuid::Uuid,
        pub current_password: String,
        pub updated_password: String,
        pub confirmed_password: String,
    }

    impl UpdatePasswordRequest {
        pub fn is_valid(&self) -> bool {
            if self.user_id.is_nil()
                || self.current_password.is_empty()
                || self.updated_password.is_empty()
                || self.confirmed_password.is_empty()
            {
                false
            } else {
                self.updated_password == self.confirmed_password
            }
        }
    }

    #[derive(Deserialize, utoipa::ToSchema)]
    pub struct UserUpdateNameRequest {
        pub user_id: uuid::Uuid,
        pub firstname: String,
        pub lastname: String,
    }

    impl UserUpdateNameRequest {
        pub fn is_valid(&self) -> (bool, Option<String>) {
            if self.user_id.is_nil() || self.firstname.is_empty() || self.lastname.is_empty() {
                let reason = String::from("Missing fields");
                (false, Some(reason))
            } else {
                (true, None)
            }
        }
    }
}

pub mod response {
    use serde::{Deserialize, Serialize};
    #[derive(Default, Deserialize, Serialize, utoipa::ToSchema)]
    pub struct LoginResponse {
        pub message: String,
        pub data: Vec<schedtxt_models::token::LoginResult>,
    }

    #[derive(Default, Deserialize, Serialize, utoipa::ToSchema)]
    pub struct ServiceUserLoginResponse {
        pub message: String,
        pub data: Vec<schedtxt_models::token::LoginResult>,
    }

    pub async fn extract(
        response: axum::response::Response,
    ) -> Result<LoginResponse, std::io::Error> {
        let body = match axum::body::to_bytes(response.into_body(), usize::MAX).await {
            Ok(body) => body,
            Err(err) => {
                return Err(std::io::Error::other(err));
            }
        };
        let parsed_body: LoginResponse = match serde_json::from_slice(&body) {
            Ok(body) => body,
            Err(err) => {
                return Err(std::io::Error::other(err));
            }
        };
        Ok(parsed_body)
    }

    #[derive(Deserialize, Serialize, utoipa::ToSchema)]
    pub struct RefreshTokenResponse {
        pub message: String,
        pub data: Vec<schedtxt_models::token::LoginResult>,
    }

    #[derive(Deserialize, Serialize, utoipa::ToSchema)]
    pub struct UpdatePasswordResponse {
        pub message: String,
        pub data: Vec<uuid::Uuid>,
    }

    #[derive(Deserialize, Serialize, utoipa::ToSchema)]
    pub struct UserUpdateNameResponse {
        pub message: String,
        pub data: Vec<schedtxt_models::user::User>,
    }

    #[derive(Default, Deserialize, Serialize, utoipa::ToSchema)]
    pub struct GetUserProfileResponse {
        pub message: String,
        pub data: Vec<schedtxt_models::user::UserProfile>,
    }
}

/// Endpoint for a user login
#[utoipa::path(
    post,
    path = super::endpoints::LOGIN,
    request_body(
        content = request::LoginRequest,
        description = "Data required for a user to lgoin",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "User login successful", body = response::LoginResponse),
        (status = 400, description = "Bad data", body = response::LoginResponse),
        (status = 500, description = "Something went wrong", body = response::LoginResponse)
    )
)]
pub async fn user_login(
    axum::Extension(pool): axum::Extension<sqlx::PgPool>,
    axum::Json(payload): axum::Json<request::LoginRequest>,
) -> (axum::http::StatusCode, axum::Json<response::LoginResponse>) {
    if payload.username.is_empty() || payload.password.is_empty() {
        let reason = if payload.username.is_empty() {
            String::from("Username not provided")
        } else {
            String::from("Password not provided")
        };

        (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(response::LoginResponse {
                message: reason,
                data: Vec::new(),
            }),
        )
    } else {
        match repo::user::exists(&pool, &payload.username).await {
            Ok(exists) => {
                if !exists {
                    println!("User does not exists");
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        axum::Json(response::LoginResponse {
                            message: String::from("Unable to login"),
                            data: Vec::new(),
                        }),
                    )
                } else {
                    let user = match repo::user::get(&pool, &payload.username).await {
                        Ok(user) => user,
                        Err(_err) => {
                            return (
                                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                axum::Json(response::LoginResponse {
                                    message: String::from("Unable to login"),
                                    data: Vec::new(),
                                }),
                            );
                        }
                    };
                    let hashed_password = user.password.clone();

                    match hashing::verify_password(&payload.password, hashed_password) {
                        Ok(matches) => {
                            if matches {
                                // Create token
                                let key =
                                    schedtxt_models::envy::environment::get_secret_key().value;
                                let cst = match token_stuff::create_token(&key, &user.id) {
                                    Ok(token) => token,
                                    Err(_err) => {
                                        return (
                                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                            axum::Json(response::LoginResponse {
                                                message: String::from("Error creating token"),
                                                data: Vec::new(),
                                            }),
                                        );
                                    }
                                };

                                if token_stuff::verify_token(&key, &cst.access_token) {
                                    let current_time = time::OffsetDateTime::now_utc();
                                    let _ =
                                        repo::user::update_last_login(&pool, &user, &current_time)
                                            .await;

                                    (
                                        axum::http::StatusCode::OK,
                                        axum::Json(response::LoginResponse {
                                            message: String::from("Successful"),
                                            data: vec![schedtxt_models::token::LoginResult {
                                                user_id: user.id,
                                                access_token: cst.access_token,
                                                token_type: String::from(
                                                    schedtxt_models::token::TOKEN_TYPE,
                                                ),
                                                issued_at: cst.issued,
                                                expires_in: cst.expires_in,
                                            }],
                                        }),
                                    )
                                } else {
                                    (
                                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        axum::Json(response::LoginResponse {
                                            message: String::from("Invalid attempt"),
                                            data: Vec::new(),
                                        }),
                                    )
                                }
                            } else {
                                (
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                    axum::Json(response::LoginResponse {
                                        message: String::from("Invalid attempt"),
                                        data: Vec::new(),
                                    }),
                                )
                            }
                        }
                        Err(err) => (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            axum::Json(response::LoginResponse {
                                message: err.to_string(),
                                data: Vec::new(),
                            }),
                        ),
                    }
                }
            }
            Err(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(response::LoginResponse {
                    message: err.to_string(),
                    data: Vec::new(),
                }),
            ),
        }
    }
}

/// Endpoint for service user login
#[utoipa::path(
    post,
    path = super::endpoints::LOGIN_SERVICE_USER,
    request_body(
        content = request::ServiceUserLoginRequest,
        description = "Data required for service user to lgoin",
        content_type = "application/json"
    ),
    responses(
        (status = 201, description = "Service uuser login successful", body = response::ServiceUserLoginResponse),
        (status = 400, description = "Bad data", body = response::ServiceUserLoginResponse),
        (status = 500, description = "Something went wrong", body = response::ServiceUserLoginResponse)
    )
)]
pub async fn service_user_login(
    axum::Extension(pool): axum::Extension<sqlx::PgPool>,
    axum::Json(payload): axum::Json<request::ServiceUserLoginRequest>,
) -> (
    axum::http::StatusCode,
    axum::Json<response::ServiceUserLoginResponse>,
) {
    if payload.username.is_empty() || payload.passphrase.is_empty() {
        let reason = if payload.username.is_empty() {
            String::from("Username not provided")
        } else {
            String::from("Passphrase not provided")
        };

        (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(response::ServiceUserLoginResponse {
                message: reason,
                data: Vec::new(),
            }),
        )
    } else {
        match repo::service::exists(&pool, &payload.username).await {
            Ok(exists) => {
                if !exists {
                    println!("User does not exists");
                    (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        axum::Json(response::ServiceUserLoginResponse {
                            message: String::from("Unable to login"),
                            data: Vec::new(),
                        }),
                    )
                } else {
                    println!("Good to create");
                    let service_user =
                        match repo::service::get_with_username(&pool, &payload.username).await {
                            Ok(service_user) => service_user,
                            Err(err) => {
                                eprintln!("Error: {err:?}");
                                return (
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                    axum::Json(response::ServiceUserLoginResponse {
                                        message: String::from("Unable to login"),
                                        data: Vec::new(),
                                    }),
                                );
                            }
                        };
                    println!("Service user: {service_user:?}");
                    println!("Payload: {:?}", payload.passphrase);
                    let hashed_password = service_user.passphrase.clone();
                    println!("Hash password: {hashed_password:?}");

                    match hashing::verify_password(&payload.passphrase, hashed_password) {
                        Ok(matches) => {
                            if matches {
                                // Create token
                                println!("Creating token");
                                let key =
                                    schedtxt_models::envy::environment::get_secret_key().value;
                                let cst =
                                    token_stuff::create_token(&key, &service_user.id).unwrap();

                                if token_stuff::verify_token(&key, &cst.access_token) {
                                    let current_time = time::OffsetDateTime::now_utc();
                                    let _ = repo::service::update_last_login(
                                        &pool,
                                        &service_user,
                                        &current_time,
                                    )
                                    .await;

                                    (
                                        axum::http::StatusCode::OK,
                                        axum::Json(response::ServiceUserLoginResponse {
                                            message: String::from(
                                                super::messages::SUCCESSFUL_MESSAGE,
                                            ),
                                            data: vec![schedtxt_models::token::LoginResult {
                                                user_id: service_user.id,
                                                access_token: cst.access_token,
                                                token_type: String::from(
                                                    schedtxt_models::token::TOKEN_TYPE,
                                                ),
                                                issued_at: cst.issued,
                                                expires_in: cst.expires_in,
                                            }],
                                        }),
                                    )
                                } else {
                                    eprintln!("Invalid token");
                                    (
                                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        axum::Json(response::ServiceUserLoginResponse {
                                            message: String::from("Invalid attempt"),
                                            data: Vec::new(),
                                        }),
                                    )
                                }
                            } else {
                                eprintln!("No match");
                                (
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                    axum::Json(response::ServiceUserLoginResponse {
                                        message: String::from("Invalid attempt"),
                                        data: Vec::new(),
                                    }),
                                )
                            }
                        }
                        Err(err) => (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            axum::Json(response::ServiceUserLoginResponse {
                                message: err.to_string(),
                                data: Vec::new(),
                            }),
                        ),
                    }
                }
            }
            Err(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(response::ServiceUserLoginResponse {
                    message: err.to_string(),
                    data: Vec::new(),
                }),
            ),
        }
    }
}

/// Endpoint for service user login
#[utoipa::path(
    post,
    path = super::endpoints::REFRESH_TOKEN,
    request_body(
        content = request::RefreshTokenRequest,
        description = "Data required refresh token",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Service uuser login successful", body = response::RefreshTokenResponse),
        (status = 400, description = "Bad data", body = response::RefreshTokenResponse),
        (status = 500, description = "Something went wrong", body = response::RefreshTokenResponse)
    )
)]
pub async fn refresh_token(
    axum::Extension(pool): axum::Extension<sqlx::PgPool>,
    axum::Json(payload): axum::Json<request::RefreshTokenRequest>,
) -> (
    axum::http::StatusCode,
    axum::Json<response::RefreshTokenResponse>,
) {
    if payload.access_token.is_empty() {
        let reason = String::from("Access token not provided");

        (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(response::RefreshTokenResponse {
                message: reason,
                data: Vec::new(),
            }),
        )
    } else {
        let key = schedtxt_models::envy::environment::get_secret_key().value;
        if token_stuff::verify_token(&key, &payload.access_token) {
            match token_stuff::extract_id_from_token(&key, &payload.access_token) {
                Ok(id) => {
                    let generate_service_token =
                        |id, key| -> Option<schedtxt_models::token::CreateTokenResult> {
                            token_stuff::create_service_refresh_token(key, id).ok()
                        };

                    let mut response = response::RefreshTokenResponse {
                        message: String::new(),
                        data: Vec::new(),
                    };

                    match repo::user::get_with_id(&pool, &id).await {
                        Ok(_user) => match generate_service_token(&id, &key) {
                            Some(cst) => {
                                response.message =
                                    String::from(super::messages::SUCCESSFUL_MESSAGE);
                                let lr = schedtxt_models::token::LoginResult {
                                    user_id: id,
                                    access_token: cst.access_token,
                                    issued_at: cst.issued,
                                    expires_in: cst.expires_in,
                                    token_type: String::from(schedtxt_models::token::TOKEN_TYPE),
                                };
                                response.data.push(lr);
                                (axum::http::StatusCode::OK, axum::Json(response))
                            }
                            None => (
                                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                axum::Json(response::RefreshTokenResponse {
                                    message: String::from("Refresh token not generated"),
                                    data: Vec::new(),
                                }),
                            ),
                        },
                        Err(err) => {
                            println!("Unable to find user, checking service user: {err:?}");
                            match repo::service::get(&pool, &id).await {
                                Ok(_service_user) => match generate_service_token(&id, &key) {
                                    Some(cst) => {
                                        response.message =
                                            String::from(super::messages::SUCCESSFUL_MESSAGE);
                                        let lr = schedtxt_models::token::LoginResult {
                                            user_id: id,
                                            access_token: cst.access_token,
                                            issued_at: cst.issued,
                                            expires_in: cst.expires_in,
                                            token_type: String::from(
                                                schedtxt_models::token::TOKEN_TYPE,
                                            ),
                                        };
                                        response.data.push(lr);
                                        (axum::http::StatusCode::OK, axum::Json(response))
                                    }
                                    None => (
                                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        axum::Json(response::RefreshTokenResponse {
                                            message: String::from("Issued at not returned"),
                                            data: Vec::new(),
                                        }),
                                    ),
                                },
                                Err(err) => (
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                    axum::Json(response::RefreshTokenResponse {
                                        message: err.to_string(),
                                        data: Vec::new(),
                                    }),
                                ),
                            }
                        }
                    }
                }
                Err(err) => (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    axum::Json(response::RefreshTokenResponse {
                        message: err.to_string(),
                        data: Vec::new(),
                    }),
                ),
            }
        } else {
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(response::RefreshTokenResponse {
                    message: String::from("Unable to verify token"),
                    data: Vec::new(),
                }),
            )
        }
    }
}

/// Endpoint for a updating password
#[utoipa::path(
    patch,
    path = super::endpoints::UPDATE_PASSWORD,
    request_body(
        content = request::UpdatePasswordRequest,
        description = "Data required to update password",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "User login successful", body = response::UpdatePasswordResponse),
        (status = 400, description = "Bad data", body = response::UpdatePasswordResponse),
        (status = 500, description = "Something went wrong", body = response::UpdatePasswordResponse)
    )
)]
pub async fn update_password(
    axum::Extension(pool): axum::Extension<sqlx::PgPool>,
    axum::Json(payload): axum::Json<request::UpdatePasswordRequest>,
) -> (
    axum::http::StatusCode,
    axum::Json<response::UpdatePasswordResponse>,
) {
    if !payload.is_valid() {
        (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(response::UpdatePasswordResponse {
                message: String::from("Invalid passwords"),
                data: Vec::new(),
            }),
        )
    } else {
        let verify_password = |current_password, hashed_password| -> Result<bool, std::io::Error> {
            match hashing::verify_password(current_password, hashed_password) {
                Ok(matches) => Ok(matches),
                Err(err) => Err(std::io::Error::other(err.to_string())),
            }
        };

        match repo::user::get_with_id(&pool, &payload.user_id).await {
            Ok(user) => {
                let hashed_password = user.password.clone();
                match verify_password(&payload.current_password, hashed_password) {
                    Ok(matches) => {
                        if matches {
                            let (generate_salt, mut salt) = super::register::generate_the_salt();
                            salt.id = repo::salt::insert(&pool, &salt).await.unwrap();
                            let updated_hashed_password = match hashing::hash_password(
                                &payload.updated_password,
                                &generate_salt,
                            ) {
                                Ok(hashed) => hashed,
                                Err(err) => {
                                    eprintln!("Error: {err:?}");
                                    String::new()
                                }
                            };

                            match repo::user::update_password(
                                &pool,
                                &user,
                                &updated_hashed_password,
                            )
                            .await
                            {
                                Ok(()) => (
                                    axum::http::StatusCode::OK,
                                    axum::Json(response::UpdatePasswordResponse {
                                        message: String::from(super::messages::SUCCESSFUL_MESSAGE),
                                        data: vec![user.id],
                                    }),
                                ),
                                Err(err) => (
                                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                    axum::Json(response::UpdatePasswordResponse {
                                        message: err.to_string(),
                                        data: Vec::new(),
                                    }),
                                ),
                            }
                        } else {
                            (
                                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                axum::Json(response::UpdatePasswordResponse {
                                    message: String::from("Issue updating password"),
                                    data: Vec::new(),
                                }),
                            )
                        }
                    }
                    Err(err) => (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        axum::Json(response::UpdatePasswordResponse {
                            message: err.to_string(),
                            data: Vec::new(),
                        }),
                    ),
                }
            }
            Err(err) => {
                println!("No User found, trying Service User: {err:?}");

                // Try service user
                match repo::service::get(&pool, &payload.user_id).await {
                    Ok(service_user) => {
                        let hashed_password = service_user.passphrase.clone();
                        match verify_password(&payload.current_password, hashed_password) {
                            Ok(matches) => {
                                if matches {
                                    let (generate_salt, mut salt) =
                                        super::register::generate_the_salt();
                                    salt.id = repo::salt::insert(&pool, &salt).await.unwrap();
                                    let updated_hashed_password = match hashing::hash_password(
                                        &payload.updated_password,
                                        &generate_salt,
                                    ) {
                                        Ok(hashed) => hashed,
                                        Err(err) => {
                                            eprintln!("Error: {err:?}");
                                            String::new()
                                        }
                                    };

                                    match repo::service::update_passphrase(
                                        &pool,
                                        &service_user,
                                        &updated_hashed_password,
                                    )
                                    .await
                                    {
                                        Ok(()) => (
                                            axum::http::StatusCode::OK,
                                            axum::Json(response::UpdatePasswordResponse {
                                                message: String::from(
                                                    super::messages::SUCCESSFUL_MESSAGE,
                                                ),
                                                data: vec![service_user.id],
                                            }),
                                        ),
                                        Err(err) => (
                                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                            axum::Json(response::UpdatePasswordResponse {
                                                message: err.to_string(),
                                                data: Vec::new(),
                                            }),
                                        ),
                                    }
                                } else {
                                    (
                                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                        axum::Json(response::UpdatePasswordResponse {
                                            message: String::from("Issue updating password"),
                                            data: Vec::new(),
                                        }),
                                    )
                                }
                            }
                            Err(err) => (
                                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                                axum::Json(response::UpdatePasswordResponse {
                                    message: err.to_string(),
                                    data: Vec::new(),
                                }),
                            ),
                        }
                    }
                    Err(err) => (
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        axum::Json(response::UpdatePasswordResponse {
                            message: err.to_string(),
                            data: Vec::new(),
                        }),
                    ),
                }
            }
        }
    }
}

/// Endpoint for a updating password
#[utoipa::path(
    patch,
    path = super::endpoints::UPDATE_USER_NAME,
    request_body(
        content = request::UserUpdateNameRequest,
        description = "Data required to update name of a user",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Names were updated", body = response::UserUpdateNameResponse),
        (status = 304, description = "Nothing to change", body = response::UserUpdateNameResponse),
        (status = 400, description = "Bad data", body = response::UserUpdateNameResponse),
        (status = 500, description = "Something went wrong", body = response::UserUpdateNameResponse)
    )
)]
pub async fn update_name_of_user(
    axum::Extension(pool): axum::Extension<sqlx::PgPool>,
    axum::Json(payload): axum::Json<request::UserUpdateNameRequest>,
) -> (
    axum::http::StatusCode,
    axum::Json<response::UserUpdateNameResponse>,
) {
    let (valid_request, reason) = payload.is_valid();
    if !valid_request {
        (
            axum::http::StatusCode::BAD_REQUEST,
            axum::Json(response::UserUpdateNameResponse {
                message: reason.unwrap_or_default(),
                data: Vec::new(),
            }),
        )
    } else {
        match repo::user::get_with_id(&pool, &payload.user_id).await {
            Ok(user) => {
                if user.firstname == payload.firstname || user.lastname == payload.lastname {
                    (
                        axum::http::StatusCode::NOT_MODIFIED,
                        axum::Json(response::UserUpdateNameResponse {
                            message: String::from("No change"),
                            data: Vec::new(),
                        }),
                    )
                } else {
                    match repo::user::update_name(
                        &pool,
                        &user.id,
                        &payload.firstname,
                        &payload.lastname,
                    )
                    .await
                    {
                        Ok(_) => (
                            axum::http::StatusCode::OK,
                            axum::Json(response::UserUpdateNameResponse {
                                message: String::from(super::messages::SUCCESSFUL_MESSAGE),
                                data: vec![schedtxt_models::user::User {
                                    id: user.id,
                                    phone_number: user.phone_number,
                                    firstname: payload.firstname,
                                    lastname: payload.lastname,
                                    username: user.username,
                                    created: user.created,
                                    last_login: user.last_login,
                                    ..Default::default()
                                }],
                            }),
                        ),
                        Err(err) => (
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            axum::Json(response::UserUpdateNameResponse {
                                message: err.to_string(),
                                data: Vec::new(),
                            }),
                        ),
                    }
                }
            }
            Err(err) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(response::UserUpdateNameResponse {
                    message: err.to_string(),
                    data: Vec::new(),
                }),
            ),
        }
    }
}

/// Endpoint to get User Profile
#[utoipa::path(
    delete,
    path = super::endpoints::GET_USER_PROFILE,
    params(("id" = uuid::Uuid, Path, description = "User Id")),
    responses(
        (status = 200, description = "Deleted", body = response::GetUserProfileResponse),
        (status = 400, description = "Bad request", body = response::GetUserProfileResponse),
        (status = 500, description = "Error", body = response::GetUserProfileResponse)
    )
)]
pub async fn get_user_profile(
    axum::Extension(pool): axum::Extension<sqlx::PgPool>,
    axum::extract::Path(id): axum::extract::Path<uuid::Uuid>,
) -> (
    axum::http::StatusCode,
    axum::Json<response::GetUserProfileResponse>,
) {
    let mut response = response::GetUserProfileResponse::default();
    if id.is_nil() {
        response.message = String::from("Invalid");
        return (axum::http::StatusCode::BAD_REQUEST, axum::Json(response));
    }

    match repo::user::get_with_id(&pool, &id).await {
        Ok(user) => {
            let user_profile = schedtxt_models::user::UserProfile {
                user_id: user.id,
                phone_number: user.phone_number,
                firstname: user.firstname,
                lastname: user.lastname,
                last_login: user.last_login,
                created: user.created,
                username: user.username,
            };
            response.message = String::from(super::messages::SUCCESSFUL_MESSAGE);
            response.data.push(user_profile);

            (axum::http::StatusCode::OK, axum::Json(response))
        }
        Err(err) => {
            eprintln!("Error: {err:?}");
            response.message = String::from("Bad request");
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                axum::Json(response),
            )
        }
    }
}
