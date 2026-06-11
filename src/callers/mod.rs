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
}
