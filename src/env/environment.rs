pub async fn get_db_url() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::DB_URL;
    let value = std::env::var(key).expect(key);

    crate::env::init_envvar(key, &value)
}

pub async fn get_secret_main_key() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::SECRET_MAIN_KEY;
    let value = std::env::var(key).expect(key);

    crate::env::init_envvar(key, &value)
}

pub async fn get_service_passphrase() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::SERVICE_PASSPHRASE;
    let value = std::env::var(key).expect(key);

    crate::env::init_envvar(key, &value)
}

pub async fn get_secret_key() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::SECRET_KEY;
    let value = std::env::var(key).expect(key);

    crate::env::init_envvar(key, &value)
}

pub async fn get_root_directory() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::ROOT_DIRECTORY;
    let value = std::env::var(key).expect(key);

    crate::env::init_envvar(key, &value)
}

pub async fn get_app_base_api_url() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::TEXTSENDER_BASE_API_URL;
    let value = std::env::var(key).expect(key);

    crate::env::init_envvar(key, &value)
}

pub async fn get_app_auth_base_api_url() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::TEXTSENDER_AUTH_BASE_API_URL;
    let value = std::env::var(key).expect(key);

    crate::env::init_envvar(key, &value)
}

pub async fn get_app_env() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::APP_ENV;
    let value = std::env::var(key).expect(key);

    crate::env::init_envvar(key, &value)
}

pub async fn get_backend_port() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::BACKEND_PORT;
    let value = std::env::var(key).expect(key);
    crate::env::init_envvar(key, &value)
}

pub async fn get_frontend_url() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::FRONTEND_URL;
    let value = std::env::var(key).expect(key);

    crate::env::init_envvar(key, &value)
}

pub async fn get_rust_log() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::RUST_LOG;
    let value = std::env::var(key).expect(key);

    crate::env::init_envvar(key, &value)
}

pub async fn get_allowed_origins() -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let key = crate::env::keys::ALLOWED_ORIGINS;
    let value = std::env::var(key).expect(key);

    let mut envvar = crate::env::init_envvar(key, &value);
    crate::env::init_delimiter(&mut envvar, ',');

    envvar
}

/// Get environment not specified in the code
pub async fn get_env(environment: &str) -> crate::env::EnvVar {
    dotenvy::dotenv().ok();
    let my_error = format!("{environment} {}", crate::env::keys::error::GENERAL_ERROR);
    let value = std::env::var(environment).expect(&my_error);

    crate::env::init_envvar(environment, &value)
}
