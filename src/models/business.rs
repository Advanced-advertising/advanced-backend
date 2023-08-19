use crate::models::category::Category;
use crate::models::screen::Screen;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::businesses;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Selectable, AsChangeset)]
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

#[derive(Clone, Serialize, Deserialize)]
pub struct BusinessInfo {
    pub business_id: Uuid,
    pub business_name: String,
    pub phone_number: String,
    pub email: String,
    pub categories: Vec<Category>,
    pub screens: Vec<Screen>,
}
