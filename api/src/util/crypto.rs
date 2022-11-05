use std::str::FromStr;

use actix_web::cookie::Cookie;

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{prelude::Uuid, DatabaseConnection};
use serde::{Deserialize, Serialize};

use crate::crud;

use super::error::CrudError;
const TOKEN_MAX_AGE: i64 = 8 * 60 * 60; //8Hours
#[derive(Serialize, Deserialize, Clone, Debug)]
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
pub async fn is_admin(cookie: &Option<Cookie<'_>>, db: &DatabaseConnection) -> bool {
    if let Some(cookie) = cookie {
        let claim = self::authorize(cookie.value());
        if let Ok(claim) = claim {
            if let Ok(sub) = Uuid::from_str(&claim.sub) {
                return crud::user::is_admin_by_user_id(sub, db)
                    .await
                    .unwrap_or(false);
            }
        }
    }
    false
}
pub fn is_self(cookie: Option<Cookie>, id: &str) -> bool {
    if let Some(cookie) = cookie {
        let claim = self::authorize(cookie.value());
        if let Ok(claim) = claim {
            return &claim.sub == id;
        }
    }
    false
}
