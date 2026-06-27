use josekit::{
    self,
    jws::alg::hmac::HmacJwsAlgorithm::Hs256,
    jwt::{self},
};

use time;

pub const KEY_ENV: &str = "SECRET_KEY";
pub const MESSAGE: &str = "Something random";
pub const ISSUER: &str = "textsender_auth";
pub const AUDIENCE: &str = "textsender";

pub fn get_expiration(issued: &time::OffsetDateTime) -> Result<time::OffsetDateTime, time::Error> {
    let duration_expire = time::Duration::hours(4);
    Ok(*issued + duration_expire)
}

pub fn create_token(
    provided_key: &String,
    id: &uuid::Uuid,
) -> Result<textsender_models::token::CreateTokenResult, josekit::JoseError> {
    let resource = textsender_models::token::TokenResource {
        message: String::from(MESSAGE),
        issuer: String::from(ISSUER),
        audiences: vec![String::from(AUDIENCE)],
        user_id: *id,
    };
    textsender_models::token::create_token(provided_key, &resource, time::Duration::hours(4))
}

pub fn create_service_token(
    provided: &String,
    id: &uuid::Uuid,
) -> Result<textsender_models::token::CreateTokenResult, josekit::JoseError> {
    let resource = textsender_models::token::TokenResource {
        message: String::from(SERVICE_SUBJECT),
        issuer: String::from(ISSUER),
        audiences: vec![String::from(AUDIENCE)],
        user_id: *id,
    };
    textsender_models::token::create_token(provided, &resource, time::Duration::hours(1))
}

pub fn create_service_refresh_token(
    key: &String,
    id: &uuid::Uuid,
) -> Result<textsender_models::token::CreateTokenResult, josekit::JoseError> {
    let resource = textsender_models::token::TokenResource {
        message: String::from(SERVICE_SUBJECT),
        issuer: String::from(ISSUER),
        audiences: vec![String::from(AUDIENCE)],
        user_id: *id,
    };
    textsender_models::token::create_token(key, &resource, time::Duration::hours(4))
}

pub fn verify_token(key: &String, token: &String) -> bool {
    match get_payload(key, token) {
        Ok((payload, _header)) => match payload.subject() {
            Some(_sub) => true,
            None => false,
        },
        Err(_err) => false,
    }
}

pub fn extract_id_from_token(key: &String, token: &String) -> Result<uuid::Uuid, std::io::Error> {
    match get_payload(key, token) {
        Ok((payload, _header)) => match payload.claim("user_id") {
            Some(id) => match uuid::Uuid::parse_str(id.as_str().unwrap()) {
                Ok(extracted) => Ok(extracted),
                Err(err) => Err(std::io::Error::other(err.to_string())),
            },
            None => Err(std::io::Error::other("No claim found")),
        },
        Err(err) => Err(std::io::Error::other(err.to_string())),
    }
}

pub const APP_TOKEN_TYPE: &str = "Textsender_App";
pub const APP_SUBJECT: &str = "Something random";
pub const SERVICE_TOKEN_TYPE: &str = "Textsender_Service";
pub const SERVICE_SUBJECT: &str = "Service random";

pub fn get_token_type(key: &String, token: &String) -> Result<String, std::io::Error> {
    match get_payload(key, token) {
        Ok((payload, _header)) => match payload.subject() {
            Some(subject) => {
                if subject == APP_SUBJECT {
                    Ok(String::from(APP_TOKEN_TYPE))
                } else if subject == SERVICE_SUBJECT {
                    Ok(String::from(SERVICE_TOKEN_TYPE))
                } else {
                    Err(std::io::Error::other(String::from("Invalid subject")))
                }
            }
            None => Err(std::io::Error::other(String::from("Invalid payload"))),
        },
        Err(err) => Err(std::io::Error::other(err.to_string())),
    }
}

pub fn is_token_type_valid(token_type: &String) -> bool {
    token_type == SERVICE_TOKEN_TYPE
}

fn get_payload(
    key: &String,
    token: &String,
) -> Result<(josekit::jwt::JwtPayload, josekit::jws::JwsHeader), josekit::JoseError> {
    let ver = Hs256.verifier_from_bytes(key.as_bytes()).unwrap();
    jwt::decode_with_verifier(token, &ver)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        let special_key = rt
            .block_on(textsender_models::envy::environment::get_secret_key())
            .value;
        let id = uuid::Uuid::new_v4();
        match create_token(&special_key, &id) {
            Ok(cst) => {
                let result = verify_token(&special_key, &cst.access_token);
                assert!(result, "Token not verified");
            }
            Err(err) => {
                assert!(false, "Error: {:?}", err.to_string());
            }
        };
    }
}
