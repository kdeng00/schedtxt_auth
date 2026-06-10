pub mod common;
pub mod login;
pub mod register;

pub mod endpoints {
    pub const ROOT: &str = "/";
    pub const REGISTER: &str = "/api/v1/register";
    /// Endpoint for a user to login
    pub const LOGIN: &str = "/api/v1/login";
    pub const DBTEST: &str = "/api/v1/test/db";
}
