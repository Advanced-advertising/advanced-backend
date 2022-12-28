use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::screens;


#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = screens)]
pub struct Screen {
    pub screen_id: Uuid,
    pub screen_name: String,
    pub price_per_time: String,
    pub characteristics: String,
    pub business_id: Uuid,
    pub address_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct ScreenData {
    pub screen_name: String,
    pub price_per_time: String,
    pub characteristics: String,
    pub business_id: Uuid,
    pub address_id: Uuid,
}
