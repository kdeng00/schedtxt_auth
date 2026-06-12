pub mod common;
pub mod login;
pub mod messages;
pub mod register;

pub mod endpoints {
    pub const ROOT: &str = "/";
    pub const REGISTER: &str = "/api/v1/register";
    /// Endpoint for a user to login
    pub const LOGIN: &str = "/api/v1/login";
    pub const DBTEST: &str = "/api/v1/test/db";
    /// Endpoint constant for service user registration
    pub const REGISTER_SERVICE_USER: &str = "/api/v1/service/register";
    /// Endpoint constant for service login user
    pub const LOGIN_SERVICE_USER: &str = "/api/v1/service/login";
    /// Endpoint constant for refresh token
    pub const REFRESH_TOKEN: &str = "/api/v1/token/refresh";
    /// Endpoint constant for updating password
    pub const UPDATE_PASSWORD: &str = "/api/v1/user/password/update";
    /// Endpoint constant for updating user's name
    pub const UPDATE_USER_NAME: &str = "/api/v1/user/name/update";
}
