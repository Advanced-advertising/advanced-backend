use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::AppError;
use crate::middleware::token::authorize;
use crate::middleware::token::Role::Client;
use crate::models::user::User;
use crate::schema::users::dsl::{img_url, user_id, user_name, users};
use actix::{Handler, Message};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::Hasher;
use diesel::prelude::*;
use serde::Deserialize;
use slog::{o, Logger};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<User, AppError>")]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub img_url: String,
    pub phone_number: String,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<String, AppError>")]
pub struct AuthorizeUser {
    pub basic_auth: BasicAuth,
}

#[derive(Message, Deserialize)]
#[rtype(result = "Result<String, AppError>")]
pub struct ChangeImg {
    pub user_id: Uuid,
    pub img_url: String,
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
            img_url: msg.img_url,
            email: msg.email,
            password: password_hash,
            phone_number: msg.phone_number,
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
        let username = msg.basic_auth.user_id().to_string();

        let mut conn = self.0.get()?;

        let user = users
            .filter(user_name.eq(username))
            .get_result::<User>(&mut conn)?;

        authorize(user.user_id, user.password, vec![Client], msg.basic_auth)
    }
}

impl Handler<ChangeImg> for DbActor {
    type Result = Result<String, AppError>;

    fn handle(&mut self, msg: ChangeImg, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;
        diesel::update(users)
            .filter(user_id.eq(msg.user_id))
            .set(img_url.eq(msg.img_url.clone()))
            .execute(&mut conn)?;

        Ok(msg.img_url)
    }
}
