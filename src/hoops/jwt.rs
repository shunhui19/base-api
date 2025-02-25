use jsonwebtoken::EncodingKey;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

use crate::config;

#[derive(Serialize, Deserialize, Debug)]
pub struct JwtClaims {
    uid: String,
    exp: i64,
}

pub fn get_token(uid: impl Into<String>) -> Result<(String, i64)> {
    let exp = OffsetDateTime::now_utc() + Duration::seconds(config::get().jwt.expiry);
    let claim = JwtClaims {
        uid: uid.into(),
        exp: exp.unix_timestamp(),
    };

    let token: String = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(config::get().jwt.secret.as_bytes()),
    )?;
    Ok((token, exp.unix_timestamp()))
}

