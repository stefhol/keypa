use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use paperclip::actix::Apiv2Schema;

use serde::{Deserialize, Serialize};

use super::error::CrudError;
const TOKEN_MAX_AGE: i64 = 8 * 60 * 60; //8Hours
#[derive(Serialize, Deserialize, Clone, Debug, Apiv2Schema)]
#[openapi(empty)]
pub struct Claims {
    pub sub: String,
    pub is_admin: bool,
    pub is_worker: bool,
    pub is_leader: bool,
    exp: usize,
}

pub fn create_jwt(
    uid: &str,
    is_admin: bool,
    is_worker: bool,
    is_leader: bool,
) -> Result<String, CrudError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::seconds(TOKEN_MAX_AGE))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: uid.to_owned(),
        exp: expiration as usize,
        //only for ui
        is_admin,
        is_worker,
        is_leader,
    };
    let header = Header::new(Algorithm::HS512);
    Ok(encode(
        &header,
        &claims,
        &EncodingKey::from_secret(dotenv::var("JWT_SECRET")?.as_bytes()),
    )?)
}

pub fn authorize(token: &str) -> anyhow::Result<Claims> {
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(dotenv::var("JWT_SECRET")?.as_bytes()),
        &Validation::new(Algorithm::HS512),
    )?;
    Ok(decoded.claims)
}
