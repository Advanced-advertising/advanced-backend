use diesel::data_types::PgNumeric;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::payments;

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = payments)]
pub struct Payment {
    pub payment_id: Uuid,
    pub price: PgNumeric,
    pub user_id: Uuid,
    pub order_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct PaymentData {
    pub price: String,
    pub user_id: Uuid,
    pub order_id: Uuid,
}
