use crate::models::ad::Ad;
use crate::models::ad_order::AdOrder;
use crate::models::user::User;
use diesel::{Associations, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::incomes;

#[derive(Debug, Clone, Queryable, Insertable, Associations, Selectable)]
#[diesel(belongs_to(AdOrder))]
#[diesel(table_name = incomes)]
pub struct Income {
    pub income_id: Uuid,
    pub income: f64,
    pub business_id: Uuid,
    pub ad_order_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct IncomeAllData {
    pub price: f64,
    pub client: User,
    pub ad: Ad,
}
