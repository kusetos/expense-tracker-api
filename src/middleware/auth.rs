use actix_web::{dev::Payload, error::ErrorUnauthorized, Error, FromRequest, HttpRequest};
use uuid::Uuid;
use crate::utils::jwt::verify_jwt;
use std::future::{ready, Ready};
pub struct AuthUser {
    pub id: Uuid,
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Ready<Result<Self, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let jwt_secret = "boi";//std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev_secret".to_string());

        let auth_header = match req.headers().get("Authorization") {
            Some(h) => h,
            None => return ready(Err(ErrorUnauthorized("Missing Authorization header"))),
        };

        let auth_str = match auth_header.to_str() {
            Ok(s) => s,
            Err(_) => return ready(Err(ErrorUnauthorized("Invalid header format"))),
        };

        if !auth_str.starts_with("Bearer ") {
            return ready(Err(ErrorUnauthorized("Token must start with Bearer")));
        }

        let token = &auth_str[7..];

        let claims = match verify_jwt(token, &jwt_secret) {
            Ok(c) => c,
            Err(_) => return ready(Err(ErrorUnauthorized("Invalid or expired token"))),
        };

        let user_id = match Uuid::parse_str(&claims.sub) {
            Ok(id) => id,
            Err(_) => return ready(Err(ErrorUnauthorized("Invalid user id in token"))),
        };

        ready(Ok(AuthUser { id: user_id }))
    }
}
