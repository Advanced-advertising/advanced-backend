use crate::models::ad::Ad;
use crate::models::screen::Screen;
use crate::models::user::User;
use diesel::data_types::PgTimestamp;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::ad_orders;

#[derive(Debug, Clone, Queryable, Insertable, Selectable)]
#[diesel(table_name = ad_orders)]
pub struct AdOrder {
    pub ad_order_id: Uuid,
    pub start_time: PgTimestamp,
    pub end_time: PgTimestamp,
    pub price: f64,
    pub is_rejected: bool,
    pub ad_id: Uuid,
    pub screen_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct AdOrderData {
    pub start_time: i64,
    pub end_time: i64,
    pub price: f64,
    pub ad_id: Uuid,
    pub screen_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct AdOrderId {
    pub order_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct AdOrderAllData {
    pub order_id: Uuid,
    pub start_time: i64,
    pub end_time: i64,
    pub price: f64,
    pub is_rejected: bool,
    pub address_name: String,
    pub ad: Ad,
    pub client: User,
    pub screen: Screen,
}
