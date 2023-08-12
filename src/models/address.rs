use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::addresses;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = addresses)]
pub struct Address {
    pub address_id: Uuid,
    pub address_name: String,
    pub business_id: Option<Uuid>,
}

#[derive(Serialize, Deserialize)]
pub struct AddressData {
    pub address_name: String,
}
