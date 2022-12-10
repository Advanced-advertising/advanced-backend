use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::categories;

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
