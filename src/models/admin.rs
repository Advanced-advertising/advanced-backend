use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::admin;

#[derive(Debug, Clone, Queryable, Insertable, Serialize)]
#[diesel(table_name = admin)]
pub struct Admin {
    pub admin_id: Uuid,
    pub admin_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdminData {
    pub admin_name: String,
    pub password: String,
}
