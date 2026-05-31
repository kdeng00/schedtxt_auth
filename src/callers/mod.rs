pub mod common;
pub mod register;

pub mod endpoints {
    pub const ROOT: &str = "/";
    pub const REGISTER: &str = "/api/v2/register";
    pub const DBTEST: &str = "/api/v2/test/db";
}
