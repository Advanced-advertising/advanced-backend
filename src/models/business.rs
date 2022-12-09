use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use uuid::Uuid;

use crate::schema::businesses;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name="businesses"]
pub struct Business {
    pub business_id: Uuid,
    pub business_name: String,
    pub email: String,
    pub password: String,
    pub working_time: String,
    pub img_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct BusinessData {
    pub business_name: String,
    pub email: String,
    pub password: String,
    pub working_time: String,
}