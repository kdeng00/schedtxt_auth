const ADDRESS: &str = "0.0.0.0";
const PORT: &str = "9080";

pub fn get_full() -> String {
    format!("{ADDRESS}:{PORT}")
}
