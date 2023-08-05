use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::{AppError, AppErrorType};
use crate::middleware::token::TokenClaims;
use crate::models::user::User;
use crate::schema::users::dsl::{user_name, users};
use actix::{Handler, Message};
use argonautica::{Hasher, Verifier};
use diesel::prelude::*;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::SignWithKey;
use serde::Deserialize;
use sha2::Sha256;
use slog::{o, Logger};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<User, AppError>")]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub logger: Logger,
}

#[derive(Message, Deserialize)]
#[rtype(result = "Result<String, AppError>")]
pub struct AuthorizeUser {
    pub name: String,
    pub password: String,
}

impl Handler<CreateUser> for DbActor {
    type Result = Result<User, AppError>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
        let mut hasher = Hasher::default();
        let password_hash = hasher
            .with_password(msg.password)
            .with_secret_key(hash_secret)
            .hash()
            .unwrap();

        let new_user = User {
            user_id: Uuid::new_v4(),
            user_name: msg.name,
            email: msg.email,
            password: password_hash,
        };

        let sub_log = msg.logger.new(o!("handle" => "create_user"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;
        let user = diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(&mut conn)?;

        Ok(user)
    }
}

impl Handler<AuthorizeUser> for DbActor {
    type Result = Result<String, AppError>;

    fn handle(&mut self, msg: AuthorizeUser, _: &mut Self::Context) -> Self::Result {
        let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
            std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set!")
                .as_bytes(),
        )
        .unwrap();
        let username = msg.name;
        let password = msg.password;

        let mut conn = self.0.get()?;

        let user = users
            .filter(user_name.eq(username))
            .get_result::<User>(&mut conn)?;

        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(user.password)
            .with_password(password)
            .with_secret_key(hash_secret)
            .verify()
            .unwrap();

        if is_valid {
            let claims = TokenClaims { id: user.user_id };
            let token_str = claims.sign_with_key(&jwt_secret).unwrap();
            Ok(token_str)
        } else {
            Err(AppError {
                message: Some("Cannot authorise user".to_string()),
                cause: None,
                error_type: AppErrorType::SomethingWentWrong,
            })
        }
    }
}
