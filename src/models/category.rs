use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::categories;
use crate::schema::business_categories;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = categories)]
pub struct Category {
    pub category_id: Uuid,
    pub category_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryData {
    pub category_name: String,
}

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = business_categories)]
pub struct BusinessCategory {
    pub category_id: Uuid,
    pub business_id: Uuid,
}