use crate::errors::{AppError, AppErrorType};
use actix_web::web::Data;
use actix_web::{dev::ServiceRequest, error::Error, HttpMessage};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::extractors::{
    bearer::{self, BearerAuth},
    AuthenticationError,
};
use argonautica::Verifier;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use uuid::Uuid;

/*
pub struct AuthorizationState {
    pub required_roles: Vec<Role>,
    pub logger: Logger,
}
 */

#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    pub(crate) id: Uuid,
    pub(crate) roles: Vec<Role>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub enum Role {
    Admin,
    Client,
    Business,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(claims) => {
            req.extensions_mut().insert(claims.clone());

            if check_access(&claims.roles, &req) {
                Ok(req)
            } else {
                let config = req
                    .app_data::<bearer::Config>()
                    .cloned()
                    .unwrap_or_default()
                    .scope("");

                Err((AuthenticationError::from(config).into(), req))
            }
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

fn check_access(roles: &[Role], req: &ServiceRequest) -> bool {
    if let Some(required_roles) = req.app_data::<Data<Vec<Role>>>() {
        roles
            .iter()
            .any(|user_role| required_roles.contains(user_role))
    } else {
        true
    }
}

pub fn authorize(
    id: Uuid,
    password: String,
    roles: Vec<Role>,
    basic_auth: BasicAuth,
) -> Result<String, AppError> {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    )
    .unwrap();

    let verifiable_password = get_password(basic_auth.clone())?;

    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut verifier = Verifier::default();
    let is_valid = verifier
        .with_hash(password)
        .with_password(verifiable_password)
        .with_secret_key(hash_secret)
        .verify()?;

    if is_valid {
        let claims = TokenClaims { id, roles };
        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
        Ok(token_str)
    } else {
        Err(AppError {
            message: Some("Cannot authorise".to_string()),
            cause: None,
            error_type: AppErrorType::SomethingWentWrong,
        })
    }
}

fn get_password(basic_auth: BasicAuth) -> Result<String, AppError> {
    match basic_auth.password() {
        Some(pass) => Ok(pass.to_string()),
        None => {
            return Err(AppError {
                message: Some("Must provide username and password".to_string()),
                cause: None,
                error_type: AppErrorType::PasswordOrLoginError,
            })
        }
    }
}
