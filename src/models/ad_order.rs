use diesel::{Insertable, Queryable};
use diesel::data_types::{PgNumeric, PgTimestamp};
use uuid::Uuid;

use crate::schema::ad_orders;

#[derive(Debug, Clone, Queryable, Insertable)]
#[diesel(table_name = ad_orders)]
pub struct AdOrder {
    pub order_id: Uuid,
    pub start_time: PgTimestamp,
    pub end_time: PgTimestamp,
    pub price: PgNumeric,
    pub is_rejected: bool,
    pub ad_id: Uuid,
    pub screen_id: Uuid,
}