use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Clone, Queryable, Insertable, Serialize)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
    pub img_url: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserData {
    pub user_name: String,
    pub email: String,
    pub password: String,
    pub phone_number: String,
}
