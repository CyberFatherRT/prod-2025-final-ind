use serde::{Deserialize, Serialize};
use sqlx::Type;
use validator::Validate;

use crate::models::clients::{ClientGenderModel, ClientModel};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "GENDER", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ClientGenderForm {
    Male,
    Female,
}

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct ClientForm {
    pub client_id: uuid::Uuid,
    pub login: String,
    pub age: i32,
    pub location: String,
    pub gender: ClientGenderForm,
}

impl From<&ClientForm> for ClientModel {
    fn from(value: &ClientForm) -> Self {
        ClientModel {
            client_id: value.client_id,
            login: value.login.clone(),
            age: value.age,
            location: value.location.clone(),
            gender: value.gender.into(),
        }
    }
}

impl From<ClientGenderForm> for ClientGenderModel {
    fn from(value: ClientGenderForm) -> Self {
        match value {
            ClientGenderForm::Male => ClientGenderModel::Male,
            ClientGenderForm::Female => ClientGenderModel::Female,
        }
    }
}
