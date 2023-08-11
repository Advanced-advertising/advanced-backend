use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::businesses;

#[derive(Debug, Clone, Queryable, Insertable, Serialize)]
#[diesel(table_name = businesses)]
pub struct Business {
    pub business_id: Uuid,
    pub business_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
    pub img_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct BusinessData {
    pub business_name: String,
    pub phone_number: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct BusinessAllData {
    pub business_id: Uuid,
    pub business_name: String,
    pub phone_number: String,
    pub email: String,
    pub categories: Option<Vec<Uuid>>,
}
