use crate::models::user::User;
use diesel::{Associations, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::ads;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Associations, Selectable)]
#[diesel(belongs_to(User))]
#[diesel(table_name = ads)]
pub struct Ad {
    pub ad_id: Uuid,
    pub ad_name: String,
    pub img_url: String,
    pub status: String,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct AdAllData {
    pub ad_id: Uuid,
    pub ad_name: String,
    pub img_url: String,
    pub status: String,
    pub user: User,
}

#[derive(Serialize, Deserialize)]
pub struct AdData {
    pub ad_name: String,
    pub categories_id: Vec<Uuid>,
    pub img_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct AdDataUpdate {
    pub ad_id: Uuid,
    pub ad_name: String,
    pub img_url: String,
    pub user_id: Uuid,
}

#[derive(Serialize, Deserialize)]
pub struct AdStatusUpdate {
    pub ad_id: Uuid,
    pub new_status: AdStatus,
}

#[derive(Serialize, Deserialize)]
pub enum AdStatus {
    Unverified,
    Approved,
    Rejected,
}

impl ToString for AdStatus {
    fn to_string(&self) -> String {
        match self {
            AdStatus::Unverified => "Unverified".to_string(),
            AdStatus::Approved => "Approved".to_string(),
            AdStatus::Rejected => "Rejected".to_string(),
        }
    }
}
