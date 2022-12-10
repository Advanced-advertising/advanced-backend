use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::{AppError, AppErrorType};
use crate::middleware::token::TokenClaims;
use crate::models::business::Business;
use crate::schema::businesses::dsl::{business_id, business_name, businesses, img_url};
use actix::{ActorContext, Handler, Message};
use argonautica::{Hasher, Verifier};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Error, Pool, PoolError, PooledConnection};
use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::SignWithKey;
use serde::Deserialize;
use sha2::Sha256;
use slog::{crit, error, o, Logger};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<Business, AppError>")]
pub struct CreateBusiness {
    pub name: String,
    pub password: String,
    pub email: String,
    pub working_time: String,
    pub img_url: String,
    pub logger: Logger,
}

#[derive(Message, Deserialize)]
#[rtype(result = "Result<String, AppError>")]
pub struct AuthorizeBusiness {
    pub name: String,
    pub password: String,
}

#[derive(Message, Deserialize)]
#[rtype(result = "Result<String, AppError>")]
pub struct ChangeImg {
    pub business_id: Uuid,
    pub img_url: String,
}

impl Handler<CreateBusiness> for DbActor {
    type Result = Result<Business, AppError>;

    fn handle(&mut self, msg: CreateBusiness, _: &mut Self::Context) -> Self::Result {
        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
        let mut hasher = Hasher::default();
        let password_hash = hasher
            .with_password(msg.password)
            .with_secret_key(hash_secret)
            .hash()
            .unwrap();

        let new_business = Business {
            business_id: Uuid::new_v4(),
            business_name: msg.name,
            email: msg.email,
            password: password_hash,
            working_time: msg.working_time,
            img_url: msg.img_url,
        };

        let sub_log = msg.logger.new(o!("handle" => "create_businesses"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;
        let business = diesel::insert_into(businesses)
            .values(new_business)
            .get_result::<Business>(&mut conn)?;

        Ok(business)
    }
}

impl Handler<AuthorizeBusiness> for DbActor {
    type Result = Result<String, AppError>;

    fn handle(&mut self, msg: AuthorizeBusiness, _: &mut Self::Context) -> Self::Result {
        let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
            std::env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set!")
                .as_bytes(),
        )
        .unwrap();

        let business_name_msg = msg.name;
        let password = msg.password;

        let mut conn = self.0.get()?;
        let business = businesses
            .filter(business_name.eq(business_name_msg))
            .get_result::<Business>(&mut conn)?;

        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
        let mut verifier = Verifier::default();
        let is_valid = verifier
            .with_hash(business.password)
            .with_password(password)
            .with_secret_key(hash_secret)
            .verify()
            .unwrap();

        if is_valid {
            let claims = TokenClaims {
                id: business.business_id,
            };
            let token_str = claims.sign_with_key(&jwt_secret).unwrap();
            Ok(token_str)
        } else {
            Err(AppError {
                message: Some("Cannot authorise business".to_string()),
                cause: None,
                error_type: AppErrorType::SomethingWentWrong,
            })
        }
    }
}

impl Handler<ChangeImg> for DbActor {
    type Result = Result<String, AppError>;

    fn handle(&mut self, msg: ChangeImg, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;
        diesel::update(businesses)
            .filter(business_id.eq(msg.business_id))
            .set(img_url.eq(msg.img_url.clone()))
            .execute(&mut conn)?;

        Ok(msg.img_url)
    }
}
