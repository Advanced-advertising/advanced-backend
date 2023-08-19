use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::AppError;
use crate::middleware::token::authorize;
use crate::middleware::token::Role::Business as BusinessRole;
use crate::models::business::{Business, BusinessInfo};
use crate::models::category::{BusinessCategory, Category};
use crate::models::screen::Screen;
use crate::schema::business_categories::business_id;
use crate::schema::business_categories::dsl::business_categories;
use crate::schema::businesses::dsl::{
    business_id as business_id_column, business_name as business_name_column,
    businesses as businesses_table, img_url as img_url_column,
};
use crate::schema::categories::dsl::categories;
use crate::schema::categories::{category_id, category_name};
use crate::schema::screens::dsl::{business_id as screen_business_id, screens};
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
#[rtype(result = "Result<Option<BusinessInfo>, AppError>")]
pub struct GetBusinessesInfo {
    pub business_id: Uuid,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Category>, AppError>")]
pub struct GetBusinessCategories {
    pub business_id: Uuid,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<(), AppError>")]
pub struct ChangeBusinessInfo {
    pub business_info: BusinessInfo,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<(), AppError>")]
pub struct ChangeBusinessCategories {
    pub business_id: Uuid,
    pub category_ids: Vec<Uuid>,
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

impl Handler<GetBusinessCategories> for DbActor {
    type Result = Result<Vec<Category>, AppError>;

    fn handle(&mut self, msg: GetBusinessCategories, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "get_business_categories"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let result = business_categories
            .inner_join(categories)
            .filter(business_id.eq(business_id))
            .select((category_id, category_name))
            .get_results::<Category>(&mut conn)?;

        Ok(result)
    }
}

impl Handler<GetBusinessesInfo> for DbActor {
    type Result = Result<Option<BusinessInfo>, AppError>;

    fn handle(&mut self, msg: GetBusinessesInfo, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "get_business_info"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let business_data: Option<Business> = businesses_table
            .filter(business_id_column.eq(msg.business_id))
            .first::<Business>(&mut conn)
            .optional()?;

        let result = match business_data {
            None => None,
            Some(business_data) => Some(BusinessInfo {
                business_id: business_data.business_id,
                business_name: business_data.business_name,
                phone_number: business_data.phone_number,
                email: business_data.email,
                categories: vec![],
                screens: vec![],
            }),
        };

        if let Some(mut business_info) = result {
            let categories_for_business: Vec<Category> = business_categories
                .inner_join(categories)
                .filter(business_id.eq(msg.business_id))
                .select((category_id, category_name))
                .load(&mut conn)?;

            let screens_for_business: Vec<Screen> = screens
                .filter(screen_business_id.eq(msg.business_id))
                .load(&mut conn)?;

            business_info.categories = categories_for_business;
            business_info.screens = screens_for_business;
            return Ok(Some(business_info));
        };

        Ok(result.clone())
    }
}

impl Handler<ChangeBusinessInfo> for DbActor {
    type Result = Result<(), AppError>;

    fn handle(&mut self, msg: ChangeBusinessInfo, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "update_business_info"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let business_data: Option<Business> = businesses_table
            .filter(business_id_column.eq(msg.business_info.business_id))
            .first::<Business>(&mut conn)
            .optional()?;

        if let Some(mut business_data) = business_data {
            business_data.business_name = msg.business_info.business_name.clone();
            business_data.phone_number = msg.business_info.phone_number.clone();
            business_data.email = msg.business_info.email.clone();

            diesel::update(businesses_table)
                .filter(business_id_column.eq(msg.business_info.business_id))
                .set(&business_data)
                .execute(&mut conn)?;

            conn.transaction::<_, diesel::result::Error, _>(|conn| {
                diesel::delete(
                    business_categories.filter(business_id.eq(msg.business_info.business_id)),
                )
                .execute(conn)?;

                let new_business_categories: Vec<BusinessCategory> = msg
                    .business_info
                    .categories
                    .iter()
                    .map(|category| BusinessCategory {
                        business_id: msg.business_info.business_id,
                        category_id: category.category_id,
                    })
                    .collect();

                diesel::insert_into(business_categories)
                    .values(&new_business_categories)
                    .execute(conn)?;

                Ok(())
            })?;
        }
        Ok(())
    }
}
