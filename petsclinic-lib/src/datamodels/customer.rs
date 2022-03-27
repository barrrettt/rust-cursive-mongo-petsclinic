use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::Utc;

#[derive(Serialize, Deserialize, Debug)]
pub struct Customer {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub note: String,
    pub contact: Vec<ContactType>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub update_time: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ContactType {
    Email(String),
    Telephone(String),
    Address(Address),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Address{
    pub line1:String,
    pub line2:String,
    pub line3:String,
}
