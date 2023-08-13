use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::AppError;
use crate::middleware::token::authorize;
use crate::middleware::token::Role::Admin as AdminRole;
use crate::models::admin::Admin;
use crate::schema::admin::dsl::{admin as admin_table, admin_name as admin_name_column};
use actix::{Handler, Message};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::Hasher;
use diesel::prelude::*;
use serde::Deserialize;
use slog::{o, Logger};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<Admin, AppError>")]
pub struct CreateAdmin {
    pub name: String,
    pub password: String,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<String, AppError>")]
pub struct AuthorizeAdmin {
    pub basic_auth: BasicAuth,
}

#[derive(Message, Deserialize)]
#[rtype(result = "Result<String, AppError>")]
pub struct ChangeImg {
    pub user_id: Uuid,
    pub img_url: String,
}

impl Handler<CreateAdmin> for DbActor {
    type Result = Result<Admin, AppError>;

    fn handle(&mut self, msg: CreateAdmin, _: &mut Self::Context) -> Self::Result {
        let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
        let mut hasher = Hasher::default();
        let password_hash = hasher
            .with_password(msg.password)
            .with_secret_key(hash_secret)
            .hash()
            .unwrap();

        let new_admin = Admin {
            admin_id: Uuid::new_v4(),
            admin_name: msg.name,
            password: password_hash,
        };

        let sub_log = msg.logger.new(o!("handle" => "create_admin"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;
        let admin = diesel::insert_into(admin_table)
            .values(new_admin)
            .get_result::<Admin>(&mut conn)?;

        Ok(admin)
    }
}

impl Handler<AuthorizeAdmin> for DbActor {
    type Result = Result<String, AppError>;

    fn handle(&mut self, msg: AuthorizeAdmin, _: &mut Self::Context) -> Self::Result {
        let admin_name = msg.basic_auth.user_id().to_string();

        let mut conn = self.0.get()?;

        let admin = admin_table
            .filter(admin_name_column.eq(admin_name))
            .get_result::<Admin>(&mut conn)?;

        authorize(
            admin.admin_id,
            admin.password,
            vec![AdminRole],
            msg.basic_auth,
        )
    }
}
