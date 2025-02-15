use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::Type;
use validator::Validate;

use crate::models::clients::{ClientGenderModel, ClientModel};

static RE_CLIENT_LOGIN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]*$").expect("Invalid regex"));

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "GENDER", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum ClientGenderForm {
    Male,
    Female,
}

#[derive(Debug, Validate, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct ClientForm {
    pub client_id: uuid::Uuid,
    #[validate(length(min = 3, max = 150, message = "Login is too short or too long"))]
    #[validate(regex(path = *RE_CLIENT_LOGIN, message = "Login contains invalid characters"))]
    pub login: String,
    #[validate(range(min = 0, max = 120, message = "Age is out of range"))]
    pub age: i32,
    #[validate(length(min = 3, max = 150, message = "Location is too short or too long"))]
    pub location: String,
    pub gender: ClientGenderForm,
}

impl From<&ClientForm> for ClientModel {
    fn from(value: &ClientForm) -> Self {
        Self {
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
            ClientGenderForm::Male => Self::Male,
            ClientGenderForm::Female => Self::Female,
        }
    }
}
