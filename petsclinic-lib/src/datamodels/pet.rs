use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pet {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub customer_id: Option<ObjectId>,
    pub name: String,
    pub note: String,
    pub pet_type: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub update_time: chrono::DateTime<Utc>,
}