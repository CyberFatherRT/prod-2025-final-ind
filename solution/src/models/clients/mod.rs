use serde::{Deserialize, Serialize};
use sqlx::Type;
use validator::Validate;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "GENDER", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum GenderModel {
    Male,
    Female,
}

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClientModel {
    pub client_id: uuid::Uuid,
    pub login: String,
    pub age: i32,
    pub location: String,
    pub gender: GenderModel,
}
