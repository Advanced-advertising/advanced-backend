use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::incomes;


#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = incomes)]
pub struct Income {
    pub income_id: Uuid,
    pub income: String,
    pub business_id: Uuid,
    pub order_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct IncomeData {
    pub price: String,
    pub user_id: Uuid,
    pub order_id: Uuid,
}
