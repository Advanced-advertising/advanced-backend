use crate::models::ad::Ad;
use crate::models::business::Business;
use diesel::{Associations, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::{ad_categories, business_categories, categories};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = categories)]
pub struct Category {
    pub category_id: Uuid,
    pub category_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct CategoryData {
    pub category_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Associations, Queryable, Insertable)]
#[diesel(belongs_to(Business))]
#[diesel(belongs_to(Category))]
#[diesel(table_name = business_categories)]
#[diesel(primary_key(category_id, business_id))]
pub struct BusinessCategory {
    pub category_id: Uuid,
    pub business_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Associations, Queryable, Insertable)]
#[diesel(belongs_to(Ad))]
#[diesel(belongs_to(Category))]
#[diesel(table_name = ad_categories)]
#[diesel(primary_key(category_id, ad_id))]
pub struct AdCategory {
    pub category_id: Uuid,
    pub ad_id: Uuid,
}
