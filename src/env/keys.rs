/// Environment key for Database management
pub const DB_URL: &str = "DATABASE_URL";

/// Environment key for secret main key
/// Used for the textsender app
pub const SECRET_MAIN_KEY: &str = "SECRET_MAIN_KEY";

/// Environment key for service logins
pub const SERVICE_PASSPHRASE: &str = "SERVICE_PASSPHRASE";

/// Environment key for secret key
/// Generic use of secret key that could be found in various apps
pub const SECRET_KEY: &str = "SECRET_KEY";

/// Environment key for root directory for the textsender app
pub const ROOT_DIRECTORY: &str = "ROOT_DIRECTORY";

/// Environment key for textsender api url
pub const TEXTSENDER_BASE_API_URL: &str = "TEXTSENDER_BASE_API_URL";

/// Environment key for textsender auth api url
pub const TEXTSENDER_AUTH_BASE_API_URL: &str = "TEXTSENDER_AUTH_BASE_API_URL";

/// Environment key for App status
pub const APP_ENV: &str = "APP_ENV";
/// Environment key for backend port. Used for both auth and core functionality
pub const BACKEND_PORT: &str = "BACKEND_PORT";
/// Environment key for frontend url
pub const FRONTEND_URL: &str = "FRONTEND_URL";
/// Environment key for application logging
pub const RUST_LOG: &str = "RUST_LOG";
/// Environment key for allowed origins for CORS support
pub const ALLOWED_ORIGINS: &str = "ALLOWED_ORIGINS";

pub mod error {
    use const_format::concatcp;

    pub const GENERAL_ERROR: &str = "must not be set in enviornment file";
    pub const DB_URL: &str = concatcp!(super::DB_URL, " ", GENERAL_ERROR);
    pub const SECRET_MAIN_KEY: &str = concatcp!(super::SECRET_MAIN_KEY, " ", GENERAL_ERROR);
    pub const SERVICE_LOGIN: &str = concatcp!(super::SERVICE_PASSPHRASE, " ", GENERAL_ERROR);
    pub const SECRET_KEY: &str = concatcp!(super::SECRET_KEY, " ", GENERAL_ERROR);
    pub const ROOT_DIRECTORY: &str = concatcp!(super::ROOT_DIRECTORY, " ", GENERAL_ERROR);
    pub const TEXTSENDER_BASE_API_URL: &str = concatcp!(super::TEXTSENDER_BASE_API_URL, " ", GENERAL_ERROR);
    pub const TEXTSENDER_AUTH_BASE_API_URL: &str =
        concatcp!(super::TEXTSENDER_AUTH_BASE_API_URL, " ", GENERAL_ERROR);
    pub const APP_ENV: &str = concatcp!(super::APP_ENV, " ", GENERAL_ERROR);
    pub const BACKEND_PORT: &str = concatcp!(super::BACKEND_PORT, " ", GENERAL_ERROR);
    pub const FRONTEND_URL: &str = concatcp!(super::FRONTEND_URL, " ", GENERAL_ERROR);
    pub const RUST_LOG: &str = concatcp!(super::RUST_LOG, " ", GENERAL_ERROR);
    pub const ALLOWED_ORIGINS: &str = concatcp!(super::ALLOWED_ORIGINS, " ", GENERAL_ERROR);
}
