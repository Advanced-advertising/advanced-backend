use crate::models::business::Business;
use diesel::{Associations, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::addresses;

#[derive(Debug, Clone, Serialize, Deserialize, Associations, Queryable, Insertable, Selectable)]
#[diesel(table_name = addresses)]
#[diesel(belongs_to(Business))]
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
