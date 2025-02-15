use serde::{Deserialize, Serialize};
use sqlx::Type;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "GENDER", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ClientGenderModel {
    Male,
    Female,
}

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct ClientModel {
    pub client_id: uuid::Uuid,
    pub login: String,
    pub age: i32,
    pub location: String,
    pub gender: ClientGenderModel,
}
