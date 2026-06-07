pub mod common;
pub mod register;

pub mod endpoints {
    pub const ROOT: &str = "/";
    pub const REGISTER: &str = "/api/v1/register";
    pub const DBTEST: &str = "/api/v1/test/db";
}
