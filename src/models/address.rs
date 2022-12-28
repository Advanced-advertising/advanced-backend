use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::address;


#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = address)]
pub struct Address {
    pub address_id: Uuid,
    pub address_name: String,
    pub business_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct AddressData {
    pub address_name: String,
    pub business_id: Uuid,
}