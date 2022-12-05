use actix::{Handler, Message};
use argonautica::{Hasher, Verifier};
use diesel::prelude::*;
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::SignWithKey;
use serde::Deserialize;
use sha2::Sha256;
use uuid::Uuid;
use crate::actors::db::DbActor;
use crate::middleware::token::TokenClaims;
use crate::models::user::User;
use crate::schema::users::dsl::{users, user_id, user_name};

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Message)]
#[derive(Deserialize)]
#[rtype(result="QueryResult<String>")]
pub struct AuthorizeUser {
    pub name: String,
    pub password: String,
}

#[derive(Message)]
#[rtype(result="QueryResult<User>")]
pub struct UpdateUser {
    pub uuid: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;

    fn handle(&mut self, msg: CreateUser, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("Unable to get a connection");

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

        diesel::insert_into(users)
            .values(new_user)
            .get_result::<User>(&mut conn)
    }
}

impl Handler<AuthorizeUser> for DbActor {
    type Result = QueryResult<String>;

    fn handle(&mut self, msg: AuthorizeUser, _: &mut Self::Context) -> Self::Result {
        let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
            std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set!")
                .as_bytes(),
        )
            .unwrap();
        let username = msg.name;
        let password = msg.password;

        let mut conn = self.0.get().expect("Unable to get a connection");
        match users.filter(user_name.eq(username)).get_result::<User>(&mut conn) {
            Ok(user) => {
                let hash_secret =
                    std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
                let mut verifier = Verifier::default();
                let is_valid = verifier
                    .with_hash(user.password)
                    .with_password(password)
                    .with_secret_key(hash_secret)
                    .verify()
                    .unwrap();

                if is_valid {
                    let claims = TokenClaims { id: user.user_id};
                    let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                    Ok(token_str)
                } else {
                    Err(diesel::result::Error::NotFound)
                }
            },
            Err(_) => Err(diesel::result::Error::NotFound)
        }
    }
}