/// Take the Environment variable and delimitize it. If the value has a delimiter,
/// extract it into some strings
pub fn delimitize(var: &crate::env::EnvVar) -> Result<Vec<String>, std::io::Error> {
    if var.has_delimiter {
        Ok(var
            .value
            .split(var.delimiter)
            .map(|c| c.parse::<String>().unwrap())
            .collect())
    } else {
        Err(std::io::Error::other(
            "Environment variable does not have a delimiter",
        ))
    }
}
