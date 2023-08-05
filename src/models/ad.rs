use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::ads;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = ads)]
pub struct Ad {
    pub ad_id: Uuid,
    pub ad_name: String,
    pub img_url: String,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct AdData {
    pub ad_name: String,
    pub img_url: String,
    pub user_id: Uuid,
}
