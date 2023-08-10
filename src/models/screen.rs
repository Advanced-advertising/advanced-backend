use diesel::{Insertable, Queryable};
use diesel::data_types::PgNumeric;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::screens;

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = screens)]
pub struct Screen {
    pub screen_id: Uuid,
    pub screen_name: String,
    pub price_per_time: PgNumeric,
    pub characteristics: String,
    pub traffic: i32,
    pub business_id: Uuid,
    pub address_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct ScreenData {
    pub screen_name: String,
    pub price_per_time: String,
    pub characteristics: String,
    pub traffic: i32,
    pub business_id: Uuid,
    pub address_id: Uuid,
}
