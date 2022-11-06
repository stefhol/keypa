use actix_web::{Error, FromRequest, HttpMessage};
use anyhow::anyhow;
use std::future::{ready, Ready};

use super::AuthenticationInfo;
#[derive(Clone, Debug)]
pub struct Authenticated(AuthenticationInfo);

impl FromRequest for Authenticated {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let value = req.extensions().get::<AuthenticationInfo>().cloned();
        let result = match value {
            Some(v) => Ok(Authenticated(v)),
            None => Err(actix_web::error::ErrorUnauthorized(anyhow!("Not allowed"))),
        };
        ready(result)
    }
}
impl std::ops::Deref for Authenticated {
    type Target = AuthenticationInfo;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
