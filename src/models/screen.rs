use crate::models::business::Business;
use diesel::{Associations, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::screens;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[derive(Associations, Queryable, Insertable, Selectable)]
#[diesel(table_name = screens)]
#[diesel(belongs_to(Business))]
pub struct Screen {
    pub screen_id: Uuid,
    pub screen_name: String,
    pub price_per_time: f64,
    pub characteristics: String,
    pub traffic: i32,
    pub business_id: Uuid,
    pub address_id: Uuid,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct ScreenData {
    pub screen_name: String,
    pub price_per_time: f64,
    pub characteristics: String,
    pub traffic: i32,
    pub business_id: Uuid,
    pub address_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct ScreenDataWithAddress {
    pub screen_id: Uuid,
    pub screen_name: String,
    pub price_per_time: f64,
    pub characteristics: String,
    pub traffic: i32,
    pub address_name: String,
    pub business_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct ScreenId {
    pub screen_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct OptimalScreensData {
    pub user_budget: f64,
    pub ad_category_ids: Vec<Uuid>,
}
