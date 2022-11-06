use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    body::MessageBody,
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};

use futures::FutureExt;
use futures_util::future::LocalBoxFuture;
use paperclip::actix::Apiv2Schema;

use crate::util::crypto;

use super::crypto::Claims;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth;
impl Default for Auth {
    fn default() -> Self {
        Self::new()
    }
}
impl Auth {
    fn new() -> Self {
        Auth
    }
}
#[derive(Clone, Debug, Apiv2Schema)]
#[openapi(empty)]
pub enum AuthenticationResult {
    Authenticated(Claims),
    NotAuthenticated,
}
impl AuthenticationResult {
    pub fn to_sercurity_level(&self) -> SecurityLevel {
        return if let AuthenticationResult::Authenticated(val) = self {
            return if val.is_admin {
                SecurityLevel::Admin
            } else if val.is_leader {
                SecurityLevel::Leader
            } else if val.is_worker {
                SecurityLevel::Worker
            } else {
                SecurityLevel::User
            };
        } else {
            SecurityLevel::External
        };
    }
    pub fn try_get_user_id(&self) -> Option<String> {
        match self {
            AuthenticationResult::Authenticated(val) => Some(val.sub.to_string()),
            _ => None,
        }
    }
    ///Returns true if user_id in auth token and provided user_id is the same
    pub fn compare_user_id(&self, user_id: &str) -> bool {
        match self.try_get_user_id() {
            Some(id) => &id == user_id,
            None => false,
        }
    }
}
#[derive(Clone, PartialOrd, PartialEq)]
pub enum SecurityLevel {
    External = 0,
    User = 1,
    Worker = 2,
    Leader = 3,
    Admin = 4,
}

pub type AuthenticationInfo = Rc<AuthenticationResult>;
// Middleware factory is `Transform` trait
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    B: MessageBody,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthenticationMiddleware { service }))
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Requested: {}", req.path());
        if let Some(cookie) = req.cookie("bearer") {
            if let Ok(val) = crypto::authorize(cookie.value()) {
                req.extensions_mut().insert::<AuthenticationInfo>(Rc::new(
                    AuthenticationResult::Authenticated(val),
                ));
            } else {
                req.extensions_mut()
                    .insert::<AuthenticationInfo>(Rc::new(AuthenticationResult::NotAuthenticated));
            }
            //TODO: Change to real value
        } else {
            req.extensions_mut()
                .insert::<AuthenticationInfo>(Rc::new(AuthenticationResult::NotAuthenticated));
        }
        let fut = self.service.call(req);
        async move {
            let res = fut.await?;
            Ok(res)
        }
        .boxed_local()
    }

    forward_ready!(service);
}
pub mod extractor;
// fn add_token<B>(res: &mut HttpResponse<B>) -> anyhow::Result<()> {
//     res.add_cookie(
//         &Cookie::build("token", "value")
//             .max_age(Duration::hours(8))
//             .http_only(true)
//             .finish(),
//     )?;
//     Ok(())
// }
