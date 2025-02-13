use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    errors::ProdError,
    forms::clients::{ClientForm, ClientGenderForm},
    map_vec,
    models::clients::{ClientGenderModel, ClientModel},
};

pub trait ClientController {
    async fn bulk(
        conn: &mut PgConnection,
        clients: Vec<ClientForm>,
    ) -> Result<Vec<ClientModel>, ProdError>;

    async fn get_client_by_id(
        conn: &mut PgConnection,
        client_id: Uuid,
    ) -> Result<ClientModel, ProdError>;
}

impl ClientController for ClientModel {
    async fn bulk(
        conn: &mut PgConnection,
        clients: Vec<ClientForm>,
    ) -> Result<Vec<ClientModel>, ProdError> {
        let _ = sqlx::query!(
            r#"
            INSERT INTO clients(id, login, age, location, gender)
            SELECT * FROM UNNEST($1::UUID[], $2::VARCHAR[], $3::INT[], $4::VARCHAR[], $5::GENDER[])
            ON CONFLICT (id) DO UPDATE SET
                login = EXCLUDED.login,
                age = EXCLUDED.age,
                location = EXCLUDED.location,
                gender = EXCLUDED.gender
            "#,
            &map_vec!(clients, client_id),
            &map_vec!(clients, login),
            &map_vec!(clients, age),
            &map_vec!(clients, location),
            map_vec!(clients, gender) as Vec<ClientGenderForm>,
        )
        .fetch_all(conn)
        .await
        .map_err(ProdError::DatabaseError)?;

        let clients = clients.iter().map(|x| x.into()).collect();
        Ok(clients)
    }

    async fn get_client_by_id(
        conn: &mut PgConnection,
        client_id: Uuid,
    ) -> Result<ClientModel, ProdError> {
        let client = sqlx::query_as!(
            ClientModel,
            r#"
            SELECT id as client_id,
                   login, age, location,
                   gender as "gender: ClientGenderModel"
            FROM clients
            WHERE id = $1
            LIMIT 1
            "#,
            client_id
        )
        .fetch_one(conn)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => {
                ProdError::NotFound("No client was found with that id.".to_string())
            }
            err => ProdError::DatabaseError(err),
        })?;

        Ok(client)
    }
}
