use serde::{Deserialize, Serialize};
use sqlx::Type;
use validator::Validate;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "GENDER")]
#[serde(rename_all = "UPPERCASE")]
pub enum Gender {
    Male,
    Female,
}

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct Client {
    pub client_id: uuid::Uuid,
    pub login: String,
    pub age: i32,
    pub location: String,
    pub gender: Gender,
}
