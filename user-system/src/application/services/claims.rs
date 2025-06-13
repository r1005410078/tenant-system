use anyhow::anyhow;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
    pub username: String,
    pub rule: String,
    pub exp: i64,
    pub iat: i64,
}

const KEY: &[u8] = b"2778205";

impl Claims {
    pub fn new(user_id: String, username: String, rule: String) -> Self {
        let iat = OffsetDateTime::now_utc();
        Claims {
            iat: iat.unix_timestamp(),
            user_id,
            username,
            rule,
            exp: (iat + Duration::days(1)).unix_timestamp(), //1723962788,
        }
    }

    pub fn get_token(&self) -> String {
        match encode(&Header::default(), &self, &EncodingKey::from_secret(KEY)) {
            Ok(t) => t,
            Err(err) => panic!("Error: {err:?}"), // in practice you would return the error
        }
    }

    // 获取
    pub fn validate(token: &str) -> anyhow::Result<Claims> {
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(KEY),
            &Validation::default(),
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => Err(anyhow!("Token is invalid")), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => Err(anyhow!("Issuer is invalid")), // Example on how to handle a specific error
                _ => Err(anyhow!("Some other errors")),
            },
        }
    }
}
