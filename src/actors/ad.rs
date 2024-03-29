use crate::actors::db::{get_pooled_connection, DbActor};
use crate::errors::AppError;
use crate::models::ad::{Ad, AdStatus};
use crate::schema::ads::dsl::ads;
use crate::schema::ads::{ad_id, ad_name, img_url, user_id};
use actix::{Handler, Message};
use diesel::expression_methods::ExpressionMethods;
use diesel::{Connection, QueryDsl, RunQueryDsl};
use slog::{o, Logger};
use uuid::Uuid;
use crate::models::category::AdCategory;
use crate::schema::ad_categories::dsl::ad_categories;

#[derive(Message)]
#[rtype(result = "Result<Ad, AppError>")]
pub struct CreateAd {
    pub ad_name: String,
    pub img_url: String,
    pub user_id: Uuid,
    pub categories_id: Vec<Uuid>,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Ad, AppError>")]
pub struct UpdateAd {
    pub id: Uuid,
    pub ad_name: String,
    pub img_url: String,
    pub user_id: Uuid,
    pub logger: Logger,
}

#[derive(Message)]
#[rtype(result = "Result<Vec<Ad>, AppError>")]
pub struct GetAllAds;

#[derive(Message)]
#[rtype(result = "Result<Vec<Ad>, AppError>")]
pub struct GetUserAds {
    pub user_id: Uuid,
    pub logger: Logger,
}

impl Handler<CreateAd> for DbActor {
    type Result = Result<Ad, AppError>;

    fn handle(&mut self, msg: CreateAd, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "create_ad"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let new_ad = Ad {
            ad_id: Uuid::new_v4(),
            ad_name: msg.ad_name,
            img_url: msg.img_url,
            status: AdStatus::Unverified.to_string(),
            user_id: msg.user_id,
        };

        let result = conn.transaction::<_, diesel::result::Error, _>(|conn| {
            let ad = diesel::insert_into(ads)
                .values(new_ad.clone())
                .get_result::<Ad>(conn)?;

            let mut ad_cats: Vec<AdCategory> = Vec::new();
            for ad_cat_id in msg.categories_id {
                let ad_category = AdCategory {
                    ad_id: new_ad.ad_id.clone(),
                    category_id: ad_cat_id,
                };
                ad_cats.push(ad_category);
            }


            diesel::insert_into(ad_categories)
                .values(ad_cats)
                .get_result::<AdCategory>(conn)?;

            Ok(ad)
        })?;

        Ok(result)
    }
}

impl Handler<UpdateAd> for DbActor {
    type Result = Result<Ad, AppError>;

    fn handle(&mut self, msg: UpdateAd, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "update_ad"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let updated_category = diesel::update(ads)
            .filter(ad_id.eq(msg.id))
            .set((
                ad_name.eq(msg.ad_name),
                img_url.eq(msg.img_url),
                user_id.eq(msg.user_id),
            ))
            .get_result::<Ad>(&mut conn)?;

        Ok(updated_category)
    }
}

impl Handler<GetAllAds> for DbActor {
    type Result = Result<Vec<Ad>, AppError>;

    fn handle(&mut self, _: GetAllAds, _: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get()?;
        let result = ads.get_results::<Ad>(&mut conn)?;
        Ok(result)
    }
}

impl Handler<GetUserAds> for DbActor {
    type Result = Result<Vec<Ad>, AppError>;

    fn handle(&mut self, msg: GetUserAds, _: &mut Self::Context) -> Self::Result {
        let sub_log = msg.logger.new(o!("handle" => "get_user_ads"));
        let mut conn = get_pooled_connection(&self.0, sub_log.clone())?;

        let result = ads.filter(user_id.eq(msg.user_id)).get_results::<Ad>(&mut conn)?;
        Ok(result)
    }
}