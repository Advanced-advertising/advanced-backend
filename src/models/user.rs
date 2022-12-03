use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Clone, Queryable, Insertable, Serialize, Deserialize)]
#[table_name="users"]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub user_name: String,
    pub email: String,
    pub password: String,
}