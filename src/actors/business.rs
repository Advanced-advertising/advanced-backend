use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::AppError;
use crate::middleware::token::authorize;
use crate::middleware::token::Role::Business as BusinessRole;
use crate::models::business::Business;
use crate::schema::businesses::dsl::{
    business_id as business_id_column, business_name as business_name_column,
    businesses as businesses_table, img_url as img_url_column,
};
use actix::{Handler, Message};
use actix_web_httpauth::extractors::basic::BasicAuth;
use argonautica::Hasher;
use diesel::prelude::*;
use serde::Deserialize;
use slog::{info, o, Logger};
use uuid::Uuid;

#[derive(Message)]
#[rtype(result = "Result<Vec<Business>, AppError>")]
pub struct GetAllBusinesses {
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Business, AppError>")]
pub struct CreateBusiness {
    pub name: String,
    pub password: String,
    pub email: String,
    pub img_url: String,
    pub phone_number: String,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<String, AppError>")]
pub struct AuthorizeBusiness {
    pub(crate) basic_auth: BasicAuth,
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
            phone_number: msg.phone_number,
            img_url: msg.img_url,
        };

        info!(
            msg.logger,
            "{}",
            format!("Business to save: {:?}", new_business.clone())
        );

        let sub_log = msg.logger.new(o!("handle" => "create_businesses"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;
        let business = diesel::insert_into(businesses_table)
            .values(new_business)
            .get_result::<Business>(&mut conn)?;

        info!(
            msg.logger,
            "{}",
            format!("Saved business: {:?}", business.clone())
        );

        Ok(business)
    }
}

impl Handler<AuthorizeBusiness> for DbActor {
    type Result = Result<String, AppError>;

    fn handle(&mut self, msg: AuthorizeBusiness, _: &mut Self::Context) -> Self::Result {
        let business_name_msg = msg.basic_auth.user_id().to_string();

        let mut conn = self.0.get()?;
        let business = businesses_table
            .filter(business_name_column.eq(business_name_msg))
            .get_result::<Business>(&mut conn)?;

        authorize(
            business.business_id,
            business.password,
            vec![BusinessRole],
            msg.basic_auth,
        )
    }
}

impl Handler<ChangeImg> for DbActor {
    type Result = Result<String, AppError>;

    fn handle(&mut self, msg: ChangeImg, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;
        diesel::update(businesses_table)
            .filter(business_id_column.eq(msg.business_id))
            .set(img_url_column.eq(msg.img_url.clone()))
            .execute(&mut conn)?;

        Ok(msg.img_url)
    }
}

impl Handler<GetAllBusinesses> for DbActor {
    type Result = Result<Vec<Business>, AppError>;

    fn handle(&mut self, msg: GetAllBusinesses, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "create_businesses"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let result = businesses_table.get_results::<Business>(&mut conn)?;

        Ok(result)
    }
}
